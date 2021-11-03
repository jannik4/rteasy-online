use super::sim::{sim, Result, SimState};
use crate::mir::*;
use crate::symbols::Symbols;
use crate::{CompilerError, CompilerErrorKind};

pub fn check(
    _symbols: &Symbols<'_>,
    mir: &Mir<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result {
    for statement in &mir.statements {
        let state = State::new();
        sim(statement, state, error_sink)?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct State {
    gotos: usize,
}

impl State {
    fn new() -> Self {
        Self { gotos: 0 }
    }

    fn add_goto(&mut self) {
        self.gotos += 1;
    }
}

impl<'s> SimState<'s> for State {
    fn condition(&mut self, _: &Expression<'s>) -> Result {
        Ok(())
    }
    fn nop(&mut self, _: &Nop) -> Result {
        Ok(())
    }
    fn goto(&mut self, _: &Goto<'s>) -> Result {
        self.add_goto();
        Ok(())
    }
    fn write(&mut self, _: &Write<'s>) -> Result {
        Ok(())
    }

    fn read(&mut self, _: &Read<'s>) -> Result {
        Ok(())
    }

    fn assignment(&mut self, _: &Assignment<'s>) -> Result {
        Ok(())
    }

    fn assert(&mut self, _: &Assert<'_>) -> Result {
        Ok(())
    }

    fn finish(self, statement: &Statement<'s>, error_sink: &mut impl FnMut(CompilerError)) {
        if self.gotos > 1 {
            error_sink(CompilerError::new(CompilerErrorKind::DoubleGoto, statement.steps.span));
        }
    }
}
