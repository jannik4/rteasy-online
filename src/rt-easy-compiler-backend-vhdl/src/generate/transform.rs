use crate::error::SynthError;
use crate::vhdl::*;

/// Checks if, given the operations performed and the criteria for the next state logic, it is
/// necessary to transform the next state logic to the previous states.
pub fn should_transform_next_state_logic(
    _operations: &[&Operation],
    criteria: &[&Expression],
) -> Result<bool, SynthError> {
    // TODO: Count criteria as dependencies only if they are actually assigned in operations.

    let mut deps = NextStateLogicDeps::empty();

    for criterion in criteria {
        deps = deps | deps_expr(criterion);
    }

    match (deps.clocked, deps.unclocked) {
        (_, true) => Err(SynthError::NextStateUnclockedDependency),
        (c, _) => Ok(c),
    }
}

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
