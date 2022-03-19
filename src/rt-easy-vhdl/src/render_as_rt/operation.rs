use super::RenderAsRt;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsRt<&Operation> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Operation::*;
        match &self.0 {
            Write(op) => write!(f, "{}", RenderAsRt(op)),
            Read(op) => write!(f, "{}", RenderAsRt(op)),
            Assignment(op) => write!(f, "{}", RenderAsRt(op)),
        }
    }
}

impl Display for RenderAsRt<&Write> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "write {}", self.0.memory.0)
    }
}

impl Display for RenderAsRt<&Read> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "read {}", self.0.memory.0)
    }
}

impl Display for RenderAsRt<&Assignment> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} <- {}", RenderAsRt(&self.0.lhs), RenderAsRt(&self.0.rhs))
    }
}

impl Display for RenderAsRt<&Lvalue> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Lvalue::*;
        match &self.0 {
            Register(lvalue) => write!(f, "{}", RenderAsRt(lvalue)),
            Bus(lvalue) => write!(f, "{}", RenderAsRt(lvalue)),
            RegisterArray(lvalue) => write!(f, "{}", RenderAsRt(lvalue)),
            ConcatClocked(lvalue) => write!(f, "{}", RenderAsRt(lvalue)),
            ConcatUnclocked(lvalue) => write!(f, "{}", RenderAsRt(lvalue)),
        }
    }
}
