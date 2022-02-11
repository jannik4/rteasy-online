use super::expression::{
    generate_bus, generate_expression, generate_register, generate_register_array,
};
use crate::vhdl::*;
use compiler::mir;

pub fn generate_assignment<'s>(assignment: &mir::Assignment<'s>) -> Assignment<'s> {
    Assignment {
        lhs: generate_lvalue(&assignment.lhs),
        rhs: generate_expression(&assignment.rhs),
        size: assignment.size,
    }
}

fn generate_lvalue<'s>(lvalue: &mir::Lvalue<'s>) -> Lvalue<'s> {
    match lvalue {
        mir::Lvalue::Register(reg) => Lvalue::Register(generate_register(reg)),
        mir::Lvalue::Bus(bus) => Lvalue::Bus(generate_bus(bus)),
        mir::Lvalue::RegisterArray(reg_array) => {
            Lvalue::RegisterArray(generate_register_array(reg_array))
        }
        mir::Lvalue::ConcatClocked(_concat) => todo!(),
        mir::Lvalue::ConcatUnclocked(_concat) => todo!(),
    }
}
