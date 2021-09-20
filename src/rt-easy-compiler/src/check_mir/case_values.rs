use super::const_eval::Evaluate;
use crate::mir::*;
use crate::{CompilerError, CompilerErrorKind, InternalError};
use std::collections::HashSet;

pub type Result = std::result::Result<(), InternalError>;

pub fn check(mir: &Mir<'_>, error_sink: &mut impl FnMut(CompilerError)) -> Result {
    for statement in &mir.statements {
        for step in &statement.steps.node {
            if let Operation::EvalCriterionSwitchGroup(group) = &step.operation {
                let mut values = HashSet::new();

                for eval_criterion in &group.eval_criteria {
                    let case_value = match &eval_criterion.condition {
                        Expression::BinaryTerm(term)
                            if term.operator.node == BinaryOperator::Eq =>
                        {
                            term.rhs.evaluate(group.switch_expression_size).ok_or_else(|| {
                                InternalError("could not evaluate const expr".into())
                            })?
                        }
                        _ => return Err(InternalError("expected eq term".into())),
                    };

                    let inserted = values.insert(case_value);
                    if !inserted {
                        error_sink(CompilerError::new(
                            CompilerErrorKind::DuplicateCaseValue,
                            eval_criterion.span(),
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}
