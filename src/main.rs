use repl_rs::{FromValue, IntoDynFn, Rustua};

fn main() {
    let mut rustua = Rustua::new()
        .register_function("add", (|(a, b): (i32, i32)| a + b).into_dyn_fn())
        .register_function("sub", (|(a, b): (i32, i32)| a - b).into_dyn_fn());

    let v = rustua.eval("add(10, 20) + sub(30, 5)").unwrap();
    println!("Result: {:?}", v);

    let mut rustua = Rustua::new()
        .register_function(
            "repeat",
            (|(a, b): (String, usize)| a.repeat(b).to_string()).into_dyn_fn(),
        )
        .register_function("add", (|(a, b): (i32, i32)| a + b).into_dyn_fn());

    let v = rustua.eval(r#"repeat("some string\n", 1+2)"#).unwrap();
    let s: String = FromValue::from_value(v.clone()).unwrap();
    println!("Result: {s}");
}
