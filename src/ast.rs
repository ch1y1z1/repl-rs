use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Call(String, Vec<Ast>),
    Constant(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(BigInt),
    Float(f64),
    String(String),
}
