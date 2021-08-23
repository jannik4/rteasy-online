use crate::mir::*;
use std::collections::HashSet;

pub fn calc_direct_dependencies(step: &Step<'_>, others: &[Step<'_>]) -> HashSet<StepId> {
    let mut ids = HashSet::new();

    for other in others {
        let is_dependent = match &other.operation.kind {
            OperationKind::EvalCriterion(eval_criterion) => {
                step.criteria.iter().any(|criterion| criterion.id() == eval_criterion.criterion_id)
            }
            OperationKind::Assignment(assignment) => match &assignment.lhs {
                Lvalue::Bus(bus) => step.is_dependent_on(bus),
                Lvalue::ConcatUnclocked(concat) => concat.parts.iter().any(|part| match part {
                    ConcatPartLvalueUnclocked::Bus(bus, _) => step.is_dependent_on(bus),
                }),
                Lvalue::Register(_) | Lvalue::RegisterArray(_) | Lvalue::ConcatClocked(_) => false,
            },
            OperationKind::Nop(_)
            | OperationKind::Goto(_)
            | OperationKind::Write(_)
            | OperationKind::Read(_) => false,
        };

        if is_dependent {
            ids.insert(other.id);
        }
    }

    ids
}

trait IsDependentOn {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool;
}

impl IsDependentOn for Step<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        match &self.operation.kind {
            OperationKind::EvalCriterion(eval_criterion) => {
                eval_criterion.condition.is_dependent_on(bus)
            }
            OperationKind::Assignment(assignment) => assignment.rhs.is_dependent_on(bus),
            OperationKind::Nop(_)
            | OperationKind::Goto(_)
            | OperationKind::Write(_)
            | OperationKind::Read(_) => false,
        }
    }
}

impl IsDependentOn for Expression<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        match self {
            Expression::Atom(atom) => atom.is_dependent_on(bus),
            Expression::BinaryTerm(term) => term.is_dependent_on(bus),
            Expression::UnaryTerm(term) => term.is_dependent_on(bus),
        }
    }
}

impl IsDependentOn for Atom<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        match self {
            Atom::Concat(concat) => concat.is_dependent_on(bus),
            Atom::Bus(self_bus) => self_bus.is_dependent_on(bus),
            Atom::Register(_) | Atom::RegisterArray(_) | Atom::Number(_) => false,
        }
    }
}

impl IsDependentOn for Concat<ConcatPartExpr<'_>> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        self.parts.iter().any(|part| part.is_dependent_on(bus))
    }
}

impl IsDependentOn for ConcatPartExpr<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        match self {
            ConcatPartExpr::Bus(self_bus) => self_bus.is_dependent_on(bus),
            ConcatPartExpr::Register(_)
            | ConcatPartExpr::RegisterArray(_)
            | ConcatPartExpr::Number(_) => false,
        }
    }
}

impl IsDependentOn for BinaryTerm<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        self.lhs.is_dependent_on(bus) || self.rhs.is_dependent_on(bus)
    }
}

impl IsDependentOn for UnaryTerm<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        self.expression.is_dependent_on(bus)
    }
}

impl IsDependentOn for Register<'_> {
    fn is_dependent_on(&self, _: &Bus<'_>) -> bool {
        false
    }
}

impl IsDependentOn for Bus<'_> {
    fn is_dependent_on(&self, bus: &Bus<'_>) -> bool {
        self.ident == bus.ident && BitRange::intersect(self.range, bus.range)
    }
}

impl IsDependentOn for RegisterArray<'_> {
    fn is_dependent_on(&self, _: &Bus<'_>) -> bool {
        false
    }
}
