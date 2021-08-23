use crate::{
    symbols::{Symbol, Symbols},
    CompilerError,
};
use rtcore::ast::{BinaryOperator, BitRange, Concat, ConcatPart, NumberKind, UnaryOperator};
use rtcore::common::CtxSize;
use std::cmp;

pub fn size_binary_op(lhs: usize, rhs: usize, op: BinaryOperator) -> usize {
    match op {
        BinaryOperator::Eq
        | BinaryOperator::Ne
        | BinaryOperator::Le
        | BinaryOperator::Lt
        | BinaryOperator::Ge
        | BinaryOperator::Gt => 1,
        BinaryOperator::Add
        | BinaryOperator::Sub
        | BinaryOperator::And
        | BinaryOperator::Nand
        | BinaryOperator::Or
        | BinaryOperator::Nor
        | BinaryOperator::Xor => cmp::max(lhs, rhs),
    }
}

pub fn ctx_size_binary_op(lhs: usize, rhs: usize, op: BinaryOperator) -> CtxSize {
    match op {
        BinaryOperator::Eq
        | BinaryOperator::Ne
        | BinaryOperator::Le
        | BinaryOperator::Lt
        | BinaryOperator::Ge
        | BinaryOperator::Gt => CtxSize::Size(cmp::max(lhs, rhs)),
        BinaryOperator::Add
        | BinaryOperator::Sub
        | BinaryOperator::And
        | BinaryOperator::Nand
        | BinaryOperator::Or
        | BinaryOperator::Nor
        | BinaryOperator::Xor => CtxSize::Inherit,
    }
}

pub fn size_unary_op(lhs: usize, op: UnaryOperator) -> usize {
    match op {
        UnaryOperator::SignNeg => lhs,
        UnaryOperator::Not => lhs,
        UnaryOperator::Sxt => lhs,
    }
}

pub fn ctx_size_unary_op(lhs: usize, op: UnaryOperator) -> CtxSize {
    match op {
        UnaryOperator::SignNeg | UnaryOperator::Not => CtxSize::Inherit,
        UnaryOperator::Sxt => CtxSize::Size(lhs),
    }
}

pub fn range_into(
    range: Option<BitRange>,
    range_idx: Option<BitRange>,
) -> Result<usize, CompilerError> {
    let range = range.unwrap_or_default();
    let range_idx = match range_idx {
        Some(range_idx) => range_idx,
        None => return Ok(range.size()),
    };

    let size = match range_idx {
        BitRange { msb, lsb: Some(lsb) } => {
            if !range.contains(msb) || !range.contains(lsb) {
                return Err(CompilerError::RangeMismatch);
            }
            if (msb >= lsb) != (range.msb >= range.lsb.unwrap_or(range.msb)) {
                return Err(CompilerError::RangeMismatch);
            }

            range_idx.size()
        }
        BitRange { msb, lsb: None } => {
            if !range.contains(msb) {
                return Err(CompilerError::RangeMismatch);
            }

            1
        }
    };

    Ok(size)
}

#[derive(Debug)]
pub struct ConcatInfo {
    pub contains_clocked: bool,
    pub contains_unclocked: bool,
    pub contains_non_lvalue: bool,
    pub contains_number_non_bit_string: bool,
}

pub fn concat_info(concat: &Concat<'_>, symbols: &Symbols<'_>) -> ConcatInfo {
    let mut info = ConcatInfo {
        contains_clocked: false,
        contains_unclocked: false,
        contains_non_lvalue: false,
        contains_number_non_bit_string: false,
    };

    for part in &concat.parts {
        match part {
            ConcatPart::RegBus(reg_bus) => match symbols.symbol(reg_bus.ident) {
                Some(Symbol::Register(..)) => info.contains_clocked = true,
                Some(Symbol::Bus(..)) => info.contains_unclocked = true,
                _ => (),
            },
            ConcatPart::RegisterArray(_) => info.contains_clocked = true,
            ConcatPart::Number(number) => {
                info.contains_non_lvalue = true;
                if number.kind == NumberKind::Other {
                    info.contains_number_non_bit_string = true;
                }
            }
        }
    }

    info
}

pub fn log_2(x: usize) -> usize {
    const fn num_bits<T>() -> usize {
        std::mem::size_of::<T>() * 8
    }

    if x == 0 {
        0
    } else {
        num_bits::<usize>() - x.leading_zeros() as usize - 1
    }
}
