use super::sim::{sim, Result, SimState};
use crate::mir::*;
use crate::symbols::Symbols;
use crate::{CompilerError, CompilerErrorKind};
use std::collections::HashMap;

pub fn check(
    symbols: &Symbols<'_>,
    mir: &Mir<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result {
    for statement in &mir.statements {
        let state = State::new(symbols);
        sim(statement, state, error_sink)?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct State<'s> {
    _symbols: &'s Symbols<'s>,
    reads: HashMap<Ident<'s>, usize>,
}

impl<'s> State<'s> {
    fn new(symbols: &'s Symbols<'s>) -> Self {
        Self { _symbols: symbols, reads: HashMap::new() }
    }

    fn insert(&mut self, name: Ident<'s>) {
        *self.reads.entry(name).or_default() += 1;
    }
}

impl<'s> SimState<'s> for State<'s> {
    fn condition(&mut self, expression: &Expression<'s>) -> Result {
        visit_expression(expression, self);
        Ok(())
    }
    fn nop(&mut self, _: &Nop) -> Result {
        Ok(())
    }
    fn goto(&mut self, _: &Goto<'s>) -> Result {
        Ok(())
    }
    fn write(&mut self, _: &Write<'s>) -> Result {
        Ok(())
    }

    fn read(&mut self, _: &Read<'s>) -> Result {
        Ok(())
    }

    fn assignment(&mut self, assignment: &Assignment<'s>) -> Result {
        visit_expression(&assignment.rhs, self);
        Ok(())
    }

    fn assert(&mut self, _: &Assert<'_>) -> Result {
        Ok(())
    }

    fn finish(self, statement: &Statement<'s>, error_sink: &mut impl FnMut(CompilerError)) {
        let allowed_reads = 2;
        for (name, reads) in self.reads {
            if reads > allowed_reads {
                error_sink(CompilerError::new(
                    CompilerErrorKind::RegisterArrayTooManyReads {
                        name: name.0.to_string(),
                        allowed: allowed_reads,
                    },
                    statement.steps.span,
                ));
            }
        }
    }
}

fn visit_expression<'s>(expression: &Expression<'s>, state: &mut State<'s>) {
    match expression {
        Expression::Atom(atom) => match atom {
            Atom::Concat(concat) => {
                for part in &concat.parts {
                    match part {
                        ConcatPartExpr::RegisterArray(reg_array) => {
                            state.insert(reg_array.ident.node);
                        }
                        ConcatPartExpr::Register(_)
                        | ConcatPartExpr::Bus(_)
                        | ConcatPartExpr::Number(_) => (),
                    }
                }
            }
            Atom::RegisterArray(reg_array) => {
                state.insert(reg_array.ident.node);
            }
            Atom::Register(_) | Atom::Bus(_) | Atom::Number(_) => (),
        },
        Expression::BinaryTerm(term) => {
            visit_expression(&term.lhs, state);
            visit_expression(&term.rhs, state);
        }
        Expression::UnaryTerm(term) => {
            visit_expression(&term.expression, state);
        }
    }
}
