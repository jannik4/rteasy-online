mod deps_absolute;
mod deps_direct;

use crate::mir::*;
use crate::{CompilerError, CompilerErrorKind, InternalError};

pub fn check_and_order(
    mir: &mut Mir<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result<(), InternalError> {
    for statement in &mut mir.statements {
        check_and_order_(statement, error_sink)?;
    }

    Ok(())
}

fn check_and_order_(
    statement: &mut Statement<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result<(), InternalError> {
    let steps = statement.steps.node.as_mut_slice();

    // Calc direct dependencies
    for i in 0..steps.len() {
        steps[i].annotation.dependencies =
            deps_direct::calc_direct_dependencies(&steps[i], &*steps);
    }

    // Calc absolute dependencies
    let mut has_feedback_loop = false;
    for i in 0..steps.len() {
        let res = deps_absolute::calc_absolute_dependencies(&steps[i], &*steps);
        steps[i].annotation.dependencies = match res {
            Ok(deps) => deps,
            Err(_feedback_loop) => {
                // Set has_feedback_loop
                has_feedback_loop = true;
                error_sink(CompilerError::new(
                    CompilerErrorKind::FeedbackLoop,
                    statement.steps.span,
                ));

                // Continue with next step to get all errors
                continue;
            }
        };
    }

    // Sort steps if no feeback loop was found
    if !has_feedback_loop {
        steps.sort_by(|a, b| {
            use std::cmp::Ordering;

            // If a is dependent on b => put b first
            if a.annotation.dependencies.contains(&b.id) {
                return Ordering::Greater;
            }

            // If b is dependent on a => put a first
            if b.annotation.dependencies.contains(&a.id) {
                return Ordering::Less;
            }

            // Otherwise put unclocked assigns before other steps
            // (sort is stable, so all other steps do not get reordered)
            match (a.annotation.is_unclocked_assign, b.annotation.is_unclocked_assign) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });
    }

    Ok(())
}
