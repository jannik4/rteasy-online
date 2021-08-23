use crate::{evaluate::Evaluate, ChangeSet, Error, State};
use rtcore::{
    program::{
        Assignment, ConcatPartLvalueClocked, ConcatPartLvalueUnclocked, CriterionId, EvalCriterion,
        EvalCriterionGroup, Goto, Lvalue, Nop, Operation, OperationKind, Read, Write,
    },
    value::Value,
};

type Result = std::result::Result<Vec<CriterionId>, Error>;

pub trait Execute {
    fn execute(&self, state: &mut State, change_set: &mut ChangeSet) -> Result;
}

impl Execute for Operation {
    fn execute(&self, state: &mut State, change_set: &mut ChangeSet) -> Result {
        match &self.kind {
            OperationKind::EvalCriterion(eval_criterion) => {
                eval_criterion.execute(state, change_set)
            }
            OperationKind::EvalCriterionGroup(eval_criterion_group) => {
                eval_criterion_group.execute(state, change_set)
            }
            OperationKind::Nop(nop) => nop.execute(state, change_set),
            OperationKind::Goto(goto) => goto.execute(state, change_set),
            OperationKind::Write(write) => write.execute(state, change_set),
            OperationKind::Read(read) => read.execute(state, change_set),
            OperationKind::Assignment(assignment) => assignment.execute(state, change_set),
        }
    }
}

impl Execute for EvalCriterion {
    fn execute(&self, state: &mut State, _: &mut ChangeSet) -> Result {
        let cond = self.condition.evaluate(state, 1)?;

        if cond == Value::one(1) {
            Ok(vec![self.criterion_id])
        } else {
            Ok(Vec::new())
        }
    }
}

impl Execute for EvalCriterionGroup {
    fn execute(&self, state: &mut State, change_set: &mut ChangeSet) -> Result {
        let mut criterion_ids = Vec::new();

        for eval_criterion in &self.0 {
            criterion_ids.extend(eval_criterion.execute(state, change_set)?);
        }

        Ok(criterion_ids)
    }
}

impl Execute for Nop {
    fn execute(&self, _: &mut State, _: &mut ChangeSet) -> Result {
        Ok(Vec::new())
    }
}

impl Execute for Goto {
    fn execute(&self, _: &mut State, change_set: &mut ChangeSet) -> Result {
        change_set.goto(self.label.clone())?;
        Ok(Vec::new())
    }
}

impl Execute for Write {
    fn execute(&self, _: &mut State, change_set: &mut ChangeSet) -> Result {
        change_set.write_memory(self.ident.clone())?;
        Ok(Vec::new())
    }
}

impl Execute for Read {
    fn execute(&self, state: &mut State, change_set: &mut ChangeSet) -> Result {
        state.read_memory(&self.ident, change_set)?;
        Ok(Vec::new())
    }
}

impl Execute for Assignment {
    fn execute(&self, state: &mut State, change_set: &mut ChangeSet) -> Result {
        let value = self.rhs.evaluate(&state, self.size)?;

        match &self.lhs {
            Lvalue::Register(reg) => {
                change_set.write_into_register(reg.clone(), value)?;
            }
            Lvalue::Bus(bus) => state.write_into_bus(bus, value)?,
            Lvalue::RegisterArray(register_array) => {
                let idx = register_array.index.evaluate(&state, register_array.index_ctx_size)?;
                change_set.write_into_register_array(register_array.ident.clone(), idx, value)?;
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
                            change_set.write_into_register(reg.clone(), value)?;
                        }
                        ConcatPartLvalueClocked::RegisterArray(reg_array, _) => {
                            let idx = reg_array.index.evaluate(&state, reg_array.index_ctx_size)?;
                            change_set.write_into_register_array(
                                reg_array.ident.clone(),
                                idx,
                                value,
                            )?;
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
                    state.write_into_bus(bus, value)?;

                    start += size;
                }
            }
        }

        Ok(Vec::new())
    }
}
