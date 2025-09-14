use std::collections::HashMap;

use super::dyn_function::{DynFunction, IntoDynFn};
use super::eval::eval;
use super::stdlib::Stdlib;
use super::value::{FromValueMulti, IntoValue};
use crate::ast::Value;
use eros::Result;

pub struct Runtime {
    pub(crate) global_functions: HashMap<String, Box<dyn DynFunction>>,
}

impl Runtime {
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

    pub fn eval(&mut self, input: &str) -> Result<Value> {
        eval(self, input)
    }
}
