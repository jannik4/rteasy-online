use crate::{execute::Execute, ChangeSet, Error, State};
use rtcore::{
    program::{Bus, Criterion, CriterionId, Program},
    value::Value,
};
use std::collections::HashSet;
use std::mem;

pub struct Simulator {
    state: State,
    change_set: ChangeSet,
    program: Program,
    cursor: Option<Cursor>,
}

impl Simulator {
    pub fn init(program: Program) -> Self {
        Self {
            state: State::init(&program),
            change_set: ChangeSet::new(),
            program,
            cursor: Some(Cursor::new(0)),
        }
    }

    // pub fn apply_change_set(&mut self, change_set: ChangeSet) -> Result<(), Error> {
    //     // TODO: Handle change_set with goto label (Remove assert is_none)
    //     assert!(self.state.apply_change_set(change_set)?.is_none());
    //     Ok(())
    // }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn is_finished(&self) -> bool {
        self.cursor.is_none()
    }

    pub fn write_into_bus(&mut self, bus: &Bus, value: Value) -> Result<(), Error> {
        self.state.write_into_bus(bus, value)
    }

    pub fn step(&mut self) -> Result<Option<std::ops::Range<usize>>, Error> {
        self.step_(false)
    }

    pub fn micro_step(&mut self) -> Result<Option<std::ops::Range<usize>>, Error> {
        self.step_(true)
    }

    pub fn step_(&mut self, micro: bool) -> Result<Option<std::ops::Range<usize>>, Error> {
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
            let (step, _is_pre_pipe) = match statement.steps.get(cursor.step_idx) {
                Some((step, is_pre_pipe)) => (step, is_pre_pipe),
                None => {
                    self.cursor = None;
                    break Ok(None);
                }
            };

            // Execute step
            let step_executed = if criteria_match(&step.criteria, &cursor.criteria_set) {
                let res = step.operation.execute(&mut self.state, &mut self.change_set)?;
                for criterion in res {
                    cursor.criteria_set.insert(criterion);
                }
                true
            } else {
                false
            };

            // Advance cursor
            cursor.step_idx += 1;

            // Check if statement completed (= no steps with matching criteria left)
            let statement_completed = statement.steps[cursor.step_idx..]
                .iter()
                .all(|step| !criteria_match(&step.criteria, &cursor.criteria_set));

            if statement_completed {
                // Apply changes
                let goto_label = self.state.apply_change_set(mem::take(&mut self.change_set))?;

                // Update cursor
                let next_statement_idx = match goto_label {
                    Some(goto_label) => self
                        .program
                        .statements()
                        .iter()
                        .position(|stmt| stmt.label.as_ref() == Some(&goto_label))
                        .ok_or(Error::Other)?,
                    None => cursor.statement_idx + 1,
                };
                *cursor = Cursor::new(next_statement_idx);

                // Finish cycle
                self.state.finish_cycle();
            }
            // Else check if steps pre pipe completed
            else if cursor.step_idx == statement.steps.split_at() {
                self.state.apply_change_set(mem::take(&mut self.change_set))?;
            }

            // Break, if progress has been made
            if micro {
                if step_executed {
                    break Ok(Some(step.operation.span.clone()));
                }
            } else {
                if statement_completed {
                    break Ok(Some(statement.span.clone()));
                }
            }
        }
    }
}

#[derive(Debug)]
struct StepResult {
    statement_span: std::ops::Range<usize>,
    step_span: std::ops::Range<usize>,
    statement_completed: bool,
}

#[derive(Debug)]
struct Cursor {
    statement_idx: usize,
    step_idx: usize,
    criteria_set: HashSet<CriterionId>,
}

impl Cursor {
    fn new(statement_idx: usize) -> Self {
        Self { statement_idx, step_idx: 0, criteria_set: HashSet::new() }
    }
}

fn criteria_match(criteria: &[Criterion], criteria_set: &HashSet<CriterionId>) -> bool {
    criteria.iter().all(|criterion| match criterion {
        Criterion::True(id) => criteria_set.contains(id),
        Criterion::False(id) => !criteria_set.contains(id),
    })
}
