use super::expression::{
    generate_bus, generate_number, generate_register, generate_register_array,
};
use crate::vhdl::*;
use compiler::mir;

pub fn generate_concat_expr<'s>(concat: &mir::ConcatExpr<'s>) -> ConcatExpr<'s> {
    ConcatExpr { parts: concat.parts.iter().map(generate_concat_part_expr).collect() }
}

fn generate_concat_part_expr<'s>(concat: &mir::ConcatPartExpr<'s>) -> ConcatPartExpr<'s> {
    match concat {
        mir::ConcatPartExpr::Register(reg) => ConcatPartExpr::Register(generate_register(reg)),
        mir::ConcatPartExpr::Bus(bus) => ConcatPartExpr::Bus(generate_bus(bus)),
        mir::ConcatPartExpr::RegisterArray(reg_array) => {
            ConcatPartExpr::RegisterArray(generate_register_array(reg_array))
        }
        mir::ConcatPartExpr::Number(number) => {
            ConcatPartExpr::Number(generate_number(&number.node))
        }
    }
}

// TODO: ...
