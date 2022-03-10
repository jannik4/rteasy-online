use super::{bit_range::RenderBitRange, Render};
use crate::vhdl;
use std::fmt::{Display, Formatter, Result};

impl Display for Render<&vhdl::Expression<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0.extend_to {
            vhdl::Extend::Zero(size) => {
                write!(f, "zero_extend({}, {})", Render(&self.0.kind), size)
            }
            vhdl::Extend::Sign(size) => {
                write!(f, "sign_extend({}, {})", Render(&self.0.kind), size)
            }
        }
    }
}

impl Display for Render<&vhdl::ExpressionKind<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            vhdl::ExpressionKind::Atom(atom) => write!(f, "{}", Render(atom)),
            vhdl::ExpressionKind::BinaryTerm(term) => write!(f, "{}", Render(&**term)),
            vhdl::ExpressionKind::UnaryTerm(term) => write!(f, "{}", Render(&**term)),
        }
    }
}

impl Display for Render<&vhdl::Atom<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            vhdl::Atom::Concat(concat) => write!(f, "{}", Render(concat)),
            vhdl::Atom::Register(register) => write!(f, "{}", Render(register)),
            vhdl::Atom::Bus(bus) => write!(f, "{}", Render(bus)),
            vhdl::Atom::RegisterArray(reg_array) => write!(f, "{}", Render(reg_array)),
            vhdl::Atom::Number(number) => write!(f, "{}", Render(number)),
        }
    }
}

impl Display for Render<&vhdl::BinaryTerm<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}({}, {})",
            binary_operator(self.0.operator),
            Render(&self.0.lhs),
            Render(&self.0.rhs),
        )?;

        Ok(())
    }
}

impl Display for Render<&vhdl::UnaryTerm<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}({})", unary_operator(self.0.operator), Render(&self.0.expression))?;

        Ok(())
    }
}

impl Display for Render<&vhdl::Register<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = match self.0.kind {
            vhdl::RegisterKind::Intern => "register",
            vhdl::RegisterKind::Output => "output",
        };

        write!(f, "{}_{}{}", prefix, self.0.ident.0, RenderBitRange(self.0.range))
    }
}

impl Display for Render<&vhdl::Bus<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = match self.0.kind {
            vhdl::BusKind::Intern => "bus",
            vhdl::BusKind::Input => "input",
        };

        write!(f, "{}_{}{}", prefix, self.0.ident.0, RenderBitRange(self.0.range))
    }
}

impl Display for Render<&vhdl::RegisterArray<'_>> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}

impl Display for Render<&vhdl::Number> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\"{}\"", self.0.value.as_bin(true))
    }
}

impl Display for Render<&vhdl::Concat<vhdl::ConcatPartExpr<'_>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(")?;
        let mut parts = self.0.parts.iter();
        write!(f, "{}", Render(parts.next().unwrap()))?;
        for part in parts {
            write!(f, " & {}", Render(part))?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl Display for Render<&vhdl::ConcatPartExpr<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            vhdl::ConcatPartExpr::Register(register) => write!(f, "{}", Render(register)),
            vhdl::ConcatPartExpr::Bus(bus) => write!(f, "{}", Render(bus)),
            vhdl::ConcatPartExpr::RegisterArray(reg_array) => write!(f, "{}", Render(reg_array)),
            vhdl::ConcatPartExpr::Number(number) => write!(f, "{}", Render(number)),
        }
    }
}

fn binary_operator(op: vhdl::BinaryOperator) -> &'static str {
    match op {
        vhdl::BinaryOperator::Eq => "f_eq",
        vhdl::BinaryOperator::Ne => "f_ne",
        vhdl::BinaryOperator::Le => "f_le",
        vhdl::BinaryOperator::Lt => "f_lt",
        vhdl::BinaryOperator::Ge => "f_ge",
        vhdl::BinaryOperator::Gt => "f_gt",
        vhdl::BinaryOperator::Add => "f_add",
        vhdl::BinaryOperator::Sub => "f_sub",
        vhdl::BinaryOperator::And => "f_and",
        vhdl::BinaryOperator::Nand => "f_nand",
        vhdl::BinaryOperator::Or => "f_or",
        vhdl::BinaryOperator::Nor => "f_nor",
        vhdl::BinaryOperator::Xor => "f_xor",
    }
}

fn unary_operator(op: vhdl::UnaryOperator) -> &'static str {
    match op {
        vhdl::UnaryOperator::Sign | vhdl::UnaryOperator::Neg => "f_neg",
        vhdl::UnaryOperator::Not => "f_not",
        vhdl::UnaryOperator::Sxt => "f_sxt",
    }
}
