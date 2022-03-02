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
mod vhdl;

/*
TODO: canonicalize common::BitRange to BitRange see below
    - in declaration:
        - None => BitRange::Downto(0, 0)
        - (x) => (x:x)  [Downto/To]
        - _ -> _        [Downto/To]
    - in expr:
        - None => BitRange::Full
        - full range of declaration => BitRange::Full
        - (x) => (x:x)  [Downto/To]
        - _ -> _        [Downto/To]

    enum BitRange {
        Full, // Render: ""
        Downto(usize, usize), // Render: "({} DOWNTO {})"
        To(usize, usize), // Render: "({} TO {})"
    }

TODO: REMOVE (PartialEq, Eq, Hash) from common::BitRange
*/

pub fn generate<'s>(mir: compiler::mir::Mir<'s>, module_name: String) -> crate::vhdl::Vhdl<'s> {
    self::vhdl::generate_vhdl(mir, module_name)
}
