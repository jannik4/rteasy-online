//! Generate VHDL from MIR.
//!
//! The criteria of the individual states are combined in a global set.
//!
//! The same happens with all operations. Nop and assert are discarded.
//!
//! Goto operations go into next_state_logic.

mod concat;
mod declarations;
mod expression;
mod operation;
mod statement;
mod vhdl;

pub fn generate<'s>(mir: compiler::mir::Mir<'s>) -> crate::vhdl::Vhdl {
    self::vhdl::generate_vhdl(mir)
}
