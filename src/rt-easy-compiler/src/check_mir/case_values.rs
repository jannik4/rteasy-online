use super::const_eval::Evaluate;
use crate::mir::*;
use crate::{CompilerError, InternalError};
use std::collections::HashSet;

pub type Result = std::result::Result<(), InternalError>;

pub fn check(mir: &Mir<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Result {
    for statement in &mir.statements {
        for step in &statement.steps {
            if let OperationKind::EvalCriterionSwitchGroup(group) = &step.operation.kind {
                let mut values = HashSet::new();

                for eval_criterion in &group.0 {
                    let case_value = match &eval_criterion.condition {
                        Expression::BinaryTerm(term) if term.operator == BinaryOperator::Eq => term
                            .rhs
                            .evaluate(group.1)
                            .ok_or_else(|| InternalError("could not evaluate const expr".into()))?,
                        _ => return Err(InternalError("expected eq term".into())),
                    };

                    let inserted = values.insert(case_value);
                    if !inserted {
                        error_sink(CompilerError::DuplicateCaseValue);
                    }
                }
            }
        }
    }

    Ok(())
}
