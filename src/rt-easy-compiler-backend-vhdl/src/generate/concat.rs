use super::expression::{
    generate_bus, generate_number, generate_register, generate_register_array,
};
use crate::vhdl::*;
use compiler::mir;

pub fn generate_concat_expr<'s>(
    concat: &mir::ConcatExpr<'s>,
    declarations: &Declarations<'s>,
) -> ConcatExpr<'s> {
    ConcatExpr {
        parts: concat
            .parts
            .iter()
            .map(|part| generate_concat_part_expr(part, declarations))
            .collect(),
    }
}

fn generate_concat_part_expr<'s>(
    part: &mir::ConcatPartExpr<'s>,
    declarations: &Declarations<'s>,
) -> ConcatPartExpr<'s> {
    match part {
        mir::ConcatPartExpr::Register(reg) => {
            ConcatPartExpr::Register(generate_register(reg, declarations))
        }
        mir::ConcatPartExpr::Bus(bus) => ConcatPartExpr::Bus(generate_bus(bus, declarations)),
        mir::ConcatPartExpr::RegisterArray(reg_array) => {
            ConcatPartExpr::RegisterArray(generate_register_array(reg_array, declarations))
        }
        mir::ConcatPartExpr::Number(number) => {
            ConcatPartExpr::Number(generate_number(&number.node))
        }
    }
}

// TODO: ...
