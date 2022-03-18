mod concat;
mod declarations;
mod expression;
mod operation;
mod statement;
mod transform;
mod vhdl;

pub fn generate(
    mir: compiler::mir::Mir<'_>,
) -> Result<crate::vhdl::Vhdl, crate::error::SynthError> {
    self::vhdl::VhdlBuilder::build(mir)
}
