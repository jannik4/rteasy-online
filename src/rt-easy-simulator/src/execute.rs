use crate::{evaluate::Evaluate, state::State, Error};
use rtcore::{
    program::{
        Assert, Assignment, ConcatPartLvalueClocked, ConcatPartLvalueUnclocked, Criterion,
        EvalCriterion, EvalCriterionGroup, Goto, Label, Lvalue, Nop, Operation, OperationKind,
        Read, Span, Write,
    },
    value::Value,
};

type Result = std::result::Result<ExecuteResult, Error>;

#[derive(Debug)]
pub enum ExecuteResult {
    Void,
    Criterion(Criterion, Span),
    Goto(Label),
    AssertError,
}

pub trait Execute {
    fn execute(&self, state: &State) -> Result;
}

impl Execute for Operation {
    fn execute(&self, state: &State) -> Result {
        match &self.kind {
            OperationKind::EvalCriterion(eval_criterion) => eval_criterion.execute(state),
            OperationKind::EvalCriterionGroup(eval_criterion_group) => {
                eval_criterion_group.execute(state)
            }
            OperationKind::Nop(nop) => nop.execute(state),
            OperationKind::Goto(goto) => goto.execute(state),
            OperationKind::Write(write) => write.execute(state),
            OperationKind::Read(read) => read.execute(state),
            OperationKind::Assignment(assignment) => assignment.execute(state),
            OperationKind::Assert(assert) => assert.execute(state),
        }
    }
}

impl Execute for EvalCriterion {
    fn execute(&self, state: &State) -> Result {
        let cond = self.condition.evaluate(state, 1)?;

        if cond == Value::one(1) {
            Ok(ExecuteResult::Criterion(Criterion::True(self.criterion_id), self.condition.span))
        } else {
            Ok(ExecuteResult::Criterion(Criterion::False(self.criterion_id), self.condition.span))
        }
    }
}

impl Execute for EvalCriterionGroup {
    fn execute(&self, state: &State) -> Result {
        for eval_criterion in &self.0 {
            match eval_criterion.execute(state)? {
                r @ ExecuteResult::Criterion(Criterion::True(_), _) => return Ok(r),
                _ => (),
            }
        }

        // No criterion was true
        Ok(ExecuteResult::Void)
    }
}

impl Execute for Nop {
    fn execute(&self, _: &State) -> Result {
        Ok(ExecuteResult::Void)
    }
}

impl Execute for Goto {
    fn execute(&self, _: &State) -> Result {
        Ok(ExecuteResult::Goto(self.label.clone()))
    }
}

impl Execute for Write {
    fn execute(&self, state: &State) -> Result {
        state.memory(&self.ident)?.write(state)?;
        Ok(ExecuteResult::Void)
    }
}

impl Execute for Read {
    fn execute(&self, state: &State) -> Result {
        state.memory(&self.ident)?.read(state)?;
        Ok(ExecuteResult::Void)
    }
}

impl Execute for Assignment {
    fn execute(&self, state: &State) -> Result {
        let value = self.rhs.evaluate(&state, self.size)?;

        match &self.lhs {
            Lvalue::Register(reg) => {
                state.register(&reg.ident)?.write(reg.range, value)?;
            }
            Lvalue::Bus(bus) => state.bus(&bus.ident)?.write(bus.range, value)?,
            Lvalue::RegisterArray(register_array) => {
                let idx = register_array.index.evaluate(&state, register_array.index_ctx_size)?;
                state.register_array(&register_array.ident)?.write(idx, value)?;
            }
            Lvalue::ConcatClocked(lhs) => {
                let mut start = 0;
                for part in lhs.parts.iter().rev() {
                    let size = match part {
                        ConcatPartLvalueClocked::Register(_, size) => *size,
                        ConcatPartLvalueClocked::RegisterArray(_, size) => *size,
                    };

                    let value = value[start..start + size].to_owned();
                    match part {
                        ConcatPartLvalueClocked::Register(reg, _) => {
                            state.register(&reg.ident)?.write(reg.range, value)?;
                        }
                        ConcatPartLvalueClocked::RegisterArray(reg_array, _) => {
                            let idx = reg_array.index.evaluate(&state, reg_array.index_ctx_size)?;
                            state.register_array(&reg_array.ident)?.write(idx, value)?;
                        }
                    }

                    start += size;
                }
            }
            Lvalue::ConcatUnclocked(lhs) => {
                let mut start = 0;
                for part in lhs.parts.iter().rev() {
                    let ConcatPartLvalueUnclocked::Bus(bus, size) = part;

                    let value = value[start..start + size].to_owned();
                    state.bus(&bus.ident)?.write(bus.range, value)?;

                    start += size;
                }
            }
        }

        Ok(ExecuteResult::Void)
    }
}

impl Execute for Assert {
    fn execute(&self, state: &State) -> Result {
        let cond = self.condition.evaluate(state, 1)?;

        if cond == Value::one(1) {
            Ok(ExecuteResult::Void)
        } else {
            Ok(ExecuteResult::AssertError)
        }
    }
}
