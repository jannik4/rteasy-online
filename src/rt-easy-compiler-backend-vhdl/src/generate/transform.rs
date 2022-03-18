use super::{GenNextStateLogic, GenStatement, GenVhdl};
use crate::vhdl::*;
use indexmap::{IndexMap, IndexSet};
use std::collections::HashSet;
use vec1::Vec1;

pub(super) fn transform(vhdl: GenVhdl) -> Vhdl {
    // ...
    let mut statements = Vec::new();

    let label_goto_prefix = calc_label_goto_prefix(&vhdl);
    let mut transform_labels_list = Vec::new();

    // ...
    for statement in vhdl.statements {
        if should_transform(&statement, &vhdl.criteria) {
            let mut transform_to = GenNextStateLogic {
                conditional: Vec::new(),
                default: Label(format!(
                    "{}{}{}",
                    statement.label, label_goto_prefix, statement.next_state_logic.default.0
                )),
            };
            for (cond, label) in statement.next_state_logic.conditional {
                transform_to.conditional.push((
                    cond,
                    Label(format!("{}{}{}", statement.label, label_goto_prefix, label)),
                ));
                statements.push(Statement {
                    label: Label(format!("{}{}{}", statement.label, label_goto_prefix, label)),
                    next_state_logic: NextStateLogic::Label(label),
                    operations: statement.operations.clone(),
                });
            }
            statements.push(Statement {
                label: Label(format!(
                    "{}{}{}",
                    statement.label, label_goto_prefix, statement.next_state_logic.default.0
                )),
                next_state_logic: NextStateLogic::Label(statement.next_state_logic.default),
                operations: statement.operations.clone(),
            });
            transform_labels_list.push((statement.label, build_logic(transform_to)));
        } else {
            statements.push(Statement {
                label: statement.label,
                next_state_logic: build_logic(statement.next_state_logic),
                operations: statement.operations,
            })
        }
    }

    // Transform labels
    for (label, to) in transform_labels_list {
        for statement in &mut statements {
            transform_labels(&mut statement.next_state_logic, &label, to.clone());
        }
    }

    // Terminated statement
    statements.push(Statement {
        label: Label::terminated(),
        next_state_logic: NextStateLogic::Label(Label::terminated()),
        operations: IndexMap::new(),
    });

    Vhdl {
        statements,
        criteria: vhdl.criteria,
        operations: vhdl.operations,
        declarations: vhdl.declarations,
    }
}

fn should_transform(statement: &GenStatement, criteria: &IndexSet<Expression>) -> bool {
    if statement.has_pipe || statement.next_state_logic.conditional.is_empty() {
        false
    } else {
        let deps = logic_deps(&statement.next_state_logic, criteria);
        match (deps.clocked, deps.unclocked) {
            (true, true) => panic!("synth error"), // TODO: Error instead of panic
            (true, false) => true,
            (false, _) => false,
        }
    }
}

fn logic_deps(logic: &GenNextStateLogic, criteria: &IndexSet<Expression>) -> NextStateLogicDeps {
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

fn build_logic(logic: GenNextStateLogic) -> NextStateLogic {
    if logic.conditional.is_empty() {
        NextStateLogic::Label(logic.default)
    } else {
        NextStateLogic::Cond {
            conditional: Vec1::try_from_vec(
                logic
                    .conditional
                    .into_iter()
                    .map(|(or, label)| (or, NextStateLogic::Label(label)))
                    .collect(),
            )
            .unwrap(),
            default: Box::new(NextStateLogic::Label(logic.default)),
        }
    }
}

fn transform_labels(logic: &mut NextStateLogic, from: &Label, to: NextStateLogic) {
    match logic {
        NextStateLogic::Label(l) => {
            if l == from {
                *logic = to;
            }
        }
        NextStateLogic::Cond { conditional, default } => {
            for (_, logic) in conditional {
                transform_labels(logic, from, to.clone());
            }
            transform_labels(&mut **default, from, to);
        }
    }
}

fn calc_label_goto_prefix(vhdl: &GenVhdl) -> String {
    let mut prefix = "_GOTO_".to_string();

    loop {
        let any_label_contains_prefix =
            vhdl.statements.iter().any(|statement| statement.label.0.contains(&prefix));
        if any_label_contains_prefix {
            prefix += "_";
        } else {
            break;
        }
    }

    return prefix;
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
