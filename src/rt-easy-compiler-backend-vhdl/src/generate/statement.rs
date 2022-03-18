use super::{
    expression::generate_expression,
    operation::{generate_assignment, generate_read, generate_write},
    util::CriteriaMapping,
    GenNextStateLogic, GenStatement,
};
use crate::vhdl::*;
use compiler::mir;
use indexmap::{IndexMap, IndexSet};
use vec1::{vec1, Vec1};

pub(super) fn generate_statement<'s>(
    idx: usize,
    mir_statement: &mir::Statement<'s>,
    next_mir_statement: Option<&mir::Statement<'s>>,
    criteria: &mut IndexSet<Expression>,
    operations: &mut IndexSet<Operation>,
    declarations: &Declarations,
) -> GenStatement {
    // Create gen statement
    let mut statement = GenStatement {
        label: match mir_statement.label.as_ref() {
            Some(label) => Label::named(label.node.0),
            None => Label::unnamed(idx),
        },
        operations: IndexMap::new(),
        next_state_logic: GenNextStateLogic {
            conditional: Vec::new(),
            default: match next_mir_statement {
                Some(next_mir_statement) => match next_mir_statement.label.as_ref() {
                    Some(label) => Label::named(label.node.0),
                    None => Label::unnamed(idx + 1),
                },
                None => Label::terminated(),
            },
        },

        has_pipe: mir_statement.steps.node.iter().any(|step| step.annotation.is_post_pipe),
        criteria_mapping: CriteriaMapping::new(),
    };

    // Add steps
    for mir_step in &mir_statement.steps.node {
        add_step(
            &mut statement,
            mir_step,
            declarations,
            |cond| CriterionId(criteria.insert_full(cond).0),
            |op| OperationId(operations.insert_full(op).0),
        );
    }

    // Sort operations by operation id ASC
    statement.operations.sort_keys();

    statement
}

fn add_step(
    statement: &mut GenStatement,
    mir_step: &mir::Step<'_>,
    declarations: &Declarations,

    mut get_criterion_id: impl FnMut(Expression) -> CriterionId,
    mut get_operation_id: impl FnMut(Operation) -> OperationId,
) {
    match &mir_step.operation {
        // MIR criteria are inserted in the global VHDL criteria set.
        // In addition, an entry is created in the mapping.
        mir::Operation::EvalCriterion(mir_eval_criterion) => {
            let condition = generate_expression(&mir_eval_criterion.condition, declarations, 1);
            statement
                .criteria_mapping
                .insert(mir_eval_criterion.criterion_id, get_criterion_id(condition));
        }
        mir::Operation::EvalCriterionSwitchGroup(group) => {
            for mir_eval_criterion in &group.eval_criteria {
                let condition = generate_expression(&mir_eval_criterion.condition, declarations, 1);
                statement
                    .criteria_mapping
                    .insert(mir_eval_criterion.criterion_id, get_criterion_id(condition));
            }
        }

        // If the step has no criteria, the default next state is overwritten.
        // Otherwise the entry for the label in next_state_conditional is created
        // or extended with the criteria of this step.
        mir::Operation::Goto(mir_goto) => {
            let label = Label::named(mir_goto.label.node.0);

            if mir_step.criteria.is_empty() {
                statement.next_state_logic.default = label;
            } else {
                let and = And(
                    Vec1::try_from(statement.criteria_mapping.map(&mir_step.criteria)).unwrap()
                );
                let entry =
                    statement.next_state_logic.conditional.iter_mut().find(|(_, l)| l == &label);
                match entry {
                    Some((criteria, _)) => criteria.0.push(and),
                    None => {
                        statement.next_state_logic.conditional.push((Or(vec1![and]), label));
                    }
                }
            }
        }

        // First, map the operation, insert it into the global VHDL operations set and get the id.
        // Then upsert the operation id into the statement operations and update the criteria.
        mir::Operation::Write(mir_write) => {
            let operation = Operation::Write(generate_write(mir_write, declarations));
            let operation_id = get_operation_id(operation);
            add_op(statement, operation_id, &mir_step.criteria);
        }
        mir::Operation::Read(mir_read) => {
            let operation = Operation::Read(generate_read(mir_read, declarations));
            let operation_id = get_operation_id(operation);
            add_op(statement, operation_id, &mir_step.criteria);
        }
        mir::Operation::Assignment(mir_assignment) => {
            let operation =
                Operation::Assignment(generate_assignment(mir_assignment, declarations));
            let operation_id = get_operation_id(operation);
            add_op(statement, operation_id, &mir_step.criteria);
        }

        // Ignore nop and assert
        mir::Operation::Nop(_) => (),
        mir::Operation::Assert(_) => (),
    }
}

fn add_op(
    statement: &mut GenStatement,
    operation_id: OperationId,
    mir_criteria: &[mir::Criterion],
) {
    if mir_criteria.is_empty() {
        let old = statement.operations.insert(operation_id, None);

        // If an operation has no criteria, it is always executed.
        // There should be no identical operation, otherwise it would possibly
        // be executed twice in one cycle.
        assert!(old.is_none());
    } else {
        let and = And(Vec1::try_from(statement.criteria_mapping.map(mir_criteria)).unwrap());
        match statement.operations.get_mut(&operation_id) {
            Some(Some(criteria)) => criteria.0.push(and),
            Some(None) => unreachable!(), // This should be unreachable for the same reason see above.
            None => {
                statement.operations.insert(operation_id, Some(Or(vec1![and])));
            }
        }
    }
}
