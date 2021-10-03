use crate::mir::*;
use rtcore::value::{Bit, Value};

pub trait Evaluate {
    fn evaluate(&self, ctx_size: usize) -> Option<Value>;
}

impl Evaluate for Expression<'_> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        match self {
            Self::Atom(atom) => atom.evaluate(ctx_size),
            Self::BinaryTerm(term) => term.evaluate(ctx_size),
            Self::UnaryTerm(term) => term.evaluate(ctx_size),
        }
    }
}

impl Evaluate for Atom<'_> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        match self {
            Self::Concat(concat) => concat.evaluate(ctx_size),
            Self::Register(reg) => reg.evaluate(ctx_size),
            Self::Bus(bus) => bus.evaluate(ctx_size),
            Self::RegisterArray(reg_array) => reg_array.evaluate(ctx_size),
            Self::Number(number) => number.node.evaluate(ctx_size),
        }
    }
}

impl Evaluate for BinaryTerm<'_> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        let ctx_size_inner = self.ctx_size.calc(ctx_size);
        let lhs = self.lhs.evaluate(ctx_size_inner)?;
        let rhs = self.rhs.evaluate(ctx_size_inner)?;

        let mut value = match self.operator.node {
            BinaryOperator::Eq => Value::from(Bit::from(lhs == rhs)),
            BinaryOperator::Ne => Value::from(Bit::from(lhs != rhs)),
            BinaryOperator::Le => Value::from(Bit::from(lhs <= rhs)),
            BinaryOperator::Lt => Value::from(Bit::from(lhs < rhs)),
            BinaryOperator::Ge => Value::from(Bit::from(lhs >= rhs)),
            BinaryOperator::Gt => Value::from(Bit::from(lhs > rhs)),
            BinaryOperator::Add => lhs + rhs,
            BinaryOperator::Sub => lhs - rhs,
            BinaryOperator::And => lhs & rhs,
            BinaryOperator::Nand => !(lhs & rhs),
            BinaryOperator::Or => lhs | rhs,
            BinaryOperator::Nor => !(lhs | rhs),
            BinaryOperator::Xor => lhs ^ rhs,
        };
        value.extend_zero(ctx_size);
        Some(value)
    }
}

impl Evaluate for UnaryTerm<'_> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        let ctx_size_inner = self.ctx_size.calc(ctx_size);
        let mut rhs = self.expression.evaluate(ctx_size_inner)?;

        let mut value = match self.operator.node {
            UnaryOperator::Sign | UnaryOperator::Neg => -rhs,
            UnaryOperator::Not => !rhs,
            UnaryOperator::Sxt => {
                rhs.extend_sign(ctx_size);
                rhs
            }
        };
        value.extend_zero(ctx_size);
        Some(value)
    }
}

impl Evaluate for Register<'_> {
    fn evaluate(&self, _: usize) -> Option<Value> {
        None
    }
}

impl Evaluate for Bus<'_> {
    fn evaluate(&self, _: usize) -> Option<Value> {
        None
    }
}

impl Evaluate for RegisterArray<'_> {
    fn evaluate(&self, _: usize) -> Option<Value> {
        None
    }
}

impl Evaluate for Number {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        let mut value = self.value.clone();
        value.extend_zero(ctx_size);
        Some(value)
    }
}

impl Evaluate for Concat<ConcatPartExpr<'_>> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        let values = self
            .parts
            .iter()
            .map(|part| match part {
                ConcatPartExpr::Register(_) => None,
                ConcatPartExpr::Bus(_) => None,
                ConcatPartExpr::RegisterArray(_) => None,
                ConcatPartExpr::Number(number) => Some(number.node.value.clone()),
            })
            .collect::<Option<Vec<Value>>>()?;

        let mut value = Value::concat(values.iter().map(Value::as_slice));
        value.extend_zero(ctx_size);
        Some(value)
    }
}
