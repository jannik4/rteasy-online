mod concat;
mod declarations;
mod expression;
mod operation;
mod statement;
mod transform;
mod vhdl;

pub fn generate(mir: compiler::mir::Mir<'_>) -> crate::vhdl::Vhdl {
    self::vhdl::VhdlBuilder::build(mir)
}
