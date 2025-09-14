use eros::{Context, IntoDynTracedError, Result, bail};
use num_bigint::BigInt;

use crate::ast::Value;

pub trait IntoValue {
    fn into_value(self) -> Value;
}

pub trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self>;
}

// pub trait IntoValueMulti {
//     fn into_value_multi(self) -> Vec<Value>;
// }

pub trait FromValueMulti: Sized {
    fn from_value_multi(v: Vec<Value>) -> Result<Self>;
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

// macro_rules! impl_into_value_multi_tuple {
//     ( $( $($name:ident),+ );+ $(;)? ) => {
//         $(
//             #[allow(non_snake_case)]
//             impl<$($name),+> IntoValueMulti for ($($name,)+)
//             where
//                 $( $name: IntoValue ),+
//             {
//                 fn into_value_multi(self) -> Vec<Value> {
//                     let ($($name,)+) = self;
//                     let mut v = Vec::new();
//                     $( v.push($name.into_value()); )+
//                     v
//                 }
//             }
//         )+
//     };
// }

// impl_into_value_multi_tuple! {
//     A;
//     A, B;
//     A, B, C;
//     A, B, C, D;
// }

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

pub trait NumInt {}

impl NumInt for i8 {}
impl NumInt for i16 {}
impl NumInt for i32 {}
impl NumInt for i64 {}
impl NumInt for i128 {}
impl NumInt for isize {}
impl NumInt for u8 {}
impl NumInt for u16 {}
impl NumInt for u32 {}
impl NumInt for u64 {}
impl NumInt for u128 {}
impl NumInt for usize {}
