use repl_rs::Runtime;

fn main() {
    let mut rt = Runtime::new().register_function("repeat", |(s, n): (String, usize)| s.repeat(n));

    let v = rt.eval("repeat(\"Hello\", 1 + int(2.5))").unwrap();
    println!("Result: {:?}", v);
}
