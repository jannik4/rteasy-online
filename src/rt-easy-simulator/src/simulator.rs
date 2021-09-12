use crate::{
    execute::{Execute, ExecuteResult},
    state::State,
    Error,
};
use rtcore::{
    program::{Criterion, CriterionId, Ident, Label, Program, Span},
    value::Value,
};
use std::collections::HashSet;
use std::mem;

pub struct Simulator {
    cycle_count: usize,
    state: State,
    buses_persist: HashSet<Ident>,

    program: Program,
    cursor: Option<Cursor>,
}

impl Simulator {
    pub fn init(program: Program) -> Self {
        Self {
            cycle_count: 0,
            state: State::init(&program),
            buses_persist: HashSet::new(),

            program,
            cursor: Some(Cursor::new(0)),
        }
    }

    pub fn reset(&mut self) {
        self.cycle_count = 0;
        self.state = State::init(&self.program);
        self.buses_persist = HashSet::new();

        self.cursor = Some(Cursor::new(0));
    }

    pub fn cycle_count(&self) -> usize {
        self.cycle_count
    }

    pub fn is_finished(&self) -> bool {
        self.cursor.is_none()
    }

    pub fn step(&mut self) -> Result<Option<Span>, Error> {
        self.step_(false)
    }

    pub fn micro_step(&mut self) -> Result<Option<Span>, Error> {
        self.step_(true)
    }

    pub fn step_(&mut self, micro: bool) -> Result<Option<Span>, Error> {
        loop {
            // Get cursor
            let cursor = match &mut self.cursor {
                Some(cursor) => cursor,
                None => break Ok(None),
            };

            // Get current statement
            let statement = match self.program.statements().get(cursor.statement_idx) {
                Some(statement) => statement,
                None => {
                    self.cursor = None;
                    break Ok(None);
                }
            };

            // Get current step
            let (step, _is_pre_pipe) = match statement.steps.node.get(cursor.step_idx) {
                Some((step, is_pre_pipe)) => (step, is_pre_pipe),
                None => {
                    self.cursor = None;
                    break Ok(None);
                }
            };

            // Clear buses if cursor is at a new statement
            if cursor.is_at_statement_start() {
                self.state.clear_buses(&mem::take(&mut self.buses_persist));
            }

            // Execute step
            let step_executed = if criteria_match(&step.criteria, &cursor.criteria_set) {
                match step.operation.execute(&self.state)? {
                    ExecuteResult::Void => (),
                    ExecuteResult::Criterion(Criterion::True(id)) => {
                        cursor.criteria_set.insert(id);
                    }
                    ExecuteResult::Criterion(Criterion::False(_)) => (),
                    ExecuteResult::Goto(label) => cursor.goto = Some(label),
                }
                true
            } else {
                false
            };

            // Advance cursor
            cursor.step_idx += 1;

            // Check if statement completed (= no steps with matching criteria left)
            let statement_completed = statement.steps.node[cursor.step_idx..]
                .iter()
                .all(|step| !criteria_match(&step.criteria, &cursor.criteria_set));

            if statement_completed {
                // Apply changes
                self.state.clock();

                // Update cursor
                let next_statement_idx = match cursor.goto.take() {
                    Some(goto_label) => self
                        .program
                        .statements()
                        .iter()
                        .position(|stmt| stmt.label.as_ref().map(|s| &s.node) == Some(&goto_label))
                        .ok_or(Error::Other)?,
                    None => cursor.statement_idx + 1,
                };
                *cursor = Cursor::new(next_statement_idx);

                // Finish cycle
                self.cycle_count += 1;
            }
            // Else check if steps pre pipe completed
            else if cursor.step_idx == statement.steps.node.split_at() {
                self.state.clock();
            }

            // Break, if progress has been made
            if micro {
                if step_executed {
                    break Ok(Some(step.operation.span));
                }
            } else {
                if statement_completed {
                    break Ok(Some(statement.steps.span));
                }
            }
        }
    }
}

#[derive(Debug)]
struct Cursor {
    statement_idx: usize,
    step_idx: usize,
    criteria_set: HashSet<CriterionId>,
    goto: Option<Label>,
}

impl Cursor {
    fn new(statement_idx: usize) -> Self {
        Self { statement_idx, step_idx: 0, criteria_set: HashSet::new(), goto: None }
    }

    fn is_at_statement_start(&self) -> bool {
        self.step_idx == 0
    }
}

fn criteria_match(criteria: &[Criterion], criteria_set: &HashSet<CriterionId>) -> bool {
    criteria.iter().all(|criterion| match criterion {
        Criterion::True(id) => criteria_set.contains(id),
        Criterion::False(id) => !criteria_set.contains(id),
    })
}

impl Simulator {
    // ------------------------------------------------------------
    // Registers
    // ------------------------------------------------------------

    pub fn registers(&self) -> impl Iterator<Item = &Ident> {
        self.state.register_names()
    }
    pub fn register_value(&self, name: &Ident) -> Result<Value, Error> {
        self.state.register(name)?.read(None)
    }
    pub fn register_value_next(&self, name: &Ident) -> Result<Option<Value>, Error> {
        Ok(self.state.register(name)?.value_next())
    }
    pub fn write_register(&mut self, name: &Ident, value: Value) -> Result<(), Error> {
        self.state.register_mut(name)?.write(None, value)?;
        self.state.register_mut(name)?.clock();
        Ok(())
    }

    // ------------------------------------------------------------
    // Buses
    // ------------------------------------------------------------

    pub fn buses(&self) -> impl Iterator<Item = &Ident> {
        self.state.bus_names()
    }
    pub fn bus_value(&self, name: &Ident) -> Result<Value, Error> {
        self.state.bus(name)?.read(None)
    }
    pub fn write_bus(&mut self, name: &Ident, value: Value) -> Result<(), Error> {
        self.state.bus_mut(name)?.write(None, value)?;

        // Persist bus value if between statements
        if self.cursor.as_ref().map(Cursor::is_at_statement_start).unwrap_or(false) {
            self.buses_persist.insert(name.clone());
        }

        Ok(())
    }

    // ------------------------------------------------------------
    // Memories
    // ------------------------------------------------------------

    pub fn memories(&self) -> impl Iterator<Item = &Ident> {
        self.state.memory_names()
    }
    pub fn memory_page_count(&self, name: &Ident) -> Result<Value, Error> {
        Ok(self.state.memory(name)?.page_count())
    }
    pub fn memory_page_prev(&self, name: &Ident, page_nr: Value) -> Result<Option<Value>, Error> {
        Ok(self.state.memory(name)?.page_prev(page_nr))
    }
    pub fn memory_page_next(&self, name: &Ident, page_nr: Value) -> Result<Option<Value>, Error> {
        Ok(self.state.memory(name)?.page_next(page_nr))
    }
    pub fn memory_page(&self, name: &Ident, page_nr: Value) -> Result<Vec<(Value, Value)>, Error> {
        Ok(self.state.memory(name)?.page(page_nr))
    }
    pub fn write_memory(&mut self, name: &Ident, addr: Value, value: Value) -> Result<(), Error> {
        Ok(self.state.memory_mut(name)?.write_at(addr, value)?)
    }
}
