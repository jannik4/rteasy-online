use super::{
    expression::generate_expression,
    operation::{generate_assignment, generate_read, generate_write},
    util::CriteriaMapping,
};
use crate::vhdl::*;
use compiler::mir;
use indexmap::IndexMap;
use std::collections::HashSet;
use vec1::{vec1, Vec1};

pub fn generate_statement<'s>(
    idx: usize,
    mir_statement: &mir::Statement<'s>,
    next_mir_statement: Option<&mir::Statement<'s>>,
    vhdl: &mut Vhdl,
    label_goto_prefix: &str,
) -> Option<(Label, NextStateLogic)> {
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
        statement.add_step(
            mir_step,
            &vhdl.declarations,
            |cond| CriterionId(vhdl.criteria.insert_full(cond).0),
            |op| OperationId(vhdl.operations.insert_full(op).0),
        );
    }

    // Sort operations by operation id ASC
    statement.operations.sort_keys();

    // Finish
    statement.finish(vhdl, label_goto_prefix)
}

// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct GenStatement {
    label: Label,
    operations: IndexMap<OperationId, Option<Or<And<Criterion>>>>,
    next_state_logic: GenNextStateLogic,

    has_pipe: bool,
    criteria_mapping: CriteriaMapping,
}

impl GenStatement {
    fn add_step(
        &mut self,
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
                self.criteria_mapping
                    .insert(mir_eval_criterion.criterion_id, get_criterion_id(condition));
            }
            mir::Operation::EvalCriterionSwitchGroup(_) => todo!(),

            // If the step has no criteria, the default next state is overwritten.
            // Otherwise the entry for the label in next_state_conditional is created
            // or extended with the criteria of this step.
            mir::Operation::Goto(mir_goto) => {
                let label = Label::named(mir_goto.label.node.0);

                if mir_step.criteria.is_empty() {
                    self.next_state_logic.default = label;
                } else {
                    let and =
                        And(Vec1::try_from(self.criteria_mapping.map(&mir_step.criteria)).unwrap());
                    let entry =
                        self.next_state_logic.conditional.iter_mut().find(|(_, l)| l == &label);
                    match entry {
                        Some((criteria, _)) => criteria.0.push(and),
                        None => {
                            self.next_state_logic.conditional.push((Or(vec1![and]), label));
                        }
                    }
                }
            }

            // First, map the operation, insert it into the global VHDL operations set and get the id.
            // Then upsert the operation id into the statement operations and update the criteria.
            mir::Operation::Write(mir_write) => {
                let operation = Operation::Write(generate_write(mir_write, declarations));
                let operation_id = get_operation_id(operation);
                self.add_op(operation_id, &mir_step.criteria);
            }
            mir::Operation::Read(mir_read) => {
                let operation = Operation::Read(generate_read(mir_read, declarations));
                let operation_id = get_operation_id(operation);
                self.add_op(operation_id, &mir_step.criteria);
            }
            mir::Operation::Assignment(mir_assignment) => {
                let operation =
                    Operation::Assignment(generate_assignment(mir_assignment, declarations));
                let operation_id = get_operation_id(operation);
                self.add_op(operation_id, &mir_step.criteria);
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

    fn should_transform(&self, vhdl: &Vhdl) -> bool {
        if self.has_pipe || self.next_state_logic.conditional.is_empty() {
            false
        } else {
            let deps = self.next_state_logic.deps(vhdl);
            match (deps.clocked, deps.unclocked) {
                (true, true) => panic!("synth error"), // TODO: Error instead of panic
                (true, false) => true,
                (false, _) => false,
            }
        }
    }

    fn finish(self, vhdl: &mut Vhdl, label_goto_prefix: &str) -> Option<(Label, NextStateLogic)> {
        // ...
        if !self.should_transform(vhdl) {
            vhdl.statements.push(Statement {
                label: self.label,
                next_state_logic: self.next_state_logic.build(),
                operations: self.operations,
            });
            return None;
        }

        // ...
        let mut fix_labels = GenNextStateLogic {
            conditional: Vec::new(),
            default: Label(format!(
                "{}{}{}",
                self.label, label_goto_prefix, self.next_state_logic.default.0
            )),
        };
        for (cond, label) in self.next_state_logic.conditional {
            fix_labels
                .conditional
                .push((cond, Label(format!("{}{}{}", self.label, label_goto_prefix, label))));
            vhdl.statements.push(Statement {
                label: Label(format!("{}{}{}", self.label, label_goto_prefix, label)),
                next_state_logic: NextStateLogic::Label(label),
                operations: self.operations.clone(),
            });
        }
        vhdl.statements.push(Statement {
            label: Label(format!(
                "{}{}{}",
                self.label, label_goto_prefix, self.next_state_logic.default.0
            )),
            next_state_logic: NextStateLogic::Label(self.next_state_logic.default),
            operations: self.operations.clone(),
        });
        Some((self.label, fix_labels.build()))
    }
}

#[derive(Debug, Clone)]
struct GenNextStateLogic {
    pub conditional: Vec<(Or<And<Criterion>>, Label)>,
    pub default: Label,
}

impl GenNextStateLogic {
    fn deps(&self, vhdl: &Vhdl) -> NextStateLogicDeps {
        let mut deps = NextStateLogicDeps::empty();

        let mut logic_criteria = HashSet::new();
        for (or, _) in &self.conditional {
            for and in &or.0 {
                for criterion in &and.0 {
                    logic_criteria.insert(criterion.id());
                }
            }
        }

        vhdl.criteria
            .iter()
            .enumerate()
            .filter(|(idx, _)| logic_criteria.contains(&CriterionId(*idx)))
            .for_each(|(_, expr)| {
                deps = deps | deps_expr(expr);
            });

        deps
    }

    fn build(self) -> NextStateLogic {
        if self.conditional.is_empty() {
            NextStateLogic::Label(self.default)
        } else {
            NextStateLogic::Cond {
                conditional: Vec1::try_from_vec(
                    self.conditional
                        .into_iter()
                        .map(|(or, label)| (or, NextStateLogic::Label(label)))
                        .collect(),
                )
                .unwrap(),
                default: Box::new(NextStateLogic::Label(self.default)),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct NextStateLogicDeps {
    clocked: bool,
    unclocked: bool,
}

impl NextStateLogicDeps {
    fn empty() -> Self {
        Self { clocked: false, unclocked: false }
    }
    fn clocked() -> Self {
        Self { clocked: true, unclocked: false }
    }
    fn unclocked() -> Self {
        Self { clocked: false, unclocked: true }
    }
}

impl std::ops::BitOr for NextStateLogicDeps {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { clocked: self.clocked || rhs.clocked, unclocked: self.unclocked || rhs.unclocked }
    }
}

fn deps_expr(expr: &Expression) -> NextStateLogicDeps {
    match &expr.kind {
        ExpressionKind::Atom(e) => deps_atom(e),
        ExpressionKind::BinaryTerm(e) => deps_binary_term(e),
        ExpressionKind::UnaryTerm(e) => deps_unary_term(e),
    }
}
fn deps_atom(atom: &Atom) -> NextStateLogicDeps {
    match atom {
        Atom::Concat(concat) => deps_concat(concat),
        Atom::Register(_) => NextStateLogicDeps::clocked(),
        Atom::Bus(_) => NextStateLogicDeps::unclocked(),
        Atom::RegisterArray(_) => NextStateLogicDeps::clocked(),
        Atom::Number(_) => NextStateLogicDeps::empty(),
    }
}
fn deps_binary_term(binary_term: &BinaryTerm) -> NextStateLogicDeps {
    deps_expr(&binary_term.lhs) | deps_expr(&binary_term.rhs)
}
fn deps_unary_term(unary_term: &UnaryTerm) -> NextStateLogicDeps {
    deps_expr(&unary_term.expression)
}
fn deps_concat(concat: &ConcatExpr) -> NextStateLogicDeps {
    let mut deps = NextStateLogicDeps::empty();
    for part in &concat.parts {
        deps = deps
            | match &part {
                ConcatPartExpr::Register(_) => NextStateLogicDeps::clocked(),
                ConcatPartExpr::Bus(_) => NextStateLogicDeps::unclocked(),
                ConcatPartExpr::RegisterArray(_) => NextStateLogicDeps::clocked(),
                ConcatPartExpr::Number(_) => NextStateLogicDeps::empty(),
            };
    }
    deps
}
