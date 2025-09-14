use eros::{Context, Result};

use super::runtime::Runtime;
use crate::ast::Ast;
use crate::ast::Value;
use crate::parser::parse;

fn eval_ast(rt: &mut Runtime, ast: Ast) -> Result<Value> {
    match ast {
        Ast::Constant(v) => Ok(v),
        Ast::Call(name, args) => {
            let arg_values: Result<Vec<Value>> =
                args.into_iter().map(|arg| eval_ast(rt, arg)).collect();
            let arg_values = arg_values.with_context(|| format!("error evaling {name}'s args"))?;
            let func = rt
                .global_functions
                .get_mut(&name)
                .ok_or_else(|| format!("Function '{}' not found", name))?;
            func.call_with_vec_value(arg_values)
                .with_context(|| format!("error calling function '{name}'"))
        }
    }
}

pub fn eval(rt: &mut Runtime, input: &str) -> Result<Value> {
    let ast = parse(input)?;
    eval_ast(rt, ast)
}
