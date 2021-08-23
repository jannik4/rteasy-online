use crate::mir::*;
use rtcore::value::Value;

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
            Self::Number(number) => number.evaluate(ctx_size),
        }
    }
}

impl Evaluate for BinaryTerm<'_> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        let ctx_size = self.ctx_size.calc(ctx_size);
        let lhs = self.lhs.evaluate(ctx_size)?;
        let rhs = self.rhs.evaluate(ctx_size)?;

        Some(match self.operator {
            BinaryOperator::Eq => {
                if lhs == rhs {
                    Value::one(1)
                } else {
                    Value::zero(1)
                }
            }
            BinaryOperator::Ne => {
                if lhs != rhs {
                    Value::one(1)
                } else {
                    Value::zero(1)
                }
            }
            BinaryOperator::Add => lhs + rhs,
            BinaryOperator::Sub => lhs - rhs,
            _ => todo!(),
        })
    }
}

impl Evaluate for UnaryTerm<'_> {
    fn evaluate(&self, ctx_size: usize) -> Option<Value> {
        let ctx_size_inner = self.ctx_size.calc(ctx_size);
        let mut value = self.expression.evaluate(ctx_size_inner)?;

        Some(match self.operator {
            UnaryOperator::SignNeg => -value,
            UnaryOperator::Not => !value,
            UnaryOperator::Sxt => {
                value.extend_sign(ctx_size);
                value
            }
        })
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
                ConcatPartExpr::Number(number) => Some(number.value.clone()),
            })
            .collect::<Option<Vec<Value>>>()?;

        let mut value = Value::concat(values.iter().map(Value::as_slice));
        value.extend_zero(ctx_size);
        Some(value)
    }
}
