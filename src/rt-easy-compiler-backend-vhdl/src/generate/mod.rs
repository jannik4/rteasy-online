//! TODO: ar, dr in read/write ops rein von declarations
//! TODO: pipe operator
//!
//! Generate VHDL from MIR.
//!
//! The criteria of the individual states are combined in a global set.
//!
//! The same happens with all operations. Nop and assert are discarded.
//!
//! Goto operations go into next_state_conditional/next_state_default.

// TODO: Fix text above

mod concat;
mod declarations;
mod expression;
mod operation;
mod statement;
mod vhdl;

pub fn generate<'s>(mir: compiler::mir::Mir<'s>, module_name: String) -> crate::vhdl::Vhdl<'s> {
    self::vhdl::generate_vhdl(mir, module_name)
}
