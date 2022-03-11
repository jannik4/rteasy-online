use super::{expression::generate_expression, operation::generate_assignment};
use crate::vhdl::*;
use compiler::mir;
use indexmap::{IndexMap, IndexSet};
use std::collections::{HashMap, HashSet};
use vec1::{vec1, Vec1};

pub fn generate_statement<'s>(
    idx: usize,
    mir_statement: &mir::Statement<'s>,
    next_mir_statement: Option<&mir::Statement<'s>>,
    vhdl: &mut Vhdl<'s>,
    label_goto_prefix: &str,
) -> Option<(Label, GenNextStateLogic)> {
    // ...
    let gen_statement = generate_statement_(
        idx,
        mir_statement,
        next_mir_statement,
        &vhdl.declarations,
        |cond| CriterionId(vhdl.criteria.insert_full(cond).0),
        |op| OperationId(vhdl.operations.insert_full(op).0),
    );

    // ...
    let fix_needed = if mir_statement_has_pipe(mir_statement)
        || gen_statement.next_state_logic.conditional.is_empty()
    {
        false
    } else {
        let deps = next_state_logic_deps(&gen_statement.next_state_logic, &vhdl.criteria);
        match (deps.clocked, deps.unclocked) {
            (true, true) => panic!("synth error"), // TODO: Error instead of panic
            (true, false) => true,
            (false, _) => false,
        }
    };

    // ...
    if !fix_needed {
        vhdl.statements.push(Statement {
            label: gen_statement.label,
            next_state_logic: gen_to_std(gen_statement.next_state_logic),
            operations: gen_statement.operations,
        });
        return None;
    }

    // ...
    let mut fix_labels = GenNextStateLogic {
        conditional: Vec::new(),
        default: Label(format!(
            "{}{}{}",
            gen_statement.label.0, label_goto_prefix, gen_statement.next_state_logic.default.0
        )),
    };
    for (cond, label) in gen_statement.next_state_logic.conditional {
        fix_labels.conditional.push((
            cond,
            Label(format!("{}{}{}", gen_statement.label.0, label_goto_prefix, label.0)),
        ));
        vhdl.statements.push(Statement {
            label: Label(format!("{}{}{}", gen_statement.label.0, label_goto_prefix, label.0)),
            next_state_logic: NextStateLogic::Label(label),
            operations: gen_statement.operations.clone(),
        });
    }
    vhdl.statements.push(Statement {
        label: Label(format!(
            "{}{}{}",
            gen_statement.label.0, label_goto_prefix, gen_statement.next_state_logic.default.0
        )),
        next_state_logic: NextStateLogic::Label(gen_statement.next_state_logic.default),
        operations: gen_statement.operations.clone(),
    });
    Some((gen_statement.label, fix_labels))
}

pub fn gen_to_std(gen: GenNextStateLogic) -> NextStateLogic {
    if gen.conditional.is_empty() {
        NextStateLogic::Label(gen.default)
    } else {
        NextStateLogic::Cond {
            conditional: Vec1::try_from_vec(
                gen.conditional
                    .into_iter()
                    .map(|(or, label)| (or, NextStateLogic::Label(label)))
                    .collect(),
            )
            .unwrap(),
            default: Box::new(NextStateLogic::Label(gen.default)),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct GenStatement {
    label: Label,
    next_state_logic: GenNextStateLogic,
    operations: IndexMap<OperationId, Option<Or<And<Criterion>>>>,
}

#[derive(Debug, Clone)]
pub struct GenNextStateLogic {
    pub conditional: Vec<(Or<And<Criterion>>, Label)>,
    pub default: Label,
}

fn generate_statement_<'s>(
    idx: usize,
    mir_statement: &mir::Statement<'s>,
    next_mir_statement: Option<&mir::Statement<'s>>,
    declarations: &Declarations<'s>,

    mut get_criterion_id: impl FnMut(Expression<'s>) -> CriterionId,
    mut get_operation_id: impl FnMut(Operation<'s>) -> OperationId,
) -> GenStatement {
    // Create statement
    let mut statement = GenStatement {
        label: match mir_statement.label.as_ref() {
            Some(label) => Label::named(label.node.0),
            None => Label::unnamed(idx),
        },
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
                let condition = generate_expression(&mir_eval_criterion.condition, declarations, 1);
                criteria_mapping
                    .insert(mir_eval_criterion.criterion_id, get_criterion_id(condition));
            }
            mir::Operation::EvalCriterionSwitchGroup(_) => todo!(),

            // If the step has no criteria, the default next state is overwritten.
            // Otherwise the entry for the label in next_state_conditional is created
            // or extended with the criteria of this step.
            mir::Operation::Goto(mir_goto) => {
                let label = Label::named(mir_goto.label.node.0);

                if mir_step.criteria.is_empty() {
                    statement.next_state_logic.default = label;
                } else {
                    let and = And(map_criteria(&mir_step.criteria, &criteria_mapping));
                    let entry = statement
                        .next_state_logic
                        .conditional
                        .iter_mut()
                        .find(|(_, l)| l == &label);
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
            mir::Operation::Write(_) => todo!(),
            mir::Operation::Read(_) => todo!(),
            mir::Operation::Assignment(mir_assignment) => {
                let operation =
                    Operation::Assignment(generate_assignment(mir_assignment, declarations));
                let operation_id = get_operation_id(operation);

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

    statement
}

// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

fn mir_statement_has_pipe(statement: &mir::Statement<'_>) -> bool {
    statement.steps.node.iter().any(|step| step.annotation.is_post_pipe)
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

fn next_state_logic_deps<'s>(
    logic: &GenNextStateLogic,
    criteria: &IndexSet<Expression<'s>>,
) -> NextStateLogicDeps {
    let mut deps = NextStateLogicDeps::empty();

    let mut logic_criteria = HashSet::new();
    for (or, _) in &logic.conditional {
        for and in &or.0 {
            for criterion in &and.0 {
                logic_criteria.insert(criterion.id());
            }
        }
    }

    criteria
        .iter()
        .enumerate()
        .filter(|(idx, _)| logic_criteria.contains(&CriterionId(*idx)))
        .for_each(|(_, expr)| {
            deps = deps | deps_expr(expr);
        });

    deps
}

fn deps_expr<'s>(expr: &Expression<'s>) -> NextStateLogicDeps {
    match &expr.kind {
        ExpressionKind::Atom(e) => deps_atom(e),
        ExpressionKind::BinaryTerm(e) => deps_binary_term(e),
        ExpressionKind::UnaryTerm(e) => deps_unary_term(e),
    }
}
fn deps_atom<'s>(atom: &Atom<'s>) -> NextStateLogicDeps {
    match atom {
        Atom::Concat(concat) => deps_concat(concat),
        Atom::Register(_) => NextStateLogicDeps::clocked(),
        Atom::Bus(_) => NextStateLogicDeps::unclocked(),
        Atom::RegisterArray(_) => NextStateLogicDeps::clocked(),
        Atom::Number(_) => NextStateLogicDeps::empty(),
    }
}
fn deps_binary_term<'s>(binary_term: &BinaryTerm<'s>) -> NextStateLogicDeps {
    deps_expr(&binary_term.lhs) | deps_expr(&binary_term.rhs)
}
fn deps_unary_term<'s>(unary_term: &UnaryTerm<'s>) -> NextStateLogicDeps {
    deps_expr(&unary_term.expression)
}
fn deps_concat<'s>(concat: &ConcatExpr<'s>) -> NextStateLogicDeps {
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
