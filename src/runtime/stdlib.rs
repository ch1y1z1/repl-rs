use eros::{Result, bail};
use num_bigint::ToBigInt;
use num_traits::cast::ToPrimitive;

use super::runtime::Runtime;
use crate::ast::Value;

pub trait Stdlib {
    fn prepare_stdlib(self) -> Self;
}

impl Stdlib for Runtime {
    fn prepare_stdlib(self) -> Self {
        self.register_function_raw("add", add)
            .register_function_raw("sub", sub)
            .register_function_raw("mul", mul)
            .register_function_raw("div", div)
            .register_function_raw("float", float)
            .register_function_raw("int", int)
    }
}

macro_rules! generate_basic_op {
    ($op:ident, $fn:tt) => {
        fn $op(args: Vec<Value>) -> Result<Value> {
            if args.len() != 2 {
                bail!("{} function takes 2 arguments", stringify!($op));
            }

            let (a, b) = (&args[0], &args[1]);

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a $fn b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a $fn b)),
                _ => bail!("{} function takes 2 arguments of the same type", stringify!($op)),
            }
        }
    }
}

generate_basic_op!(add, +);
generate_basic_op!(sub, -);
generate_basic_op!(mul, *);
generate_basic_op!(div, /);

fn float(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        bail!("float function takes 1 argument");
    }

    let a = &args[0];

    match a {
        Value::Int(a) => Ok(Value::Float(a.to_f64().unwrap())),
        Value::Float(a) => Ok(Value::Float(*a)),
        Value::String(a) => Ok(Value::Float(a.parse().unwrap())),
    }
}

fn int(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        bail!("int function takes 1 argument");
    }

    let a = &args[0];

    match a {
        Value::Int(a) => Ok(Value::Int(a.clone())),
        Value::Float(a) => Ok(Value::Int(a.to_bigint().unwrap())),
        Value::String(a) => Ok(Value::Int(a.parse().unwrap())),
    }
}
