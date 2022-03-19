mod impl_state_view;
mod impl_step;

use self::impl_step::Cursor;
use crate::{state::State, Changed};
use rtprogram::{Ident, Label, Program, Signals, Span};
use std::collections::{BTreeSet, HashSet};

pub struct Simulator {
    cycle_count: usize,
    state: State,
    buses_persist: HashSet<Ident>,

    program: Program,
    cursor: Cursor,

    breakpoints: BTreeSet<usize>,
}

impl Simulator {
    pub fn init(program: Program) -> Self {
        Self {
            cycle_count: 0,
            state: State::init(&program),
            buses_persist: HashSet::new(),

            program,
            cursor: Cursor::new(0),

            breakpoints: BTreeSet::new(),
        }
    }

    pub fn reset(&mut self, reset_breakpoints: bool) {
        self.cycle_count = 0;
        self.state = State::init(&self.program);
        self.buses_persist = HashSet::new();

        self.cursor = Cursor::new(0);
        if reset_breakpoints {
            self.breakpoints = BTreeSet::new();
        }
    }

    pub fn cycle_count(&self) -> usize {
        self.cycle_count
    }

    pub fn is_finished(&self) -> bool {
        !self.cursor.is_live()
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn signals(&self) -> Signals {
        self.program.signals()
    }

    pub fn statement_span(&self, statement: usize) -> Option<Span> {
        self.program.statements().get(statement).map(|s| s.steps.span)
    }

    pub fn add_breakpoint(&mut self, statement: usize) {
        if statement < self.program.statements().len() {
            self.breakpoints.insert(statement);
        }
    }

    pub fn remove_breakpoint(&mut self, statement: usize) {
        self.breakpoints.remove(&statement);
    }

    pub fn add_breakpoint_at_label(&mut self, label: &Label) {
        if let Some(statement) = self.statement_idx(label) {
            self.breakpoints.insert(statement);
        }
    }

    pub fn remove_breakpoint_at_label(&mut self, label: &Label) {
        if let Some(statement) = self.statement_idx(label) {
            self.breakpoints.remove(&statement);
        }
    }

    pub fn breakpoints(&self) -> impl Iterator<Item = usize> + '_ {
        self.breakpoints.iter().copied()
    }

    fn statement_idx(&self, label: &Label) -> Option<usize> {
        self.program
            .statements()
            .iter()
            .position(|stmt| stmt.label.as_ref().map(|s| &s.node) == Some(label))
    }
}

#[derive(Debug)]
pub struct StepResult {
    pub statement: usize,
    pub span: Span,
    pub kind: StepResultKind,
}

#[derive(Debug)]
pub enum StepResultKind {
    Void,
    Condition { result: bool, span: Span },
    Pipe(Changed),
    StatementEnd(Changed),
    Breakpoint,
    AssertError,
}
