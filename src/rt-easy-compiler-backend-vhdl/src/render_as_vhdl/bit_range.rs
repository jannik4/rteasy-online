use super::RenderAsVhdl;
use crate::vhdl::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<Option<BitRange>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            Some(bit_range) => {
                let (msb, lsb) = bit_range.msb_lsb();
                if msb > lsb {
                    write!(f, "({} DOWNTO {})", msb, lsb)
                } else {
                    write!(f, "({} TO {})", msb, lsb)
                }
            }
            None => Ok(()),
        }
    }
}
