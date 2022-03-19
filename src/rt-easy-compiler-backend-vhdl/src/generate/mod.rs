mod concat;
mod declarations;
mod expression;
mod next_state_logic_deps;
mod operation;
mod statement;
mod vhdl;

pub fn generate(
    mir: compiler::mir::Mir<'_>,
) -> Result<crate::vhdl::Vhdl, crate::error::SynthError> {
    self::vhdl::VhdlBuilder::build(mir)
}
