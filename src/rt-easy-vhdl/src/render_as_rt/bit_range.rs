use super::RenderAsRt;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsRt<Option<BitRange>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            Some(bit_range) => write!(f, "{}", RenderAsRt(bit_range)),
            None => Ok(()),
        }
    }
}

impl Display for RenderAsRt<BitRange> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (BitRange::Downto(a, b) | BitRange::To(a, b)) = self.0;
        if a == b {
            write!(f, "({})", a)
        } else {
            write!(f, "({}:{})", a, b)
        }
    }
}
