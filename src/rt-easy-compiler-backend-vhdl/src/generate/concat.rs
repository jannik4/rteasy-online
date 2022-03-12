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

pub fn generate_concat_lvalue_clocked<'s>(
    concat: &mir::ConcatLvalueClocked<'s>,
    declarations: &Declarations<'s>,
) -> ConcatLvalueClocked<'s> {
    ConcatLvalueClocked {
        parts: concat
            .parts
            .iter()
            .map(|part| generate_concat_part_lvalue_clocked(part, declarations))
            .collect(),
    }
}

fn generate_concat_part_lvalue_clocked<'s>(
    part: &mir::ConcatPartLvalueClocked<'s>,
    declarations: &Declarations<'s>,
) -> ConcatPartLvalueClocked<'s> {
    match part {
        mir::ConcatPartLvalueClocked::Register(reg, size) => {
            ConcatPartLvalueClocked::Register(generate_register(reg, declarations), *size)
        }
        mir::ConcatPartLvalueClocked::RegisterArray(reg_array, size) => {
            ConcatPartLvalueClocked::RegisterArray(
                generate_register_array(reg_array, declarations),
                *size,
            )
        }
    }
}

pub fn generate_concat_lvalue_unclocked<'s>(
    concat: &mir::ConcatLvalueUnclocked<'s>,
    declarations: &Declarations<'s>,
) -> ConcatLvalueUnclocked<'s> {
    ConcatLvalueUnclocked {
        parts: concat
            .parts
            .iter()
            .map(|part| generate_concat_part_lvalue_unclocked(part, declarations))
            .collect(),
    }
}

fn generate_concat_part_lvalue_unclocked<'s>(
    part: &mir::ConcatPartLvalueUnclocked<'s>,
    declarations: &Declarations<'s>,
) -> ConcatPartLvalueUnclocked<'s> {
    match part {
        mir::ConcatPartLvalueUnclocked::Bus(bus, size) => {
            ConcatPartLvalueUnclocked::Bus(generate_bus(bus, declarations), *size)
        }
    }
}
