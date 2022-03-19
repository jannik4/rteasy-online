use super::RenderAsVhdl;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<Option<BitRange>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            Some(bit_range) => write!(f, "{}", RenderAsVhdl(bit_range)),
            None => Ok(()),
        }
    }
}

impl Display for RenderAsVhdl<BitRange> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            BitRange::Downto(a, b) => write!(f, "({} DOWNTO {})", a, b),
            BitRange::To(a, b) => write!(f, "({} TO {})", a, b),
        }
    }
}
