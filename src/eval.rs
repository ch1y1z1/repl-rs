use eros::{Context, Result};
use std::collections::HashMap;

use crate::{
    ast::{Ast, parse_input},
    value::{DynFunction, Value},
};

pub struct Rustua {
    global_functions: HashMap<String, Box<dyn DynFunction>>,
}

impl Rustua {
    pub fn new() -> Self {
        Self {
            global_functions: HashMap::new(),
        }
    }

    pub fn register_function(mut self, name: &str, func: Box<dyn DynFunction>) -> Self {
        self.global_functions.insert(name.to_string(), func);
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
