use repl_rs::Runtime;

fn main() {
    let mut rt = Runtime::new().register_function("repeat", |(s, n): (String, usize)| s.repeat(n));

    let v = rt.eval("repeat(\"Hello\", 3)").unwrap();
    println!("Result: {:?}", v);
}
