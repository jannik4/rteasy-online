use crate::{
    symbols::{Symbol, Symbols},
    util, CompilerError, SymbolType,
};
use rtcore::ast::*;

#[derive(Debug)]
pub struct Res {
    /// Size in bits
    pub size: Option<usize>,
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
            Self::Number(number) => number.check_expr(symbols, error_sink),
        }
    }
}

impl<'s> CheckExpr<'s> for BinaryTerm<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let lhs = self.lhs.check_expr(symbols, error_sink);
        let rhs = self.rhs.check_expr(symbols, error_sink);

        match (lhs.size, rhs.size) {
            (Some(lhs), Some(rhs)) => {
                Res { size: Some(util::size_binary_op(lhs, rhs, self.operator)) }
            }
            _ => Res { size: None },
        }
    }
}

impl<'s> CheckExpr<'s> for UnaryTerm<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        if self.operator == UnaryOperator::Sxt {
            match self.expression {
                Expression::Atom(_) => (),
                Expression::BinaryTerm(_) | Expression::UnaryTerm(_) => {
                    error_sink(CompilerError::SxtTerm);
                }
            }
        }

        let res = self.expression.check_expr(symbols, error_sink);

        match res.size {
            Some(lhs) => Res { size: Some(util::size_unary_op(lhs, self.operator)) },
            None => Res { size: None },
        }
    }
}

impl<'s> CheckExpr<'s> for Concat<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let info = util::concat_info(self, symbols);
        if info.contains_number_non_bit_string {
            error_sink(CompilerError::ConcatContainsNumberNonBitString);
        }

        let mut size = Some(0);

        for part in &self.parts {
            size = match (size, part.check_expr(symbols, error_sink).size) {
                (Some(curr), Some(part)) => Some(curr + part),
                _ => None,
            };
        }

        Res { size }
    }
}

impl<'s> CheckExpr<'s> for ConcatPart<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match self {
            Self::RegBus(reg_bus) => reg_bus.check_expr(symbols, error_sink),
            Self::RegisterArray(reg_array) => reg_array.check_expr(symbols, error_sink),
            Self::Number(number) => number.check_expr(symbols, error_sink),
        }
    }
}

impl<'s> CheckExpr<'s> for RegBus<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match symbols.symbol(self.ident) {
            Some(Symbol::Register(range)) => match util::range_into(range, self.range) {
                Ok(size) => Res { size: Some(size) },
                Err(e) => {
                    error_sink(e);
                    Res { size: None }
                }
            },
            Some(Symbol::Bus(range)) => match util::range_into(range, self.range) {
                Ok(size) => Res { size: Some(size) },
                Err(e) => {
                    error_sink(e);
                    Res { size: None }
                }
            },
            Some(Symbol::RegisterArray { .. }) => {
                error_sink(CompilerError::RegArrayMissingIndex(self.ident.0.to_string()));
                Res { size: None }
            }
            Some(symbol) => {
                error_sink(CompilerError::WrongSymbolType {
                    expected: &[SymbolType::Register, SymbolType::Bus],
                    found: symbol.type_(),
                });
                Res { size: None }
            }
            _ => {
                error_sink(CompilerError::SymbolNotFound(
                    &[SymbolType::Register, SymbolType::Bus],
                    self.ident.0.to_string(),
                ));
                Res { size: None }
            }
        }
    }
}

impl<'s> CheckExpr<'s> for RegisterArray<'s> {
    fn check_expr(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let index_expr = self.index.check_expr(symbols, error_sink);

        match symbols.symbol(self.ident) {
            Some(Symbol::RegisterArray { range, len }) => {
                let index_size = util::log_2(len);
                if let Some(index_expr_size) = index_expr.size {
                    if index_size < index_expr_size {
                        error_sink(CompilerError::RegArrayIndexDoesNotFit(
                            index_size,
                            index_expr_size,
                        ))
                    }
                }

                Res { size: Some(range.map(|range| range.size()).unwrap_or(1)) }
            }
            Some(symbol) => {
                error_sink(CompilerError::WrongSymbolType {
                    expected: &[SymbolType::RegisterArray],
                    found: symbol.type_(),
                });
                Res { size: None }
            }
            _ => {
                error_sink(CompilerError::SymbolNotFound(
                    &[SymbolType::RegisterArray],
                    self.ident.0.to_string(),
                ));
                Res { size: None }
            }
        }
    }
}

impl<'s> CheckExpr<'s> for Number {
    fn check_expr(&self, _: &Symbols<'_>, _: &mut impl FnMut(CompilerError)) -> Res {
        Res { size: Some(self.value.size()) }
    }
}
