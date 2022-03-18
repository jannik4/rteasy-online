#![deny(rust_2018_idioms)]

mod generate;
mod impl_render;
mod render_as_rt;
mod render_as_vhdl;
mod signals;

pub mod error;
pub mod vhdl;

pub use self::{signals::Signals, vhdl::Vhdl};

#[derive(Debug)]
pub struct BackendVhdl;

impl compiler::Backend for BackendVhdl {
    type Args = ();
    type Output = vhdl::Vhdl;
    type Error = error::SynthError;

    fn generate(
        &self,
        mir: compiler::mir::Mir<'_>,
        (): Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        generate::generate(mir)
    }
}
