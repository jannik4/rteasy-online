use crate::{Generate, Result};
use compiler::mir;
use rtcore::program::*;

impl<P1, P2> Generate<mir::Concat<P1>> for Concat<P2>
where
    P2: Generate<P1>,
{
    fn generate(concat: mir::Concat<P1>) -> Result<Self> {
        Ok(Concat { parts: Generate::generate(concat.parts)? })
    }
}

impl Generate<mir::ConcatPartLvalueClocked<'_>> for ConcatPartLvalueClocked {
    fn generate(part: mir::ConcatPartLvalueClocked<'_>) -> Result<Self> {
        match part {
            mir::ConcatPartLvalueClocked::Register(reg, size) => {
                Ok(ConcatPartLvalueClocked::Register(Generate::generate(reg)?, size))
            }
            mir::ConcatPartLvalueClocked::RegisterArray(reg_array, size) => {
                Ok(ConcatPartLvalueClocked::RegisterArray(Generate::generate(reg_array)?, size))
            }
        }
    }
}

impl Generate<mir::ConcatPartLvalueUnclocked<'_>> for ConcatPartLvalueUnclocked {
    fn generate(part: mir::ConcatPartLvalueUnclocked<'_>) -> Result<Self> {
        match part {
            mir::ConcatPartLvalueUnclocked::Bus(bus, size) => {
                Ok(ConcatPartLvalueUnclocked::Bus(Generate::generate(bus)?, size))
            }
        }
    }
}

impl Generate<mir::ConcatPartExpr<'_>> for ConcatPartExpr {
    fn generate(part: mir::ConcatPartExpr<'_>) -> Result<Self> {
        match part {
            mir::ConcatPartExpr::Register(reg) => {
                Ok(ConcatPartExpr::Register(Generate::generate(reg)?))
            }
            mir::ConcatPartExpr::Bus(bus) => Ok(ConcatPartExpr::Bus(Generate::generate(bus)?)),
            mir::ConcatPartExpr::RegisterArray(reg_array) => {
                Ok(ConcatPartExpr::RegisterArray(Generate::generate(reg_array)?))
            }
            mir::ConcatPartExpr::Number(number) => Ok(ConcatPartExpr::Number(number)),
        }
    }
}
