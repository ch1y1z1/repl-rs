use eros::Result;
use num_bigint::BigInt;

enum Value {
    Int(BigInt),
    Float(f64),
    String(String),
}

enum ValueType {
    Int,
    Float,
    String,
}

struct FunctionSignature {
    name: String,
    arg_types: Vec<ValueType>,
    ret_type: ValueType,
}

#[test]
fn test() {
    use std::str::FromStr;

    let num = BigInt::from_str("1256").unwrap();
    let num: i32 = num.try_into().unwrap();
    println!("{}", num);
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn sub_dyn(args: Vec<Value>) -> Result<Value> {
    let arg0 = args.get(0).ok_or("missing argument 0")?;
    let arg0: i32 = match arg0 {
        Value::Int(n) => n.try_into().map_err(|_| "argument 0 out of range")?,
        _ => return Err("argument 0 type mismatch".into()),
    };
    let arg1 = args.get(1).ok_or("missing argument 1")?;
    let arg1: i32 = match arg1 {
        Value::Int(n) => n.try_into().map_err(|_| "argument 1 out of range")?,
        _ => return Err("argument 1 type mismatch".into()),
    };

    let ret = sub(arg0, arg1);
    Ok(Value::Int(ret.into()))
}
