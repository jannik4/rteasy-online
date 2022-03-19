use super::RenderAsRt;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl<P> Display for RenderAsRt<&Concat<P>>
where
    for<'a> RenderAsRt<&'a P>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut parts = self.0.parts.iter();
        write!(f, "{}", RenderAsRt(parts.next().unwrap()))?;
        for part in parts {
            write!(f, ".{}", RenderAsRt(part))?;
        }

        Ok(())
    }
}

impl Display for RenderAsRt<&ConcatPartLvalueClocked> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ConcatPartLvalueClocked::*;
        match &self.0 {
            Register(reg, _) => write!(f, "{}", RenderAsRt(reg)),
            RegisterArray(reg_array, _) => write!(f, "{}", RenderAsRt(reg_array)),
        }
    }
}

impl Display for RenderAsRt<&ConcatPartLvalueUnclocked> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ConcatPartLvalueUnclocked::*;
        match &self.0 {
            Bus(bus, _) => write!(f, "{}", RenderAsRt(bus)),
        }
    }
}

impl Display for RenderAsRt<&ConcatPartExpr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ConcatPartExpr::*;
        match &self.0 {
            Register(reg) => write!(f, "{}", RenderAsRt(reg)),
            Bus(bus) => write!(f, "{}", RenderAsRt(bus)),
            RegisterArray(reg_array) => write!(f, "{}", RenderAsRt(reg_array)),
            Number(number) => write!(f, "{}", RenderAsRt(number)),
        }
    }
}
