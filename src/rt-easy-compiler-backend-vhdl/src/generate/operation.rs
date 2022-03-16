use super::{
    concat::{generate_concat_lvalue_clocked, generate_concat_lvalue_unclocked},
    expression::{generate_bus, generate_expression, generate_register, generate_register_array},
};
use crate::vhdl::*;
use compiler::mir;

pub fn generate_read<'s>(read: &mir::Read<'s>, declarations: &Declarations) -> Read {
    // TODO: internal error instead of unwrap?
    let (_, (ar_name, _, ar_kind), (dr_name, _, dr_kind)) =
        declarations.memories.iter().find(|(name, _, _)| *name == read.ident.node.into()).unwrap();

    Read {
        memory: read.ident.node.into(),
        ar: Register { ident: ar_name.clone(), range: None, kind: *ar_kind },
        dr: Register { ident: dr_name.clone(), range: None, kind: *dr_kind },
    }
}

pub fn generate_write<'s>(write: &mir::Write<'s>, declarations: &Declarations) -> Write {
    // TODO: internal error instead of unwrap?
    let (_, (ar_name, _, ar_kind), (dr_name, _, dr_kind)) =
        declarations.memories.iter().find(|(name, _, _)| *name == write.ident.node.into()).unwrap();

    Write {
        memory: write.ident.node.into(),
        ar: Register { ident: ar_name.clone(), range: None, kind: *ar_kind },
        dr: Register { ident: dr_name.clone(), range: None, kind: *dr_kind },
    }
}

pub fn generate_assignment<'s>(
    assignment: &mir::Assignment<'s>,
    declarations: &Declarations,
) -> Assignment {
    Assignment {
        lhs: generate_lvalue(&assignment.lhs, declarations),
        rhs: generate_expression(&assignment.rhs, declarations, assignment.size),
    }
}

fn generate_lvalue<'s>(lvalue: &mir::Lvalue<'s>, declarations: &Declarations) -> Lvalue {
    match lvalue {
        mir::Lvalue::Register(reg) => Lvalue::Register(generate_register(reg, declarations)),
        mir::Lvalue::Bus(bus) => Lvalue::Bus(generate_bus(bus, declarations)),
        mir::Lvalue::RegisterArray(reg_array) => {
            Lvalue::RegisterArray(generate_register_array(reg_array, declarations))
        }
        mir::Lvalue::ConcatClocked(concat) => {
            Lvalue::ConcatClocked(generate_concat_lvalue_clocked(concat, declarations))
        }
        mir::Lvalue::ConcatUnclocked(concat) => {
            Lvalue::ConcatUnclocked(generate_concat_lvalue_unclocked(concat, declarations))
        }
    }
}
