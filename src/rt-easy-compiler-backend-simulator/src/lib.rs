#![deny(rust_2018_idioms)]

mod concat;
mod declaration;
mod expression;
mod helper;
mod operation;
mod program;

#[derive(Debug)]
pub struct BackendSimulator;

impl compiler::Backend for BackendSimulator {
    type Args = ();
    type Output = rtcore::program::Program;
    type Error = std::convert::Infallible;

    fn generate(&self, mir: compiler::mir::Mir<'_>, _args: Self::Args) -> Result<Self::Output> {
        rtcore::program::Program::generate(mir)
    }
}

type Result<T> = std::result::Result<T, std::convert::Infallible>;

trait Generate<I>: Sized {
    fn generate(input: I) -> Result<Self>;
}
