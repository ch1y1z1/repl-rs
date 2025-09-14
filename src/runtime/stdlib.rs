use super::runtime::Runtime;

pub trait Stdlib {
    fn prepare_stdlib(self) -> Self;
}

impl Stdlib for Runtime {
    fn prepare_stdlib(self) -> Self {
        self
    }
}

// todo: add stdlib functions
