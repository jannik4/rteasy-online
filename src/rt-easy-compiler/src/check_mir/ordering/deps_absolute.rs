use crate::mir::*;
use std::collections::HashSet;

pub fn calc_absolute_dependencies(
    step: &Step<'_>,
    all_steps: &[Step<'_>],
) -> Result<HashSet<StepId>, HashSet<StepId>> {
    // Start from current dependencies
    let mut absolute_deps = step.annotation.dependencies.clone();

    loop {
        let current_len = absolute_deps.len();

        // Add all dependencies of current dependencies
        let dep_steps =
            all_steps.iter().filter(|step| absolute_deps.contains(&step.id)).collect::<Vec<_>>();
        for dep_step in dep_steps {
            absolute_deps.extend(dep_step.annotation.dependencies.iter());
        }

        // Stop if no new dependencies were added
        if current_len == absolute_deps.len() {
            break;
        }
    }

    // Feeback loop if absolute_deps contains self
    if absolute_deps.contains(&step.id) {
        return Err(absolute_deps);
    }

    Ok(absolute_deps)
}
