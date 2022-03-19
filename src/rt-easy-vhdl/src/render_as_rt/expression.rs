use super::RenderAsRt;
use crate::*;
use rtcore::util;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsRt<&Expression> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", RenderAsRt(&self.0.kind))
    }
}

impl Display for RenderAsRt<&ExpressionKind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ExpressionKind::*;
        match &self.0 {
            Atom(expr) => write!(f, "{}", RenderAsRt(expr)),
            BinaryTerm(expr) => write!(f, "{}", RenderAsRt(&**expr)),
            UnaryTerm(expr) => write!(f, "{}", RenderAsRt(&**expr)),
        }
    }
}

impl Display for RenderAsRt<&Atom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Atom::*;
        match &self.0 {
            Concat(atom) => write!(f, "{}", RenderAsRt(atom)),
            Register(atom) => write!(f, "{}", RenderAsRt(atom)),
            Bus(atom) => write!(f, "{}", RenderAsRt(atom)),
            RegisterArray(atom) => write!(f, "{}", RenderAsRt(atom)),
            Number(atom) => write!(f, "{}", RenderAsRt(atom)),
        }
    }
}

impl Display for RenderAsRt<&BinaryTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if util::parentheses_binary_left(
            self.0.operator.precedence(),
            precedence(&self.0.lhs),
            self.0.operator.associativity(),
        ) {
            write!(f, "({})", RenderAsRt(&self.0.lhs))?;
        } else {
            write!(f, "{}", RenderAsRt(&self.0.lhs))?;
        }

        write!(f, " {} ", self.0.operator)?;

        if util::parentheses_binary_right(
            self.0.operator.precedence(),
            precedence(&self.0.rhs),
            self.0.operator.associativity(),
        ) {
            write!(f, "({})", RenderAsRt(&self.0.rhs))?;
        } else {
            write!(f, "{}", RenderAsRt(&self.0.rhs))?;
        }

        Ok(())
    }
}

impl Display for RenderAsRt<&UnaryTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ws = match self.0.operator {
            UnaryOperator::Sign => "",
            UnaryOperator::Neg | UnaryOperator::Not | UnaryOperator::Sxt => " ",
        };

        if util::parentheses_unary(self.0.operator.precedence(), precedence(&self.0.expression)) {
            write!(f, "{}{}({})", self.0.operator, ws, RenderAsRt(&self.0.expression))
        } else {
            write!(f, "{}{}{}", self.0.operator, ws, RenderAsRt(&self.0.expression))
        }
    }
}

impl Display for RenderAsRt<&Register> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.0.ident, RenderAsRt(self.0.range))
    }
}

impl Display for RenderAsRt<&Bus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.0.ident, RenderAsRt(self.0.range))
    }
}

impl Display for RenderAsRt<&RegisterArray> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}[{}]", self.0.ident, RenderAsRt(&*self.0.index))
    }
}

impl Display for RenderAsRt<&Number> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0.kind.0 {
            NumberKind::BitString => write!(f, "\"{}\"", self.0.value.as_bin(true)),
            NumberKind::Binary => write!(f, "0b{}", self.0.value.as_bin(false)),
            NumberKind::Decimal => write!(f, "{}", self.0.value.as_dec()),
            NumberKind::Hexadecimal => write!(f, "0x{}", self.0.value.as_hex()),
        }
    }
}

fn precedence(expression: &Expression) -> u32 {
    match &expression.kind {
        ExpressionKind::Atom(_) => u32::MAX,
        ExpressionKind::BinaryTerm(binary) => binary.operator.precedence(),
        ExpressionKind::UnaryTerm(unary) => unary.operator.precedence(),
    }
}
