use std::marker::PhantomData;

use eros::{Context, IntoDynTracedError, Result, bail};
use num_bigint::BigInt;

use crate::number::NumInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(BigInt),
    Float(f64),
    String(String),
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

pub trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self>;
}

pub trait IntoValueMulti {
    fn into_value_multi(self) -> Vec<Value>;
}

pub trait FromValueMulti: Sized {
    fn from_value_multi(v: Vec<Value>) -> Result<Self>;
}

pub trait DynFunction {
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value>;
}

struct FnAdapter<F, A, R> {
    f: F,
    _marker: PhantomData<(A, R)>,
}

impl<F, A, R> FnAdapter<F, A, R> {
    fn new(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }
}

pub trait IntoDynFn: Sized {
    fn into_dyn_fn<A, R>(self) -> Box<dyn DynFunction>
    where
        Self: 'static + FnMut(A) -> R,
        A: FromValueMulti + 'static,
        R: IntoValue + 'static;
}

impl<T> IntoValue for T
where
    T: Into<BigInt> + NumInt,
{
    fn into_value(self) -> Value {
        Value::Int(self.into())
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl<T> FromValue for T
where
    T: TryFrom<BigInt> + NumInt,
    <T as TryFrom<BigInt>>::Error: std::error::Error + Send + Sync + 'static,
{
    fn from_value(v: Value) -> Result<Self> {
        match v {
            Value::Int(i) => i
                .clone()
                .try_into()
                .traced_dyn()
                .with_context(|| format!("BigInt to target type conversion error: {i}")),
            _ => bail!("Unsupported type"),
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> Result<Self> {
        match v {
            Value::String(s) => Ok(s),
            _ => bail!("Unsupported type"),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: Value) -> Result<Self> {
        match v {
            Value::Float(f) => Ok(f),
            _ => bail!("Unsupported type"),
        }
    }
}

macro_rules! impl_into_value_multi_tuple {
    ( $( $($name:ident),+ );+ $(;)? ) => {
        $(
            impl<$($name),+> IntoValueMulti for ($($name,)+)
            where
                $( $name: IntoValue ),+
            {
                fn into_value_multi(self) -> Vec<Value> {
                    let ($($name,)+) = self;
                    let mut v = Vec::new();
                    $( v.push($name.into_value()); )+
                    v
                }
            }
        )+
    };
}

impl_into_value_multi_tuple! {
    A;
    A, B;
    A, B, C;
    A, B, C, D;
}

macro_rules! count {
    ($($xs:ident),* $(,)?) => {
        <[()]>::len(&[ $( { let _ = stringify!($xs); () } ),* ])
    };
}

macro_rules! impl_from_value_multi_tuple {
    ( $( $($name:ident),+ );+ $(;)? ) => {
        $(
            impl<$($name),+> FromValueMulti for ($($name,)+)
            where
                $( $name: FromValue ),+
            {
                fn from_value_multi(v: Vec<Value>) -> Result<Self> {
                    const LEN: usize = count!($($name),+);
                    if v.len() != LEN {
                        bail!("Argument length mismatch");
                    }
                    let mut it = v.into_iter();
                    Ok((
                        $(
                            <$name as FromValue>::from_value(it.next().unwrap())?,
                        )+
                    ))
                }
            }
        )+
    };
}

impl_from_value_multi_tuple! {
    A;
    A, B;
    A, B, C;
    A, B, C, D;
}

// 真正的实现：现在 A/R 出现在 Self 上了 -> 不会再 E0207
impl<F, A, R> DynFunction for FnAdapter<F, A, R>
where
    F: FnMut(A) -> R,
    A: FromValueMulti,
    R: IntoValue,
{
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value> {
        let a = A::from_value_multi(args).context("args into tuple error")?;
        let r = (self.f)(a);
        Ok(r.into_value())
    }
}

impl<F> DynFunction for F
where
    F: FnMut(Vec<Value>) -> Value,
{
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value> {
        Ok((self)(args))
    }
}

impl<F> IntoDynFn for F {
    fn into_dyn_fn<A, R>(self) -> Box<dyn DynFunction>
    where
        F: 'static + FnMut(A) -> R,
        A: FromValueMulti + 'static,
        R: IntoValue + 'static,
    {
        Box::new(FnAdapter::<F, A, R>::new(self))
    }
}

#[test]
fn test_fn_trait() {
    let con = 2;
    let sub = move |(x, y): (i32, i32)| x - y - con;
    let args = (10, 4).into_value_multi();
    let mut f = sub.into_dyn_fn();
    let ret = f.call_with_vec_value(args).unwrap();
    let ret: i32 = FromValue::from_value(ret).unwrap();
    assert_eq!(ret, 4);
}

#[test]
fn test_fn_trait_multi_type() {
    let repeat = |(s, n): (String, usize)| s.repeat(n);
    let args = ("ab".to_string(), 3).into_value_multi();
    let mut f = repeat.into_dyn_fn();
    let ret = f.call_with_vec_value(args).unwrap();
    let ret: String = FromValue::from_value(ret).unwrap();
    assert_eq!(ret, "ababab".to_string());
}

#[test]
fn test_fn_trait_float() {
    let div = |(x, y): (f64, f64)| x / y;
    let args = (5.0f64, 2.0f64).into_value_multi();
    let mut f = div.into_dyn_fn();
    let ret = f.call_with_vec_value(args).unwrap();
    let ret: f64 = FromValue::from_value(ret).unwrap();
    assert_eq!(ret, 2.5);
}
