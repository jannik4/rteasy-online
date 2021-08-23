use super::expression::CheckExpr;
use crate::{
    symbols::{Symbol, Symbols},
    util, CompilerError, SymbolType,
};
use rtcore::ast::*;

#[derive(Debug, Default)]
pub struct Res {
    /// If the operation(s) contains a goto
    pub contains_goto: bool,
    /// If the operation(s) contains a mutating op
    pub contains_mutate: bool,
}

impl Res {
    fn merge(a: Self, b: Self) -> Self {
        Self {
            contains_goto: a.contains_goto || b.contains_goto,
            contains_mutate: a.contains_mutate || b.contains_mutate,
        }
    }
}

pub trait CheckOp<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res;
}

impl<'s> CheckOp<'s> for [Operation<'s>] {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let mut operations = self.iter();
        let mut res = operations
            .next()
            .expect("expected at least one operation")
            .check_op(symbols, error_sink);

        for operation in operations {
            res = Res::merge(res, operation.check_op(symbols, error_sink));
        }

        res
    }
}

impl<'s> CheckOp<'s> for Operation<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match &self.kind {
            OperationKind::Nop(nop) => nop.check_op(symbols, error_sink),
            OperationKind::Goto(goto) => goto.check_op(symbols, error_sink),
            OperationKind::If(if_) => if_.check_op(symbols, error_sink),
            OperationKind::Switch(switch) => switch.check_op(symbols, error_sink),
            OperationKind::Write(write) => write.check_op(symbols, error_sink),
            OperationKind::Read(read) => read.check_op(symbols, error_sink),
            OperationKind::Assignment(assignment) => assignment.check_op(symbols, error_sink),
        }
    }
}

impl<'s> CheckOp<'s> for Nop {
    fn check_op(&self, _: &Symbols<'_>, _: &mut impl FnMut(CompilerError)) -> Res {
        Res::default()
    }
}

impl<'s> CheckOp<'s> for Goto<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        if !symbols.contains_label(self.label) {
            error_sink(CompilerError::LabelNotFound(self.label.0.to_string()));
        }

        Res { contains_goto: true, contains_mutate: false }
    }
}

impl<'s> CheckOp<'s> for If<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        if let Some(size) = self.condition.check_expr(symbols, error_sink).size {
            if size > 1 {
                error_sink(CompilerError::ConditionToWide(size));
            }
        }

        let mut res = self.operations_if.check_op(symbols, error_sink);
        if let Some(operations_else) = &self.operations_else {
            res = Res::merge(res, operations_else.check_op(symbols, error_sink));
        }

        res
    }
}

impl<'s> CheckOp<'s> for Switch<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let expr_res = self.expression.check_expr(symbols, error_sink);
        if !expr_res.fixed_size {
            error_sink(CompilerError::ExpectedFixedSize);
        }

        let mut res = Res { contains_goto: false, contains_mutate: false };
        let mut default_clauses_count = 0;

        for clause in &self.clauses {
            match clause {
                Either::Left(case) => {
                    let value_res = case.value.check_expr(symbols, error_sink);

                    if !value_res.constant {
                        error_sink(CompilerError::ExpectedConstantExpression);
                    }

                    match (expr_res.size, value_res.size) {
                        (Some(expr_size), Some(value_size)) if value_size > expr_size => {
                            error_sink(CompilerError::CaseValueTooWide);
                        }
                        _ => (),
                    }

                    res = Res::merge(res, case.operations.check_op(symbols, error_sink));
                }
                Either::Right(default) => {
                    default_clauses_count += 1;
                    res = Res::merge(res, default.operations.check_op(symbols, error_sink));
                }
            }
        }

        if default_clauses_count != 1 {
            error_sink(CompilerError::ExpectedExactlyOneDefaultClause);
        }

        res
    }
}

impl<'s> CheckOp<'s> for Write<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match symbols.symbol(self.ident) {
            Some(Symbol::Memory(_)) => (),
            Some(symbol) => error_sink(CompilerError::WrongSymbolType {
                expected: &[SymbolType::Memory],
                found: symbol.type_(),
            }),
            _ => error_sink(CompilerError::SymbolNotFound(
                &[SymbolType::Memory],
                self.ident.0.to_string(),
            )),
        }

        Res { contains_goto: false, contains_mutate: true }
    }
}

impl<'s> CheckOp<'s> for Read<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match symbols.symbol(self.ident) {
            Some(Symbol::Memory(_)) => (),
            Some(symbol) => error_sink(CompilerError::WrongSymbolType {
                expected: &[SymbolType::Memory],
                found: symbol.type_(),
            }),
            _ => error_sink(CompilerError::SymbolNotFound(
                &[SymbolType::Memory],
                self.ident.0.to_string(),
            )),
        }

        Res { contains_goto: false, contains_mutate: true }
    }
}

impl<'s> CheckOp<'s> for Assignment<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        let lhs = match &self.lhs {
            Lvalue::RegBus(reg_bus) => reg_bus.check_expr(symbols, error_sink),
            Lvalue::RegisterArray(reg_array) => reg_array.check_expr(symbols, error_sink),
            Lvalue::Concat(concat) => {
                let info = util::concat_info(concat, symbols);
                if info.contains_clocked && info.contains_unclocked {
                    error_sink(CompilerError::AssignmentLhsContainsClockedAndUnclocked);
                }
                if info.contains_non_lvalue {
                    error_sink(CompilerError::AssignmentLhsContainsANonLvalue);
                }

                concat.check_expr(symbols, error_sink)
            }
        };

        let rhs = self.rhs.check_expr(symbols, error_sink);

        if let (Some(lhs), Some(rhs)) = (lhs.size, rhs.size) {
            if lhs < rhs {
                error_sink(CompilerError::AssignmentDoesNotFit(lhs, rhs))
            }
        }

        Res { contains_goto: false, contains_mutate: true }
    }
}

impl<'s, L, R> CheckOp<'s> for Either<L, R>
where
    L: CheckOp<'s>,
    R: CheckOp<'s>,
{
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Res {
        match self {
            Self::Left(left) => left.check_op(symbols, error_sink),
            Self::Right(right) => right.check_op(symbols, error_sink),
        }
    }
}