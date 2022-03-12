use super::{
    concat::{generate_concat_lvalue_clocked, generate_concat_lvalue_unclocked},
    expression::{generate_bus, generate_expression, generate_register, generate_register_array},
};
use crate::vhdl::*;
use compiler::mir;

pub fn generate_assignment<'s>(
    assignment: &mir::Assignment<'s>,
    declarations: &Declarations<'s>,
) -> Assignment<'s> {
    Assignment {
        lhs: generate_lvalue(&assignment.lhs, declarations),
        rhs: generate_expression(&assignment.rhs, declarations, assignment.size),
    }
}

fn generate_lvalue<'s>(lvalue: &mir::Lvalue<'s>, declarations: &Declarations<'s>) -> Lvalue<'s> {
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
