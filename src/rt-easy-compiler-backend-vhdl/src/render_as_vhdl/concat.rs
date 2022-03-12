use super::RenderAsVhdl;
use crate::vhdl::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<&Concat<ConcatPartExpr<'_>>> {
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

impl Display for RenderAsVhdl<&ConcatPartExpr<'_>> {
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

// impl Display for RenderAsVhdl<&Concat<ConcatPartLvalueClocked<'_>>> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut parts = self.0.parts.iter();
//         write!(f, "{}", RenderAsVhdl(parts.next().unwrap()))?;
//         for part in parts {
//             write!(f, " & {}", RenderAsVhdl(part))?;
//         }
//
//         Ok(())
//     }
// }
//
// impl Display for RenderAsVhdl<&ConcatPartLvalueClocked<'_>> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self.0 {
//             ConcatPartLvalueClocked::Register(register, _) => {
//                 write!(f, "{}", RenderAsVhdl(register))
//             }
//             ConcatPartLvalueClocked::RegisterArray(reg_array, _) => {
//                 write!(f, "{}", RenderAsVhdl(reg_array))
//             }
//         }
//     }
// }
//
// impl Display for RenderAsVhdl<&Concat<ConcatPartLvalueUnclocked<'_>>> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut parts = self.0.parts.iter();
//         write!(f, "{}", RenderAsVhdl(parts.next().unwrap()))?;
//         for part in parts {
//             write!(f, " & {}", RenderAsVhdl(part))?;
//         }
//
//         Ok(())
//     }
// }
//
// impl Display for RenderAsVhdl<&ConcatPartLvalueUnclocked<'_>> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self.0 {
//             ConcatPartLvalueUnclocked::Bus(bus, _) => write!(f, "{}", RenderAsVhdl(bus)),
//         }
//     }
// }
