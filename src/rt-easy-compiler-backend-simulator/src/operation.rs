use crate::{Generate, Result};
use compiler::mir;
use rtcore::program::*;

impl Generate<mir::Operation<'_>> for Operation {
    fn generate(operation: mir::Operation<'_>) -> Result<Self> {
        let kind = match operation.kind {
            mir::OperationKind::EvalCriterion(eval_criterion) => {
                OperationKind::EvalCriterion(Generate::generate(eval_criterion)?)
            }
            mir::OperationKind::EvalCriterionSwitchGroup(eval_criterion_group) => {
                OperationKind::EvalCriterionGroup(Generate::generate(eval_criterion_group)?)
            }
            mir::OperationKind::Nop(mir::Nop) => OperationKind::Nop(Nop),
            mir::OperationKind::Goto(goto) => OperationKind::Goto(Generate::generate(goto)?),
            mir::OperationKind::Write(write) => OperationKind::Write(Generate::generate(write)?),
            mir::OperationKind::Read(read) => OperationKind::Read(Generate::generate(read)?),
            mir::OperationKind::Assignment(assignment) => {
                OperationKind::Assignment(Generate::generate(assignment)?)
            }
        };

        Ok(Operation { kind, span: operation.span.clone() })
    }
}

impl Generate<mir::EvalCriterion<'_>> for EvalCriterion {
    fn generate(eval_criterion: mir::EvalCriterion<'_>) -> Result<Self> {
        Ok(EvalCriterion {
            criterion_id: Generate::generate(eval_criterion.criterion_id)?,
            condition: Generate::generate(eval_criterion.condition)?,
        })
    }
}

impl Generate<mir::EvalCriterionSwitchGroup<'_>> for EvalCriterionGroup {
    fn generate(eval_criterion_group: mir::EvalCriterionSwitchGroup<'_>) -> Result<Self> {
        Ok(EvalCriterionGroup(Generate::generate(eval_criterion_group.0)?))
    }
}

impl Generate<mir::Goto<'_>> for Goto {
    fn generate(goto: mir::Goto<'_>) -> Result<Self> {
        Ok(Goto { label: goto.label.into() })
    }
}

impl Generate<mir::Write<'_>> for Write {
    fn generate(write: mir::Write<'_>) -> Result<Self> {
        Ok(Write { ident: write.ident.into() })
    }
}

impl Generate<mir::Read<'_>> for Read {
    fn generate(read: mir::Read<'_>) -> Result<Self> {
        Ok(Read { ident: read.ident.into() })
    }
}

impl Generate<mir::Assignment<'_>> for Assignment {
    fn generate(assignment: mir::Assignment<'_>) -> Result<Self> {
        Ok(Assignment {
            lhs: Generate::generate(assignment.lhs)?,
            rhs: Generate::generate(assignment.rhs)?,
            size: assignment.size,
        })
    }
}

impl Generate<mir::Lvalue<'_>> for Lvalue {
    fn generate(lvalue: mir::Lvalue<'_>) -> Result<Self> {
        match lvalue {
            mir::Lvalue::Register(reg) => Ok(Lvalue::Register(Generate::generate(reg)?)),
            mir::Lvalue::Bus(bus) => Ok(Lvalue::Bus(Generate::generate(bus)?)),
            mir::Lvalue::RegisterArray(reg_array) => {
                Ok(Lvalue::RegisterArray(Generate::generate(reg_array)?))
            }
            mir::Lvalue::ConcatClocked(concat) => {
                Ok(Lvalue::ConcatClocked(Generate::generate(concat)?))
            }
            mir::Lvalue::ConcatUnclocked(concat) => {
                Ok(Lvalue::ConcatUnclocked(Generate::generate(concat)?))
            }
        }
    }
}
