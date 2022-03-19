use compiler::mir::*;
use std::collections::HashMap;

// TODO: Count clocked dependencies only if they are actually assigned in this statement.

pub fn next_state_logic_deps(statement: &Statement<'_>) -> NextStateLogicDeps {
    // No dependencies if goto is already post pipe
    if statement.steps.node.iter().any(|step| step.annotation.is_post_pipe) {
        return NextStateLogicDeps::empty();
    }

    // Collect criteria
    let mut criteria = HashMap::new();
    for step in &statement.steps.node {
        match &step.operation {
            Operation::EvalCriterion(eval_criterion) => {
                criteria.insert(eval_criterion.criterion_id, &eval_criterion.condition);
            }
            Operation::EvalCriterionSwitchGroup(group) => {
                for eval_criterion in &group.eval_criteria {
                    criteria.insert(eval_criterion.criterion_id, &eval_criterion.condition);
                }
            }
            Operation::Nop(_)
            | Operation::Goto(_)
            | Operation::Write(_)
            | Operation::Read(_)
            | Operation::Assignment(_)
            | Operation::Assert(_) => (),
        }
    }

    // Collect criteria for gotos
    let mut goto_criteria = HashMap::new();
    for step in &statement.steps.node {
        match &step.operation {
            Operation::Goto(_) => {
                for criterion in &step.criteria {
                    let id = criterion.id();
                    goto_criteria.insert(id, criteria.get(&id).unwrap());
                }
            }
            Operation::EvalCriterion(_)
            | Operation::EvalCriterionSwitchGroup(_)
            | Operation::Nop(_)
            | Operation::Write(_)
            | Operation::Read(_)
            | Operation::Assignment(_)
            | Operation::Assert(_) => (),
        }
    }

    // Build deps
    let mut deps = NextStateLogicDeps::empty();
    for (_, expr) in goto_criteria {
        deps = deps | deps_expr(expr);
    }
    deps
}

#[derive(Debug, Clone, Copy)]
pub struct NextStateLogicDeps {
    pub clocked: bool,
    pub unclocked: bool,
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

fn deps_expr(expr: &Expression<'_>) -> NextStateLogicDeps {
    match &expr {
        Expression::Atom(e) => deps_atom(e),
        Expression::BinaryTerm(e) => deps_binary_term(e),
        Expression::UnaryTerm(e) => deps_unary_term(e),
    }
}
fn deps_atom(atom: &Atom<'_>) -> NextStateLogicDeps {
    match atom {
        Atom::Concat(concat) => deps_concat(concat),
        Atom::Register(_) => NextStateLogicDeps::clocked(),
        Atom::Bus(bus) => deps_bus(bus),
        Atom::RegisterArray(_) => NextStateLogicDeps::clocked(),
        Atom::Number(_) => NextStateLogicDeps::empty(),
    }
}
fn deps_binary_term(binary_term: &BinaryTerm<'_>) -> NextStateLogicDeps {
    deps_expr(&binary_term.lhs) | deps_expr(&binary_term.rhs)
}
fn deps_unary_term(unary_term: &UnaryTerm<'_>) -> NextStateLogicDeps {
    deps_expr(&unary_term.expression)
}
fn deps_concat(concat: &ConcatExpr<'_>) -> NextStateLogicDeps {
    let mut deps = NextStateLogicDeps::empty();
    for part in &concat.parts {
        deps = deps
            | match &part {
                ConcatPartExpr::Register(_) => NextStateLogicDeps::clocked(),
                ConcatPartExpr::Bus(bus) => deps_bus(bus),
                ConcatPartExpr::RegisterArray(_) => NextStateLogicDeps::clocked(),
                ConcatPartExpr::Number(_) => NextStateLogicDeps::empty(),
            };
    }
    deps
}
fn deps_bus(bus: &Bus<'_>) -> NextStateLogicDeps {
    match bus.kind {
        BusKind::Intern => NextStateLogicDeps::unclocked(),
        BusKind::Input => NextStateLogicDeps::empty(), // TODO: ???
    }
}
