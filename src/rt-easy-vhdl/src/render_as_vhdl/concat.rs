use super::RenderAsVhdl;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<&Concat<ConcatPartExpr>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(")?;
        let mut parts = self.0.parts.iter();
        write!(f, "{}", RenderAsVhdl(parts.next().unwrap()))?;
        for part in parts {
            write!(f, " & {}", RenderAsVhdl(part))?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl Display for RenderAsVhdl<&ConcatPartExpr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            ConcatPartExpr::Register(register) => write!(f, "{}", RenderAsVhdl(register)),
            ConcatPartExpr::Bus(bus) => write!(f, "{}", RenderAsVhdl(bus)),
            ConcatPartExpr::RegisterArray(reg_array) => {
                write!(f, "{}", RenderAsVhdl(reg_array))
            }
            ConcatPartExpr::Number(number) => write!(f, "{}", RenderAsVhdl(number)),
        }
    }
}
