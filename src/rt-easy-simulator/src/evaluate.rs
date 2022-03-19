use crate::{state::State, Error};
use rtcore::value::{Bit, Value};
use rtprogram::{
    Atom, BinaryOperator, BinaryTerm, Bus, Concat, ConcatPartExpr, Expression, ExpressionKind,
    Number, Register, RegisterArray, UnaryOperator, UnaryTerm,
};
use std::convert::Infallible;

type Result<T = Value> = std::result::Result<T, Error>;

pub trait Evaluate {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result;
}

impl Evaluate for Expression {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        match &self.kind {
            ExpressionKind::Atom(atom) => atom.evaluate(state, ctx_size),
            ExpressionKind::BinaryTerm(term) => term.evaluate(state, ctx_size),
            ExpressionKind::UnaryTerm(term) => term.evaluate(state, ctx_size),
        }
    }
}

impl Evaluate for Atom {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        match self {
            Self::Concat(concat) => concat.evaluate(state, ctx_size),
            Self::Register(reg) => reg.evaluate(state, ctx_size),
            Self::Bus(bus) => bus.evaluate(state, ctx_size),
            Self::RegisterArray(reg_array) => reg_array.evaluate(state, ctx_size),
            Self::Number(number) => number.evaluate(state, ctx_size),
        }
    }
}

impl Evaluate for BinaryTerm {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let ctx_size_inner = self.ctx_size.calc(ctx_size);
        let lhs = self.lhs.evaluate(state, ctx_size_inner)?;
        let rhs = self.rhs.evaluate(state, ctx_size_inner)?;

        let mut value = match self.operator {
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
        Ok(value)
    }
}

impl Evaluate for UnaryTerm {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let ctx_size_inner = self.ctx_size.calc(ctx_size);
        let mut rhs = self.expression.evaluate(state, ctx_size_inner)?;

        let mut value = match self.operator {
            UnaryOperator::Sign | UnaryOperator::Neg => -rhs,
            UnaryOperator::Not => !rhs,
            UnaryOperator::Sxt => {
                rhs.extend_sign(ctx_size);
                rhs
            }
        };
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for Register {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let mut value = state.register(&self.ident)?.read(self.range)?;
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for Bus {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let mut value = state.bus(&self.ident)?.read(self.range)?;
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for RegisterArray {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let idx = self.index.evaluate(state, self.index_ctx_size)?;

        let mut value = state.register_array(&self.ident)?.read(idx)?;
        value.extend_zero(ctx_size);

        Ok(value)
    }
}

impl Evaluate for Number {
    fn evaluate(&self, _: &State, ctx_size: usize) -> Result {
        let mut value = self.value.clone();
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for Concat<ConcatPartExpr> {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let values = self
            .parts
            .iter()
            .map(|part| match part {
                ConcatPartExpr::Register(reg) => state.register(&reg.ident)?.read(reg.range),
                ConcatPartExpr::Bus(bus) => state.bus(&bus.ident)?.read(bus.range),
                ConcatPartExpr::RegisterArray(reg_array) => {
                    let idx = reg_array.index.evaluate(state, reg_array.index_ctx_size)?;
                    state.register_array(&reg_array.ident)?.read(idx)
                }
                ConcatPartExpr::Number(number) => Ok(number.value.clone()),
            })
            .collect::<Result<Vec<Value>>>()?;

        let mut value = Value::concat(values.iter().map(Value::as_slice));
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for Infallible {
    fn evaluate(&self, _: &State, _: usize) -> Result {
        match *self {}
    }
}
