use super::RenderAsVhdl;
use crate::vhdl::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<&Operation<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.0 {
            Operation::Write(write) => write!(f, "{}", RenderAsVhdl(write)),
            Operation::Read(read) => write!(f, "{}", RenderAsVhdl(read)),
            Operation::Assignment(assignment) => write!(f, "{}", RenderAsVhdl(assignment)),
        }
    }
}

impl Display for RenderAsVhdl<&Write<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: get AR/DR from declarations ...
        write!(f, "VHDL_write {};", self.0.ident.0)
    }
}

impl Display for RenderAsVhdl<&Read<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: get AR/DR from declarations ...
        write!(f, "VHDL_read {};", self.0.ident.0)
    }
}

impl Display for RenderAsVhdl<&Assignment<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.0.lhs {
            Lvalue::Register(reg) => write!(f, "{}", RenderAsVhdl(reg))?,
            Lvalue::Bus(bus) => write!(f, "{}", RenderAsVhdl(bus))?,
            Lvalue::RegisterArray(_lvalue) => todo!(),
            Lvalue::ConcatClocked(_lvalue) => todo!(),
            Lvalue::ConcatUnclocked(_lvalue) => todo!(),
        }

        write!(f, " <= {};", RenderAsVhdl(&self.0.rhs))?;

        Ok(())
    }
}
