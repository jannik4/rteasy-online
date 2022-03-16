use super::*;
use crate::util;
use std::fmt::{Display, Formatter, Result};

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for statement in &self.statements {
            write!(f, "{}\n", statement)?;
        }

        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.label {
            Some(label) => write!(f, "{}:\n", label.node.0)?,
            None => write!(f, "_:\n")?,
        }

        for step in self.steps.node.as_slice() {
            write!(f, "    {}\n", step)?;
        }

        Ok(())
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.criteria.is_empty() {
            let mut criteria = self.criteria.iter();
            write!(f, "{}", criteria.next().unwrap())?;
            for criterion in criteria {
                write!(f, ",{}", criterion)?;
            }
            write!(f, " => ")?;
        }

        write!(f, "{}", self.operation)?;

        Ok(())
    }
}

impl Display for Criterion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Criterion::True(id) => write!(f, "k{}", id.0),
            Criterion::False(id) => write!(f, "!k{}", id.0),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use OperationKind::*;
        match &self.kind {
            EvalCriterion(op) => write!(f, "{}", op),
            EvalCriterionGroup(op) => write!(f, "{}", op),
            Nop(op) => write!(f, "{}", op),
            Goto(op) => write!(f, "{}", op),
            Write(op) => write!(f, "{}", op),
            Read(op) => write!(f, "{}", op),
            Assignment(op) => write!(f, "{}", op),
            Assert(op) => write!(f, "{}", op),
        }
    }
}

impl Display for EvalCriterion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "k{} := {}", self.criterion_id.0, self.condition)
    }
}

impl Display for EvalCriterionGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut elements = self.0.iter();

        if let Some(e) = elements.next() {
            write!(f, "{}", e)?;
        }
        for e in elements {
            write!(f, ", {}", e)?;
        }

        Ok(())
    }
}

impl Display for Nop {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "nop")
    }
}

impl Display for Goto {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "goto {}", self.label.0)
    }
}

impl Display for Write {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "write {}", self.ident.0)
    }
}

impl Display for Read {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "read {}", self.ident.0)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} <- {}", self.lhs, self.rhs)
    }
}

impl Display for Lvalue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Lvalue::*;
        match self {
            Register(lvalue) => write!(f, "{}", lvalue),
            Bus(lvalue) => write!(f, "{}", lvalue),
            RegisterArray(lvalue) => write!(f, "{}", lvalue),
            ConcatClocked(lvalue) => write!(f, "{}", lvalue),
            ConcatUnclocked(lvalue) => write!(f, "{}", lvalue),
        }
    }
}

impl Display for Assert {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "assert {}", self.condition)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ExpressionKind::*;
        match &self.kind {
            Atom(expr) => write!(f, "{}", expr),
            BinaryTerm(expr) => write!(f, "{}", expr),
            UnaryTerm(expr) => write!(f, "{}", expr),
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Atom::*;
        match self {
            Concat(atom) => write!(f, "{}", atom),
            Register(atom) => write!(f, "{}", atom),
            Bus(atom) => write!(f, "{}", atom),
            RegisterArray(atom) => write!(f, "{}", atom),
            Number(atom) => write!(f, "{}", atom),
        }
    }
}

impl Display for BinaryTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if util::parentheses_binary_left(
            self.operator.precedence(),
            self.lhs.precedence(),
            self.operator.associativity(),
        ) {
            write!(f, "({})", self.lhs)?;
        } else {
            write!(f, "{}", self.lhs)?;
        }

        write!(f, " {} ", self.operator)?;

        if util::parentheses_binary_right(
            self.operator.precedence(),
            self.rhs.precedence(),
            self.operator.associativity(),
        ) {
            write!(f, "({})", self.rhs)?;
        } else {
            write!(f, "{}", self.rhs)?;
        }

        Ok(())
    }
}

impl Display for UnaryTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ws = match self.operator {
            UnaryOperator::Sign => "",
            UnaryOperator::Neg | UnaryOperator::Not | UnaryOperator::Sxt => " ",
        };

        if util::parentheses_unary(self.operator.precedence(), self.expression.precedence()) {
            write!(f, "{}{}({})", self.operator, ws, self.expression)
        } else {
            write!(f, "{}{}{}", self.operator, ws, self.expression)
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.range {
            Some(range) => write!(f, "{}{}", self.ident.0, range),
            None => write!(f, "{}", self.ident.0),
        }
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.range {
            Some(range) => write!(f, "{}{}", self.ident.0, range),
            None => write!(f, "{}", self.ident.0),
        }
    }
}

impl Display for RegisterArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}[{}]", self.ident.0, self.index)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.kind {
            NumberKind::BitString => write!(f, "\"{}\"", self.value.as_bin(true)),
            NumberKind::Binary => write!(f, "0b{}", self.value.as_bin(false)),
            NumberKind::Decimal => write!(f, "{}", self.value.as_dec()),
            NumberKind::Hexadecimal => write!(f, "0x{}", self.value.as_hex()),
        }
    }
}

impl<P> Display for Concat<P>
where
    P: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.parts.is_empty() {
            let mut parts = self.parts.iter();
            write!(f, "{}", parts.next().unwrap())?;
            for part in parts {
                write!(f, ".{}", part)?;
            }
        }

        Ok(())
    }
}

impl Display for ConcatPartLvalueClocked {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ConcatPartLvalueClocked::*;
        match self {
            Register(reg, _) => write!(f, "{}", reg),
            RegisterArray(reg_array, _) => write!(f, "{}", reg_array),
        }
    }
}

impl Display for ConcatPartLvalueUnclocked {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ConcatPartLvalueUnclocked::*;
        match self {
            Bus(bus, _) => write!(f, "{}", bus),
        }
    }
}

impl Display for ConcatPartExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ConcatPartExpr::*;
        match self {
            Register(reg) => write!(f, "{}", reg),
            Bus(bus) => write!(f, "{}", bus),
            RegisterArray(reg_array) => write!(f, "{}", reg_array),
            Number(number) => write!(f, "{}", number),
        }
    }
}
