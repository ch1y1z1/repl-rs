use eros::{Context, Result};
use std::collections::HashMap;

use crate::{
    IntoDynFn,
    ast::{Ast, parse_input},
    stdlib::Stdlib,
    value::{DynFunction, FromValueMulti, IntoValue, Value},
};

pub struct Rustua {
    global_functions: HashMap<String, Box<dyn DynFunction>>,
}

impl Rustua {
    pub fn new() -> Self {
        Self {
            global_functions: HashMap::new(),
        }
        .prepare_stdlib()
    }

    pub fn register_function<F, A, R>(mut self, name: &str, func: F) -> Self
    where
        F: IntoDynFn + 'static + FnMut(A) -> R,
        A: FromValueMulti + 'static,
        R: IntoValue + 'static,
    {
        self.global_functions
            .insert(name.to_string(), func.into_dyn_fn());
        self
    }

    pub fn register_function_raw<F>(mut self, name: &str, func: F) -> Self
    where
        F: FnMut(Vec<Value>) -> Value + 'static,
    {
        self.global_functions
            .insert(name.to_string(), Box::new(func));
        self
    }

    fn eval_ast(&mut self, ast: Ast) -> Result<Value> {
        match ast {
            Ast::Constant(v) => Ok(v),
            Ast::Call(name, args) => {
                let arg_values: Result<Vec<Value>> =
                    args.into_iter().map(|arg| self.eval_ast(arg)).collect();
                let arg_values =
                    arg_values.with_context(|| format!("error evaling {name}'s args"))?;
                let func = self
                    .global_functions
                    .get_mut(&name)
                    .ok_or_else(|| format!("Function '{}' not found", name))?;
                func.call_with_vec_value(arg_values)
                    .with_context(|| format!("error calling function '{name}'"))
            }
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<Value> {
        let ast = parse_input(input)?;
        self.eval_ast(ast)
    }
}
