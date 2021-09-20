use crate::mir::*;
use crate::{CompilerError, InternalError};
use std::collections::HashSet;

pub type Result = std::result::Result<(), InternalError>;

pub trait SimState<'s> {
    fn condition(&mut self, condition: &Expression<'s>) -> Result;
    fn nop(&mut self, nop: &Nop) -> Result;
    fn goto(&mut self, goto: &Goto<'s>) -> Result;
    fn write(&mut self, write: &Write<'s>) -> Result;
    fn read(&mut self, read: &Read<'s>) -> Result;
    fn assignment(&mut self, assignment: &Assignment<'s>) -> Result;
    fn assert(&mut self, assert: &Assert<'s>) -> Result;

    fn finish(self, statement: &Statement<'s>, error_sink: &mut impl FnMut(CompilerError));
}

pub fn sim<'s, S>(
    statement: &Statement<'s>,
    state: S,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result
where
    S: SimState<'s> + Clone,
{
    sim_(statement, &statement.steps.node, &HashSet::new(), state, error_sink)
}

fn sim_<'s, S>(
    statement: &Statement<'s>,
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
                match &step.operation {
                    Operation::EvalCriterion(eval_criterion) => {
                        state.condition(&eval_criterion.condition)?;

                        // Sim remaining steps with criterion set
                        {
                            let mut criteria_set = criteria_set.clone();
                            criteria_set.insert(eval_criterion.criterion_id);
                            let state = state.clone();
                            sim_(statement, steps, &criteria_set, state, error_sink)?;
                        }
                    }
                    Operation::EvalCriterionSwitchGroup(eval_criterion_group) => {
                        for eval_criterion in &eval_criterion_group.eval_criteria {
                            state.condition(&eval_criterion.condition)?;
                        }

                        // Sim remaining steps with exactly one criterion set
                        for eval_criterion in &eval_criterion_group.eval_criteria {
                            let mut criteria_set = criteria_set.clone();
                            criteria_set.insert(eval_criterion.criterion_id);
                            let state = state.clone();
                            sim_(statement, steps, &criteria_set, state, error_sink)?;
                        }
                    }
                    Operation::Nop(nop) => state.nop(nop)?,
                    Operation::Goto(goto) => state.goto(goto)?,
                    Operation::Write(write) => state.write(write)?,
                    Operation::Read(read) => state.read(read)?,
                    Operation::Assignment(assignment) => state.assignment(assignment)?,
                    Operation::Assert(assert) => state.assert(assert)?,
                }
            }

            // Sim remaining steps
            sim_(statement, steps, criteria_set, state, error_sink)?;
        }
        None => {
            // Finish state
            state.finish(statement, error_sink);
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
