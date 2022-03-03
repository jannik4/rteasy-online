use super::bit_range::RenderBitRange;
use crate::vhdl;
use std::fmt::{Display, Formatter, Result};

const ZERO_EXTEND: &str = "zero_extend";
const SIGN_EXTEND: &str = "sign_extend";

#[derive(Debug)]
pub struct RenderExpression<T> {
    pub expression: T,
    pub ctx_size: usize,
}

macro_rules! gen {
    ($expression:expr, $ctx_size:expr) => {
        RenderExpression { expression: $expression, ctx_size: $ctx_size }
    };
}

impl Display for RenderExpression<&vhdl::Expression<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.expression {
            vhdl::Expression::Atom(atom) => write!(f, "{}", gen!(atom, self.ctx_size)),
            vhdl::Expression::BinaryTerm(term) => write!(f, "{}", gen!(&**term, self.ctx_size)),
            vhdl::Expression::UnaryTerm(term) => write!(f, "{}", gen!(&**term, self.ctx_size)),
        }
    }
}

impl Display for RenderExpression<&vhdl::Atom<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.expression {
            vhdl::Atom::Concat(concat) => write!(f, "{}", gen!(concat, self.ctx_size)),
            vhdl::Atom::Register(register) => write!(f, "{}", gen!(register, self.ctx_size)),
            vhdl::Atom::Bus(bus) => write!(f, "{}", gen!(bus, self.ctx_size)),
            vhdl::Atom::RegisterArray(reg_array) => write!(f, "{}", gen!(reg_array, self.ctx_size)),
            vhdl::Atom::Number(number) => write!(f, "{}", gen!(number, self.ctx_size)),
        }
    }
}

impl Display for RenderExpression<&vhdl::BinaryTerm<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ctx_size_inner = self.expression.ctx_size.calc(self.ctx_size);
        let (extend, op) = binary_operator(self.expression.operator);

        if let Some(extend) = extend {
            write!(f, "{}(", extend)?;
        }

        write!(
            f,
            "{}({}, {})",
            op,
            gen!(&self.expression.lhs, ctx_size_inner),
            gen!(&self.expression.rhs, ctx_size_inner)
        )?;

        if let Some(_extend) = extend {
            write!(f, ", {})", self.ctx_size)?;
        }

        Ok(())
    }
}

impl Display for RenderExpression<&vhdl::UnaryTerm<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ctx_size_inner = self.expression.ctx_size.calc(self.ctx_size);
        let (extend, op) = unary_operator(self.expression.operator);

        if let Some(extend) = extend {
            write!(f, "{}(", extend)?;
        }

        write!(f, "{}({})", op, gen!(&self.expression.expression, ctx_size_inner))?;

        if let Some(_extend) = extend {
            write!(f, ", {})", self.ctx_size)?;
        }

        Ok(())
    }
}

impl Display for RenderExpression<&vhdl::Register<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = match self.expression.kind {
            vhdl::RegisterKind::Intern => "register",
            vhdl::RegisterKind::Output => "output",
        };

        write!(
            f,
            "{}({}_{}{}, {})",
            ZERO_EXTEND,
            prefix,
            self.expression.ident.0,
            RenderBitRange(self.expression.range),
            self.ctx_size
        )
    }
}

impl Display for RenderExpression<&vhdl::Bus<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = match self.expression.kind {
            vhdl::BusKind::Intern => "bus",
            vhdl::BusKind::Input => "input",
        };

        write!(
            f,
            "{}({}_{}{}, {})",
            ZERO_EXTEND,
            prefix,
            self.expression.ident.0,
            RenderBitRange(self.expression.range),
            self.ctx_size
        )
    }
}

impl Display for RenderExpression<&vhdl::RegisterArray<'_>> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}

impl Display for RenderExpression<&vhdl::Number> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}(\"{}\", {})", ZERO_EXTEND, self.expression.value.as_bin(true), self.ctx_size)
    }
}

impl Display for RenderExpression<&vhdl::Concat<vhdl::ConcatPartExpr<'_>>> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}

fn binary_operator(op: vhdl::BinaryOperator) -> (Option<&'static str>, &'static str) {
    match op {
        vhdl::BinaryOperator::Eq => (Some(ZERO_EXTEND), "f_eq"),
        vhdl::BinaryOperator::Ne => (Some(ZERO_EXTEND), "f_ne"),
        vhdl::BinaryOperator::Le => (Some(ZERO_EXTEND), "f_le"),
        vhdl::BinaryOperator::Lt => (Some(ZERO_EXTEND), "f_lt"),
        vhdl::BinaryOperator::Ge => (Some(ZERO_EXTEND), "f_ge"),
        vhdl::BinaryOperator::Gt => (Some(ZERO_EXTEND), "f_gt"),
        vhdl::BinaryOperator::Add => (None, "f_add"),
        vhdl::BinaryOperator::Sub => (None, "f_sub"),
        vhdl::BinaryOperator::And => (None, "f_and"),
        vhdl::BinaryOperator::Nand => (None, "f_nand"),
        vhdl::BinaryOperator::Or => (None, "f_or"),
        vhdl::BinaryOperator::Nor => (None, "f_nor"),
        vhdl::BinaryOperator::Xor => (None, "f_xor"),
    }
}

fn unary_operator(op: vhdl::UnaryOperator) -> (Option<&'static str>, &'static str) {
    match op {
        vhdl::UnaryOperator::Sign | vhdl::UnaryOperator::Neg => (None, "f_neg"),
        vhdl::UnaryOperator::Not => (None, "f_not"),
        vhdl::UnaryOperator::Sxt => (Some(SIGN_EXTEND), "f_sxt"),
    }
}
