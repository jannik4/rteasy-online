use super::{Changed, Simulator, StepResult, StepResultKind};
use crate::{
    execute::{Execute, ExecuteResult},
    state::State,
    Error,
};
use rtcore::program::{Criterion, CriterionId, Label, Step};
use std::{collections::HashSet, mem};

impl Simulator {
    pub fn step(&mut self, stop_on_breakpoint: bool) -> Result<Option<StepResult>, Error> {
        let mut changed = Changed::default();

        loop {
            match self.micro_step(stop_on_breakpoint)? {
                Some(step_result) => match step_result.kind {
                    StepResultKind::Void => (),
                    StepResultKind::Condition { .. } => (),
                    StepResultKind::Pipe(c) => changed.extend(c),
                    StepResultKind::StatementEnd(c) => {
                        changed.extend(c);
                        break Ok(Some(StepResult {
                            statement: step_result.statement,
                            span: self.program.statements()[step_result.statement].steps.span,
                            kind: StepResultKind::StatementEnd(changed),
                        }));
                    }
                    StepResultKind::Breakpoint | StepResultKind::AssertError => {
                        break Ok(Some(step_result));
                    }
                },
                None => break Ok(None),
            }
        }
    }

    pub fn micro_step(&mut self, stop_on_breakpoint: bool) -> Result<Option<StepResult>, Error> {
        match self.micro_step_impl_(stop_on_breakpoint) {
            Ok(Some(step_result)) => {
                if matches!(step_result.kind, StepResultKind::AssertError) {
                    self.cursor = Cursor::Terminated;
                }

                Ok(Some(step_result))
            }
            Ok(None) => {
                self.cursor = Cursor::Terminated;
                Ok(None)
            }
            Err(e) => {
                self.cursor = Cursor::Terminated;
                Err(e)
            }
        }
    }

    // Runs a micro step, but does NOT set cursor to Terminated on end/error/assert_error
    fn micro_step_impl_(&mut self, stop_on_breakpoint: bool) -> Result<Option<StepResult>, Error> {
        loop {
            let is_at_statement_start = self.cursor.is_at_statement_start();

            // Clear intern buses if cursor is at a new statement
            if is_at_statement_start {
                self.state.clear_intern_buses(&mem::take(&mut self.buses_persist));
            }

            // Get cursor live
            let cursor = match &mut self.cursor {
                Cursor::Live(cursor) => cursor,
                Cursor::Terminated => break Ok(None),
            };

            // Get current statement
            let statement = match self.program.statements().get(cursor.statement_idx) {
                Some(statement) => statement,
                None => break Ok(None),
            };

            // Stop on breakpoint
            if is_at_statement_start
                && stop_on_breakpoint
                && !cursor.triggered_breakpoint
                && self.breakpoints.contains(&cursor.statement_idx)
            {
                cursor.triggered_breakpoint = true;
                break (Ok(Some(StepResult {
                    statement: cursor.statement_idx,
                    span: statement.steps.span,
                    kind: StepResultKind::Breakpoint,
                })));
            }

            match cursor.step_idx {
                StepIdx::Step(step_idx) => {
                    // Get current step
                    let (step, _is_pre_pipe) =
                        statement.steps.node.get(step_idx).ok_or_else(|| Error::Other)?;

                    // Execute step
                    let step_result = exec_step(cursor, &self.state, cursor.statement_idx, step)?;

                    // Advance cursor
                    if step_idx == statement.steps.node.as_slice().len() - 1 {
                        cursor.step_idx = StepIdx::Semicolon;
                    } else if step_idx == statement.steps.node.split_at() - 1 {
                        cursor.step_idx = StepIdx::Pipe;
                    } else {
                        cursor.step_idx = StepIdx::Step(step_idx + 1);
                    }

                    // Break, if progress has been made
                    if let Some(step_result) = step_result {
                        break Ok(Some(step_result));
                    }
                }
                StepIdx::Pipe => {
                    // Clock
                    let changed = self.state.clock();

                    // Step result
                    let step_result = StepResult {
                        statement: cursor.statement_idx,
                        span: statement.span_pipe.ok_or_else(|| Error::Other)?,
                        kind: StepResultKind::Pipe(changed),
                    };

                    // Advance cursor
                    cursor.step_idx = StepIdx::Step(statement.steps.node.split_at());

                    break Ok(Some(step_result));
                }
                StepIdx::Semicolon => {
                    // Clock
                    let changed = self.state.clock();

                    // Step result
                    let step_result = StepResult {
                        statement: cursor.statement_idx,
                        span: statement.span_semicolon,
                        kind: StepResultKind::StatementEnd(changed),
                    };

                    // Advance cursor
                    let next_statement_idx = match &cursor.goto {
                        Some(goto_label) => self
                            .program
                            .statements()
                            .iter()
                            .position(|stmt| {
                                stmt.label.as_ref().map(|s| &s.node) == Some(goto_label)
                            })
                            .ok_or(Error::Other)?,
                        None => cursor.statement_idx + 1,
                    };
                    self.cursor = Cursor::new(next_statement_idx);

                    // Finish cycle
                    self.cycle_count += 1;

                    break Ok(Some(step_result));
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Cursor {
    Live(CursorLive),
    Terminated,
}

impl Cursor {
    pub fn new(statement_idx: usize) -> Self {
        Self::Live(CursorLive {
            statement_idx,
            step_idx: StepIdx::Step(0),
            criteria_set: HashSet::new(),
            goto: None,
            triggered_breakpoint: false,
        })
    }

    pub fn is_live(&self) -> bool {
        matches!(self, Cursor::Live(..))
    }

    pub fn is_at_statement_start(&self) -> bool {
        matches!(self, Cursor::Live(CursorLive { step_idx: StepIdx::Step(0), .. }))
    }
}

#[derive(Debug)]
pub struct CursorLive {
    statement_idx: usize,
    step_idx: StepIdx,
    criteria_set: HashSet<CriterionId>,
    goto: Option<Label>,
    triggered_breakpoint: bool,
}

#[derive(Debug)]
enum StepIdx {
    Step(usize),
    Pipe,
    Semicolon,
}

fn criteria_match(criteria: &[Criterion], criteria_set: &HashSet<CriterionId>) -> bool {
    criteria.iter().all(|criterion| match criterion {
        Criterion::True(id) => criteria_set.contains(id),
        Criterion::False(id) => !criteria_set.contains(id),
    })
}

fn exec_step(
    cursor: &mut CursorLive,
    state: &State,
    statement_idx: usize,
    step: &Step,
) -> Result<Option<StepResult>, Error> {
    if criteria_match(&step.criteria, &cursor.criteria_set) {
        let kind = match step.operation.execute(state)? {
            ExecuteResult::Void => StepResultKind::Void,
            ExecuteResult::Criterion(Criterion::True(id), cond_span) => {
                cursor.criteria_set.insert(id);
                StepResultKind::Condition { result: true, span: cond_span }
            }
            ExecuteResult::Criterion(Criterion::False(_), cond_span) => {
                StepResultKind::Condition { result: false, span: cond_span }
            }
            ExecuteResult::Goto(label) => {
                cursor.goto = Some(label);
                StepResultKind::Void
            }
            ExecuteResult::AssertError => StepResultKind::AssertError,
        };
        Ok(Some(StepResult { statement: statement_idx, span: step.span(), kind }))
    } else {
        Ok(None)
    }
}
