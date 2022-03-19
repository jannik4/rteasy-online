use crate::{
    symbols::{Symbol, Symbols},
    util, CompilerError, CompilerErrorKind, SymbolType,
};
use rtast::*;

#[derive(Debug)]
pub struct Res {
    /// Size in bits
    pub size: Option<usize>,
    pub fixed_size: bool,
    pub constant: bool,
}

pub trait CheckExpr<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res;
}

impl<'s> CheckExpr<'s> for Expression<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match self {
            Self::Atom(atom) => atom.check_expr(symbols, error_sink),
            Self::BinaryTerm(term) => term.check_expr(symbols, error_sink),
            Self::UnaryTerm(term) => term.check_expr(symbols, error_sink),
        }
    }
}

impl<'s> CheckExpr<'s> for Atom<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match self {
            Self::Concat(concat) => concat.check_expr(symbols, error_sink),
            Self::RegBus(reg_bus) => reg_bus.check_expr(symbols, error_sink),
            Self::RegisterArray(reg_array) => reg_array.check_expr(symbols, error_sink),
            Self::Number(number) => number.node.check_expr(symbols, error_sink),
        }
    }
}

impl<'s> CheckExpr<'s> for BinaryTerm<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let lhs = self.lhs.check_expr(symbols, error_sink);
        let rhs = self.rhs.check_expr(symbols, error_sink);

        Res {
            size: match (lhs.size, rhs.size) {
                (Some(lhs), Some(rhs)) => Some(util::size_binary_op(lhs, rhs, self.operator.node)),
                _ => None,
            },
            fixed_size: util::is_fixed_size_binary_op(self.operator.node),
            constant: lhs.constant && rhs.constant,
        }
    }
}

impl<'s> CheckExpr<'s> for UnaryTerm<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        if self.operator.node == UnaryOperator::Sxt {
            match self.expression {
                Expression::Atom(_) => (),
                Expression::BinaryTerm(_) | Expression::UnaryTerm(_) => {
                    error_sink(CompilerError::new(CompilerErrorKind::SxtTerm, self.span));
                }
            }
        }

        let res = self.expression.check_expr(symbols, error_sink);

        Res {
            size: match res.size {
                Some(lhs) => Some(util::size_unary_op(lhs, self.operator.node)),
                None => None,
            },
            fixed_size: util::is_fixed_size_unary_op(self.operator.node),
            constant: res.constant,
        }
    }
}

impl<'s> CheckExpr<'s> for Concat<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let info = util::concat_info(self, symbols);
        if info.contains_number_non_bit_string {
            error_sink(CompilerError::new(
                CompilerErrorKind::ConcatContainsNumberNonBitString,
                self.span,
            ));
        }

        let mut size = Some(0);
        let mut constant = true;

        for part in &self.parts {
            let part_res = part.check_expr(symbols, error_sink);
            size = match (size, part_res.size) {
                (Some(curr), Some(part)) => Some(curr + part),
                _ => None,
            };
            constant &= part_res.constant;
        }

        Res { size, fixed_size: true, constant }
    }
}

impl<'s> CheckExpr<'s> for ConcatPart<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match self {
            Self::RegBus(reg_bus) => reg_bus.check_expr(symbols, error_sink),
            Self::RegisterArray(reg_array) => reg_array.check_expr(symbols, error_sink),
            Self::Number(number) => number.node.check_expr(symbols, error_sink),
        }
    }
}

impl<'s> CheckExpr<'s> for RegBus<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let size = match symbols.symbol(self.ident.node) {
            Some(Symbol::Register(range, _)) => match util::range_into(range, self.range) {
                Ok(size) => Some(size),
                Err(e) => {
                    error_sink(e);
                    None
                }
            },
            Some(Symbol::Bus(range, _)) => match util::range_into(range, self.range) {
                Ok(size) => Some(size),
                Err(e) => {
                    error_sink(e);
                    None
                }
            },
            Some(Symbol::RegisterArray { .. }) => {
                error_sink(CompilerError::new(
                    CompilerErrorKind::RegArrayMissingIndex(self.ident.node.0.to_string()),
                    self.ident.span,
                ));
                None
            }
            Some(symbol) => {
                error_sink(CompilerError::new(
                    CompilerErrorKind::WrongSymbolType {
                        expected: &[SymbolType::Register, SymbolType::Bus],
                        found: symbol.type_(),
                    },
                    self.ident.span,
                ));
                None
            }
            _ => {
                error_sink(CompilerError::new(
                    CompilerErrorKind::SymbolNotFound(
                        &[SymbolType::Register, SymbolType::Bus],
                        self.ident.node.0.to_string(),
                    ),
                    self.ident.span,
                ));
                None
            }
        };

        Res { size, fixed_size: true, constant: false }
    }
}

impl<'s> CheckExpr<'s> for RegisterArray<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let index_expr = self.index.check_expr(symbols, error_sink);

        let size = match symbols.symbol(self.ident.node) {
            Some(Symbol::RegisterArray { range, len }) => {
                let index_size = util::log_2(len);
                if let Some(index_expr_size) = index_expr.size {
                    if index_size < index_expr_size {
                        error_sink(CompilerError::new(
                            CompilerErrorKind::RegArrayIndexDoesNotFit {
                                index_size,
                                index_expr_size,
                            },
                            self.index.span(),
                        ))
                    }
                }

                Some(range.map(|range| range.size()).unwrap_or(1))
            }
            Some(symbol) => {
                error_sink(CompilerError::new(
                    CompilerErrorKind::WrongSymbolType {
                        expected: &[SymbolType::RegisterArray],
                        found: symbol.type_(),
                    },
                    self.ident.span,
                ));
                None
            }
            _ => {
                error_sink(CompilerError::new(
                    CompilerErrorKind::SymbolNotFound(
                        &[SymbolType::RegisterArray],
                        self.ident.node.0.to_string(),
                    ),
                    self.ident.span,
                ));
                None
            }
        };

        Res { size, fixed_size: true, constant: false }
    }
}

impl<'s> CheckExpr<'s> for Number {
    fn check_expr(&self, _: &Symbols<'_>, _: &mut impl FnMut(CompilerError)) -> Res {
        Res {
            size: Some(self.value.size()),
            fixed_size: match self.kind {
                NumberKind::BitString => true,
                NumberKind::Binary | NumberKind::Decimal | NumberKind::Hexadecimal => false,
            },
            constant: true,
        }
    }
}
