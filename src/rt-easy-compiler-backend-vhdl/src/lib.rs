#![deny(rust_2018_idioms)]

mod generate;
mod impl_render;
mod signals;

pub mod vhdl;

pub use self::{signals::Signals, vhdl::Vhdl};

#[derive(Debug)]
pub struct BackendVhdl;

#[derive(Debug)]
pub struct Args {
    pub module_name: String,
}

impl<'s> compiler::Backend<'s> for BackendVhdl {
    type Args = Args;
    type Output = vhdl::Vhdl<'s>;
    type Error = std::convert::Infallible;

    fn generate(
        &self,
        mir: compiler::mir::Mir<'s>,
        args: Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        Ok(generate::generate(mir, args.module_name))
    }
}
