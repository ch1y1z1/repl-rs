mod ast;
mod token;
mod value;

use std::{any::Any, str::FromStr};

use num_bigint::BigInt;

type DynArgs = Vec<Box<dyn Any>>;
type DynRet = Box<dyn Any>;

// #[repl]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn add_dyn(args: DynArgs) -> DynRet {
    assert!(args.len() == 2);
    let x = *args[0].downcast_ref::<i32>().unwrap();
    let y = *args[1].downcast_ref::<i32>().unwrap();
    Box::new(add(x, y))
}

fn main() {
    let ret = *add_dyn(vec![Box::new(1i32), Box::new(2i32)])
        .downcast_ref::<i32>()
        .unwrap();
    println!("1 + 2 = {}", ret);
    let ret = *add_dyn(vec![Box::new(1i32), Box::new(2i32), Box::new(2i32)])
        .downcast_ref::<i32>()
        .unwrap();
    println!("1 + 2 = {}", ret);
    let ret = *add_dyn(vec![Box::new("str"), Box::new(2i32)])
        .downcast_ref::<i32>()
        .unwrap();
    println!("1 + 2 = {}", ret);
    println!("Hello, world!");

    let num = BigInt::from_str("1256").unwrap();
    let num: u8 = num.try_into().unwrap();
}
