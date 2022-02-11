use crate::vhdl;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct RenderBitRange(pub Option<vhdl::BitRange>);

impl Display for RenderBitRange {
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
