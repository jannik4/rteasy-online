use crate::common::OperatorAssociativity;
use std::cmp::Ordering;

pub fn parentheses_binary_left(
    precedence: u32,
    inner_precedence: u32,
    associativity: OperatorAssociativity,
) -> bool {
    match inner_precedence.cmp(&precedence) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => match associativity {
            OperatorAssociativity::Left => false,
            OperatorAssociativity::Right => true,
        },
    }
}

pub fn parentheses_binary_right(
    precedence: u32,
    inner_precedence: u32,
    associativity: OperatorAssociativity,
) -> bool {
    match inner_precedence.cmp(&precedence) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => match associativity {
            OperatorAssociativity::Left => true,
            OperatorAssociativity::Right => false,
        },
    }
}

pub fn parentheses_unary(precedence: u32, inner_precedence: u32) -> bool {
    match inner_precedence.cmp(&precedence) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => false,
    }
}
