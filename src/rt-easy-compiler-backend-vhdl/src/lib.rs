#![deny(rust_2018_idioms)]

mod concat;
mod declarations;
mod expression;
mod next_state_logic_deps;
mod operation;
mod statement;
mod vhdl;

pub mod error;

#[derive(Debug)]
pub struct BackendVhdl;

impl compiler::Backend for BackendVhdl {
    type Args = ();
    type Output = rtvhdl::Vhdl;
    type Error = error::SynthError;

    fn generate(
        &self,
        mir: compiler::mir::Mir<'_>,
        (): Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        self::vhdl::VhdlBuilder::build(mir)
    }
}

fn gen_ident(id: compiler::mir::Ident<'_>) -> rtvhdl::Ident {
    rtvhdl::Ident(id.0.to_owned())
}
