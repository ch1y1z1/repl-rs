use eros::{Context, Result};
use std::marker::PhantomData;

use super::value::{FromValueMulti, IntoValue};
use crate::ast::Value;

pub trait DynFunction {
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value>;
}

struct FnAdapter<F, A, R> {
    f: F,
    _marker: PhantomData<(A, R)>,
}

impl<F, A, R> FnAdapter<F, A, R> {
    fn new(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }
}

pub trait IntoDynFn: Sized {
    fn into_dyn_fn<A, R>(self) -> Box<dyn DynFunction>
    where
        Self: 'static + FnMut(A) -> R,
        A: FromValueMulti + 'static,
        R: IntoValue + 'static;
}

impl<F, A, R> DynFunction for FnAdapter<F, A, R>
where
    F: FnMut(A) -> R,
    A: FromValueMulti,
    R: IntoValue,
{
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value> {
        let a = A::from_value_multi(args).context("args into tuple error")?;
        let r = (self.f)(a);
        Ok(r.into_value())
    }
}

impl<F> DynFunction for F
where
    F: FnMut(Vec<Value>) -> Value,
{
    fn call_with_vec_value(&mut self, args: Vec<Value>) -> Result<Value> {
        Ok((self)(args))
    }
}

impl<F> IntoDynFn for F {
    fn into_dyn_fn<A, R>(self) -> Box<dyn DynFunction>
    where
        F: 'static + FnMut(A) -> R,
        A: FromValueMulti + 'static,
        R: IntoValue + 'static,
    {
        Box::new(FnAdapter::<F, A, R>::new(self))
    }
}
