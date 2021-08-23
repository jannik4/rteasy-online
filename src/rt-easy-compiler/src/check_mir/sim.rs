use crate::mir::*;
use crate::{CompilerError, InternalError};
use std::collections::HashSet;

pub type Result = std::result::Result<(), InternalError>;

pub trait SimState<'s> {
    fn condition(&mut self, condition: &Expression<'s>, span: Range<usize>) -> Result;
    fn nop(&mut self, nop: &Nop, span: Range<usize>) -> Result;
    fn goto(&mut self, goto: &Goto<'s>, span: Range<usize>) -> Result;
    fn write(&mut self, write: &Write<'s>, span: Range<usize>) -> Result;
    fn read(&mut self, read: &Read<'s>, span: Range<usize>) -> Result;
    fn assignment(&mut self, assignment: &Assignment<'s>, span: Range<usize>) -> Result;

    fn finish(self, error_sink: &mut impl FnMut(CompilerError));
}

pub fn sim<'s, S>(
    statement: &Statement<'s>,
    state: S,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result
where
    S: SimState<'s> + Clone,
{
    sim_(&statement.steps, &HashSet::new(), state, error_sink)
}

fn sim_<'s, S>(
    steps: &[Step<'s>],
    criteria_set: &HashSet<CriterionId>,
    mut state: S,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result
where
    S: SimState<'s> + Clone,
{
    match steps.split_first() {
        Some((step, steps)) => {
            // Sim step
            if criteria_match(&step.criteria, &criteria_set) {
                let span = step.operation.span.clone();
                match &step.operation.kind {
                    OperationKind::EvalCriterion(eval_criterion) => {
                        state.condition(&eval_criterion.condition, span)?;

                        // Sim remaining steps with criterion set
                        {
                            let mut criteria_set = criteria_set.clone();
                            criteria_set.insert(eval_criterion.criterion_id);
                            let state = state.clone();
                            sim_(steps, &criteria_set, state, error_sink)?;
                        }
                    }
                    OperationKind::EvalCriterionGroup(eval_criterion_group) => {
                        for eval_criterion in &eval_criterion_group.0 {
                            state.condition(&eval_criterion.condition, span.clone())?;
                        }

                        // Sim remaining steps with exactly one criterion set
                        for eval_criterion in &eval_criterion_group.0 {
                            let mut criteria_set = criteria_set.clone();
                            criteria_set.insert(eval_criterion.criterion_id);
                            let state = state.clone();
                            sim_(steps, &criteria_set, state, error_sink)?;
                        }
                    }
                    OperationKind::Nop(nop) => state.nop(nop, span)?,
                    OperationKind::Goto(goto) => state.goto(goto, span)?,
                    OperationKind::Write(write) => state.write(write, span)?,
                    OperationKind::Read(read) => state.read(read, span)?,
                    OperationKind::Assignment(assignment) => state.assignment(assignment, span)?,
                }
            }

            // Sim remaining steps
            sim_(steps, criteria_set, state, error_sink)?;
        }
        None => {
            // Finish state
            state.finish(error_sink);
        }
    }

    Ok(())
}

fn criteria_match(criteria: &[Criterion], criteria_set: &HashSet<CriterionId>) -> bool {
    criteria.iter().all(|criterion| match criterion {
        Criterion::True(id) => criteria_set.contains(id),
        Criterion::False(id) => !criteria_set.contains(id),
    })
}
