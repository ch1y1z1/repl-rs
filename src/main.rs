use repl_rs::{FromValue, Rustua};

fn main() {
    let mut rustua = Rustua::new()
        .register_function("add", |(a, b): (i32, i32)| a + b)
        .register_function("sub", |(a, b): (i32, i32)| a - b);

    let v = rustua.eval("add(10, 20) + sub(30, 5)").unwrap();
    println!("Result: {:?}", v);

    let mut rustua = Rustua::new()
        .register_function("repeat", |(a, b): (String, usize)| a.repeat(b).to_string())
        .register_function("add", |(a, b): (i32, i32)| a + b);

    let v = rustua.eval(r#"repeat("some string\n", 1+2)"#).unwrap();
    let s: String = FromValue::from_value(v.clone()).unwrap();
    println!("Result: {s}");

    let _ = rustua.eval(r#"some err string"#).unwrap();
}
