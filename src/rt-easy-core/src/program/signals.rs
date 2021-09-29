use super::*;

#[derive(Debug)]
pub struct Signals {
    pub condition_signals: Vec<String>,
    pub control_signals: Vec<String>,
}

impl Program {
    pub fn signals(&self) -> Signals {
        let mut condition_signals = Vec::new();
        let mut control_signals = Vec::new();

        for statement in &self.statements {
            for step in statement.steps.node.as_slice() {
                match &step.operation.kind {
                    OperationKind::EvalCriterion(eval_criterion) => {
                        push_if_not_exists(
                            &mut condition_signals,
                            eval_criterion.condition.to_string(),
                        );
                    }
                    OperationKind::EvalCriterionGroup(eval_criterion_group) => {
                        for eval_criterion in &eval_criterion_group.0 {
                            push_if_not_exists(
                                &mut condition_signals,
                                eval_criterion.condition.to_string(),
                            );
                        }
                    }
                    OperationKind::Nop(_nop) => (),
                    OperationKind::Goto(_goto) => (),
                    OperationKind::Write(write) => {
                        push_if_not_exists(&mut control_signals, write.to_string());
                    }
                    OperationKind::Read(read) => {
                        push_if_not_exists(&mut control_signals, read.to_string());
                    }
                    OperationKind::Assignment(assignment) => {
                        push_if_not_exists(&mut control_signals, assignment.to_string());
                    }
                    OperationKind::Assert(_assert) => (),
                }
            }
        }

        Signals { condition_signals, control_signals }
    }
}

fn push_if_not_exists<T>(vec: &mut Vec<T>, element: T)
where
    T: PartialEq,
{
    if vec.iter().find(|e| **e == element).is_none() {
        vec.push(element);
    }
}
