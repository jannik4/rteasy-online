use super::Simulator;
use crate::{
    execute::{Execute, ExecuteResult},
    Error,
};
use rtcore::program::{Criterion, CriterionId, Label, Span};
use std::{collections::HashSet, mem};

impl Simulator {
    pub fn step(&mut self) -> Result<Option<StepResult>, Error> {
        self.step_(false)
    }

    pub fn micro_step(&mut self) -> Result<Option<StepResult>, Error> {
        self.step_(true)
    }

    fn step_(&mut self, micro: bool) -> Result<Option<StepResult>, Error> {
        loop {
            // Get cursor
            let cursor = match &mut self.cursor {
                Some(cursor) => cursor,
                None => break Ok(None),
            };

            let is_at_statement_start = cursor.is_at_statement_start();

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

            // Clear intern buses if cursor is at a new statement
            if is_at_statement_start {
                self.state.clear_intern_buses(&mem::take(&mut self.buses_persist));
            }

            // Execute step
            let step_result = if criteria_match(&step.criteria, &cursor.criteria_set) {
                Some(match step.operation.execute(&self.state)? {
                    ExecuteResult::Void => {
                        StepResult { is_at_statement_start, span: step.span(), condition: None }
                    }
                    ExecuteResult::Criterion(Criterion::True(id), cond_span) => {
                        cursor.criteria_set.insert(id);
                        StepResult {
                            is_at_statement_start,
                            span: step.span(),
                            condition: Some((true, cond_span)),
                        }
                    }
                    ExecuteResult::Criterion(Criterion::False(_), cond_span) => StepResult {
                        is_at_statement_start,
                        span: step.span(),
                        condition: Some((false, cond_span)),
                    },
                    ExecuteResult::Goto(label) => {
                        cursor.goto = Some(label);
                        StepResult { is_at_statement_start, span: step.span(), condition: None }
                    }
                    ExecuteResult::AssertError => {
                        self.cursor = None;
                        // TODO: Return error instead ???
                        break Ok(Some(StepResult {
                            is_at_statement_start,
                            span: step.span(),
                            condition: Some((false, step.span())),
                        }));
                    }
                })
            } else {
                None
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
                if let Some(step_result) = step_result {
                    break Ok(Some(step_result));
                }
            } else {
                if statement_completed {
                    break Ok(Some(StepResult {
                        is_at_statement_start,
                        span: statement.steps.span,
                        condition: None,
                    }));
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StepResult {
    pub is_at_statement_start: bool,
    pub span: Span,
    pub condition: Option<(bool, Span)>,
}

#[derive(Debug)]
pub struct Cursor {
    statement_idx: usize,
    step_idx: usize,
    criteria_set: HashSet<CriterionId>,
    goto: Option<Label>,
}

impl Cursor {
    pub fn new(statement_idx: usize) -> Self {
        Self { statement_idx, step_idx: 0, criteria_set: HashSet::new(), goto: None }
    }

    pub fn is_at_statement_start(&self) -> bool {
        self.step_idx == 0
    }
}

fn criteria_match(criteria: &[Criterion], criteria_set: &HashSet<CriterionId>) -> bool {
    criteria.iter().all(|criterion| match criterion {
        Criterion::True(id) => criteria_set.contains(id),
        Criterion::False(id) => !criteria_set.contains(id),
    })
}
