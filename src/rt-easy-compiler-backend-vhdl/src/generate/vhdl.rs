use super::{expression::generate_expression, operation::generate_assignment};
use crate::vhdl::*;
use compiler::mir;
use indexmap::{IndexMap, IndexSet};
use std::collections::HashMap;
use vec1::{vec1, Vec1};

pub fn generate_vhdl<'s>(mir: mir::Mir<'s>, module_name: String) -> Vhdl<'s> {
    // Create vhdl
    let mut vhdl = Vhdl {
        module_name,
        statements: Vec::new(),
        criteria: IndexSet::new(),
        operations: IndexSet::new(),

        declarations: mir.declarations,
    };

    // Generate statements
    for (idx, mir_statement) in mir.statements.iter().enumerate() {
        generate_statement(idx, mir_statement, mir.statements.get(idx + 1), &mut vhdl);
    }

    // Loop statement
    vhdl.statements.push(Statement {
        label: Label::End,
        next_state_conditional: IndexMap::new(),
        next_state_default: Label::End,
        operations: IndexMap::new(),
    });

    vhdl
}

pub fn generate_statement<'s>(
    idx: usize,
    mir_statement: &mir::Statement<'s>,
    next_mir_statement: Option<&mir::Statement<'s>>,
    vhdl: &mut Vhdl<'s>,
) {
    // Create statement
    let mut statement = Statement {
        label: match mir_statement.label.as_ref() {
            Some(label) => Label::Named(label.node),
            None => Label::Unnamed(idx),
        },

        next_state_conditional: IndexMap::new(),
        next_state_default: match next_mir_statement {
            Some(next_mir_statement) => match next_mir_statement.label.as_ref() {
                Some(label) => Label::Named(label.node),
                None => Label::Unnamed(idx + 1),
            },
            None => Label::End,
        },
        operations: IndexMap::new(),
    };

    // Since all criteria are combined in the VHDL code,
    // the MIR criteria IDs must be mapped to the new ones.
    let mut criteria_mapping: HashMap<mir::CriterionId, CriterionId> = HashMap::new();

    // Map steps
    for mir_step in &mir_statement.steps.node {
        match &mir_step.operation {
            // MIR criteria are inserted in the global VHDL criteria set.
            // In addition, an entry is created in the mapping.
            mir::Operation::EvalCriterion(mir_eval_criterion) => {
                let condition = generate_expression(&mir_eval_criterion.condition);
                let (criterion_id, _) = vhdl.criteria.insert_full(condition);
                criteria_mapping.insert(mir_eval_criterion.criterion_id, CriterionId(criterion_id));
            }
            mir::Operation::EvalCriterionSwitchGroup(_) => todo!(),

            // If the step has no criteria, the default next state is overwritten.
            // Otherwise the entry for the label in next_state_conditional is created
            // or extended with the criteria of this step.
            mir::Operation::Goto(mir_goto) => {
                let label = Label::Named(mir_goto.label.node);

                if mir_step.criteria.is_empty() {
                    statement.next_state_default = label;
                } else {
                    let and = And(map_criteria(&mir_step.criteria, &criteria_mapping));
                    match statement.next_state_conditional.get_mut(&label) {
                        Some(criteria) => criteria.0.push(and),
                        None => {
                            statement.next_state_conditional.insert(label, Or(vec1![and]));
                        }
                    }
                }
            }

            // First, map the operation, insert it into the global VHDL operations set and get the id.
            // Then upsert the operation id into the statement operations and update the criteria.
            mir::Operation::Write(_) => todo!(),
            mir::Operation::Read(_) => todo!(),
            mir::Operation::Assignment(mir_assignment) => {
                let assignment = generate_assignment(mir_assignment);
                let (operation_id, _) =
                    vhdl.operations.insert_full(Operation::Assignment(assignment));
                let operation_id = OperationId(operation_id);

                if mir_step.criteria.is_empty() {
                    let old = statement.operations.insert(operation_id, None);

                    // If an operation has no criteria, it is always executed.
                    // There should be no identical operation, otherwise it would possibly
                    // be executed twice in one cycle.
                    assert!(old.is_none());
                } else {
                    let and = And(map_criteria(&mir_step.criteria, &criteria_mapping));
                    match statement.operations.get_mut(&operation_id) {
                        Some(Some(criteria)) => criteria.0.push(and),
                        Some(None) => unreachable!(), // This should be unreachable for the same reason see above.
                        None => {
                            statement.operations.insert(operation_id, Some(Or(vec1![and])));
                        }
                    }
                }
            }

            // Ignore nop and assert
            mir::Operation::Nop(_) => (),
            mir::Operation::Assert(_) => (),
        }
    }

    // Sort operations by operation id ASC
    statement.operations.sort_keys();

    // Push statement
    vhdl.statements.push(statement);
}

/// `mir_criteria` must not be empty.
/// All `mir_criteria` must be in `criteria_mapping`.
///
/// # Panics
///
/// Panics if `mir_criteria` is empty or any `mir_criteria` is not in `criteria_mapping`.
fn map_criteria(
    mir_criteria: &[mir::Criterion],
    criteria_mapping: &HashMap<mir::CriterionId, CriterionId>,
) -> Vec1<Criterion> {
    Vec1::try_from_vec(
        mir_criteria
            .iter()
            .map(|criterion| match criterion {
                mir::Criterion::True(id) => Criterion::True(*criteria_mapping.get(id).unwrap()),
                mir::Criterion::False(id) => Criterion::False(*criteria_mapping.get(id).unwrap()),
            })
            .collect(),
    )
    .unwrap()
}
