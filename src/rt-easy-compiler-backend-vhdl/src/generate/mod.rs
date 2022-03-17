mod concat;
mod declarations;
mod expression;
mod operation;
mod statement;
mod vhdl;

pub fn generate<'s>(mir: compiler::mir::Mir<'s>) -> crate::vhdl::Vhdl {
    self::vhdl::generate_vhdl(mir)
}
