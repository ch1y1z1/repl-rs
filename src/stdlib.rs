use crate::Rustua;

pub trait Stdlib {
    fn prepare_stdlib(self) -> Self;
}

impl Stdlib for Rustua {
    fn prepare_stdlib(self) -> Self {
        self
    }
}
