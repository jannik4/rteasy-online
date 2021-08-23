use crate::{Error, State};
use rtcore::{
    program::{
        Atom, BinaryOperator, BinaryTerm, Bus, Concat, ConcatPartExpr, Expression, Number,
        Register, RegisterArray, UnaryOperator, UnaryTerm,
    },
    value::Value,
};
use std::convert::Infallible;

type Result<T = Value> = std::result::Result<T, Error>;

pub trait Evaluate {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result;
}

impl Evaluate for Expression {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        match self {
            Self::Atom(atom) => atom.evaluate(state, ctx_size),
            Self::BinaryTerm(term) => term.evaluate(state, ctx_size),
            Self::UnaryTerm(term) => term.evaluate(state, ctx_size),
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
        let ctx_size = self.ctx_size.calc(ctx_size);
        let lhs = self.lhs.evaluate(state, ctx_size)?;
        let rhs = self.rhs.evaluate(state, ctx_size)?;

        Ok(match self.operator {
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

impl Evaluate for UnaryTerm {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let ctx_size_inner = self.ctx_size.calc(ctx_size);
        let mut value = self.expression.evaluate(state, ctx_size_inner)?;

        Ok(match self.operator {
            UnaryOperator::SignNeg => -value,
            UnaryOperator::Not => !value,
            UnaryOperator::Sxt => {
                value.extend_sign(ctx_size);
                value
            }
        })
    }
}

impl Evaluate for Register {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let mut value = state.read_register(&self.ident, self.range)?;
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for Bus {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let mut value = state.read_bus(&self.ident, self.range)?;
        value.extend_zero(ctx_size);
        Ok(value)
    }
}

impl Evaluate for RegisterArray {
    fn evaluate(&self, state: &State, ctx_size: usize) -> Result {
        let idx = self.index.evaluate(state, self.index_ctx_size)?;

        let mut value = state.read_register_array(&self.ident, idx)?;
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
                ConcatPartExpr::Register(reg) => state.read_register(&reg.ident, reg.range),
                ConcatPartExpr::Bus(bus) => state.read_bus(&bus.ident, bus.range),
                ConcatPartExpr::RegisterArray(reg_array) => {
                    let idx = reg_array.index.evaluate(state, reg_array.index_ctx_size)?;
                    state.read_register_array(&reg_array.ident, idx)
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