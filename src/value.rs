use eros::{
    Context, IntoConcreteTracedError, IntoDynTracedError, IntoUnionResult, Result, TracedError,
    bail,
};
use num_bigint::BigInt;

enum Value {
    Int(BigInt),
    Float(f64),
    String(String),
}

trait IntoValue {
    fn into_value(self) -> Value;
}

trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self>;
}

trait IntoValueMulti {
    fn into_value_multi(self) -> Vec<Value>;
}

trait FromValueMulti: Sized {
    fn from_value_multi(v: Vec<Value>) -> Result<Self>;
}

trait IntoFunction<A, R> {
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value>;
}

#[test]
fn test() {
    use std::str::FromStr;

    let num = BigInt::from_str("1256").unwrap();
    let num: i32 = num.try_into().unwrap();

    let value_multi = (1, 128, 12).into_value_multi();
    println!("{}", num);
}

impl<T> IntoValue for T
where
    T: Into<BigInt>,
{
    fn into_value(self) -> Value {
        Value::Int(self.into())
    }
}

impl<T> FromValue for T
where
    T: TryFrom<BigInt>,
    <T as TryFrom<BigInt>>::Error: std::error::Error + Send + Sync + 'static,
{
    fn from_value(v: Value) -> Result<Self> {
        match v {
            Value::Int(i) => i
                .clone()
                .try_into()
                .traced_dyn()
                .with_context(|| format!("BigInt to target type conversion error: {i}")),
            _ => panic!("Unsupported type"),
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

impl<F, A, R> IntoFunction<A, R> for F
where
    F: FnMut(A) -> R,
    A: FromValueMulti,
    R: IntoValue,
{
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value> {
        let args = A::from_value_multi(args).context("args into tuple error")?;
        let ret = self(args);
        Ok(ret.into_value())
    }
}

#[test]
fn test_fn_trait() {
    let con = 2;
    let sub = move |(x, y): (i32, i32)| x - y - con;
    let args = (10, 4).into_value_multi();
    let mut f = sub;
    let ret = f.call_with_vec_value(args).unwrap();
    let ret: i32 = FromValue::from_value(ret).unwrap();
    assert_eq!(ret, 4);
}
