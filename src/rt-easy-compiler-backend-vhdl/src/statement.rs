use super::{
    expression::generate_expression,
    operation::{generate_assignment, generate_read, generate_write},
    vhdl::VhdlBuilder,
};
use compiler::mir;
use rtvhdl::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct StatementBuilder {
    label: Label,
    operations: IndexMap<OperationId, Option<Or<And<Criterion>>>>,
    next_state_conditional: Vec<(Or<And<Criterion>>, Label)>,
    next_state_default: Label,

    transform: bool,
    criteria_mapping: CriteriaMapping,
}

impl StatementBuilder {
    pub fn build(
        label: Label,
        label_next: Label,
        steps: &[mir::Step<'_>],
        transform: bool,
        vhdl_builder: &mut VhdlBuilder,
    ) {
        // Create builder
        let mut builder = Self {
            label,
            operations: IndexMap::new(),
            next_state_conditional: Vec::new(),
            next_state_default: label_next,

            transform,
            criteria_mapping: CriteriaMapping::new(),
        };

        // Add steps
        for step in steps {
            builder.add_step(step, vhdl_builder);
        }

        // Sort operations by operation id ASC
        builder.operations.sort_keys();

        // Finish
        builder.finish(vhdl_builder)
    }

    fn add_step(&mut self, step: &mir::Step<'_>, vhdl_builder: &mut VhdlBuilder) {
        match &step.operation {
            // MIR criteria are inserted in the global VHDL criteria set.
            // In addition, an entry is created in the mapping.
            mir::Operation::EvalCriterion(mir_eval_criterion) => {
                let condition = generate_expression(
                    &mir_eval_criterion.condition,
                    &vhdl_builder.declarations(),
                    1,
                );
                let criterion_id = vhdl_builder.insert_criterion(condition);
                self.criteria_mapping.insert(mir_eval_criterion.criterion_id, criterion_id);
            }
            mir::Operation::EvalCriterionSwitchGroup(group) => {
                for mir_eval_criterion in &group.eval_criteria {
                    let condition = generate_expression(
                        &mir_eval_criterion.condition,
                        &vhdl_builder.declarations(),
                        1,
                    );
                    let criterion_id = vhdl_builder.insert_criterion(condition);
                    self.criteria_mapping.insert(mir_eval_criterion.criterion_id, criterion_id);
                }
            }

            // If the step has no criteria, the default next state is overwritten.
            // Otherwise the entry for the label in next_state_conditional is created
            // or extended with the criteria of this step.
            mir::Operation::Goto(mir_goto) => {
                let label = Label::named(mir_goto.label.node.0);

                if step.criteria.is_empty() {
                    self.next_state_default = label;
                } else {
                    let and =
                        And(Vec1::try_from(self.criteria_mapping.map(&step.criteria)).unwrap());
                    let entry = self.next_state_conditional.iter_mut().find(|(_, l)| l == &label);
                    match entry {
                        Some((criteria, _)) => criteria.0.push(and),
                        None => {
                            self.next_state_conditional.push((Or(vec1![and]), label));
                        }
                    }
                }
            }

            // First, map the operation, insert it into the global VHDL operations set and get the id.
            // Then upsert the operation id into the statement operations and update the criteria.
            mir::Operation::Write(mir_write) => {
                let operation =
                    Operation::Write(generate_write(mir_write, &vhdl_builder.declarations()));
                let operation_id = vhdl_builder.insert_operation(operation);
                self.add_op(operation_id, &step.criteria);
            }
            mir::Operation::Read(mir_read) => {
                let operation =
                    Operation::Read(generate_read(mir_read, &vhdl_builder.declarations()));
                let operation_id = vhdl_builder.insert_operation(operation);
                self.add_op(operation_id, &step.criteria);
            }
            mir::Operation::Assignment(mir_assignment) => {
                let operation = Operation::Assignment(generate_assignment(
                    mir_assignment,
                    &vhdl_builder.declarations(),
                ));
                let operation_id = vhdl_builder.insert_operation(operation);
                self.add_op(operation_id, &step.criteria);
            }

            // Ignore nop and assert
            mir::Operation::Nop(_) => (),
            mir::Operation::Assert(_) => (),
        }
    }

    fn add_op(&mut self, operation_id: OperationId, mir_criteria: &[mir::Criterion]) {
        if mir_criteria.is_empty() {
            let old = self.operations.insert(operation_id, None);

            // If an operation has no criteria, it is always executed.
            // There should be no identical operation, otherwise it would possibly
            // be executed twice in one cycle.
            assert!(old.is_none());
        } else {
            let and = And(Vec1::try_from(self.criteria_mapping.map(mir_criteria)).unwrap());
            match self.operations.get_mut(&operation_id) {
                Some(Some(criteria)) => criteria.0.push(and),
                Some(None) => unreachable!(), // This should be unreachable for the same reason see above.
                None => {
                    self.operations.insert(operation_id, Some(Or(vec1![and])));
                }
            }
        }
    }

    fn finish(self, vhdl_builder: &mut VhdlBuilder) {
        // Push if no transform is needed
        if !self.transform {
            vhdl_builder.push_statement(Statement {
                label: self.label,
                operations: self.operations,
                next_state_logic: build_logic(self.next_state_conditional, self.next_state_default),
            });
            return;
        }

        // Transform
        {
            // Conditional
            let mut transform_to_conditional = Vec::new();
            for (cond, cond_label) in self.next_state_conditional {
                let transform_to_label = Label(format!(
                    "{}{}{}",
                    self.label,
                    vhdl_builder.transform_goto_prefix(),
                    cond_label
                ));
                transform_to_conditional.push((cond, transform_to_label.clone()));
                vhdl_builder.push_statement(Statement {
                    label: transform_to_label,
                    operations: self.operations.clone(),
                    next_state_logic: NextStateLogic::Label(cond_label),
                });
            }

            // Default
            let transform_to_default = Label(format!(
                "{}{}{}",
                self.label,
                vhdl_builder.transform_goto_prefix(),
                self.next_state_default
            ));
            vhdl_builder.push_statement(Statement {
                label: transform_to_default.clone(),
                operations: self.operations,
                next_state_logic: NextStateLogic::Label(self.next_state_default),
            });

            // Insert transform
            vhdl_builder.insert_transform(
                self.label,
                build_logic(transform_to_conditional, transform_to_default),
            );
        }
    }
}

/// Since all criteria are combined in the VHDL code,
/// the MIR criteria IDs must be mapped to the new ones.
#[derive(Debug)]
struct CriteriaMapping(HashMap<mir::CriterionId, CriterionId>);

impl CriteriaMapping {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, from: mir::CriterionId, to: CriterionId) {
        self.0.insert(from, to);
    }

    /// All `mir_criteria` must be in the CriteriaMapping.
    ///
    /// # Panics
    ///
    /// Panics if any criterion in `mir_criteria` is not in the CriteriaMapping.
    fn map(&self, mir_criteria: &[mir::Criterion]) -> Vec<Criterion> {
        mir_criteria
            .iter()
            .map(|criterion| match criterion {
                mir::Criterion::True(id) => Criterion::True(*self.0.get(id).unwrap()),
                mir::Criterion::False(id) => Criterion::False(*self.0.get(id).unwrap()),
            })
            .collect()
    }
}

fn build_logic(conditional: Vec<(Or<And<Criterion>>, Label)>, default: Label) -> NextStateLogic {
    if conditional.is_empty() {
        NextStateLogic::Label(default)
    } else {
        NextStateLogic::Cond {
            conditional: Vec1::try_from_vec(
                conditional
                    .into_iter()
                    .map(|(or, label)| (or, NextStateLogic::Label(label)))
                    .collect(),
            )
            .unwrap(),
            default: Box::new(NextStateLogic::Label(default)),
        }
    }
}
