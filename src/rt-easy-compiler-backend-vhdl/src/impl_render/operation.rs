use super::{bit_range::RenderBitRange, Render};
use crate::vhdl;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct RenderOperation<'a, T> {
    pub operation: T,
    // pub vhdl: &'a vhdl::Vhdl<'a>,
    pub _p: std::marker::PhantomData<&'a ()>, // TODO: Remove me
}

macro_rules! gen {
    ($operation:expr /*, $vhdl:expr*/) => {
        RenderOperation {
            operation: $operation, /*, vhdl: $vhdl*/
            _p: std::marker::PhantomData,
        }
    };
}

impl Display for RenderOperation<'_, &vhdl::Operation<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.operation {
            vhdl::Operation::Write(write) => write!(f, "{}", gen!(write /*, &self.vhdl*/)),
            vhdl::Operation::Read(read) => write!(f, "{}", gen!(read /*, &self.vhdl*/)),
            vhdl::Operation::Assignment(assignment) => {
                write!(f, "{}", gen!(assignment /*, &self.vhdl*/))
            }
        }
    }
}

impl Display for RenderOperation<'_, &vhdl::Write<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: get AR/DR from declarations ...
        write!(f, "VHDL_write {};", self.operation.ident.0)
    }
}

impl Display for RenderOperation<'_, &vhdl::Read<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: get AR/DR from declarations ...
        write!(f, "VHDL_read {};", self.operation.ident.0)
    }
}

impl Display for RenderOperation<'_, &vhdl::Assignment<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.operation.lhs {
            vhdl::Lvalue::Register(reg) => {
                let prefix = match reg.kind {
                    vhdl::RegisterKind::Intern => "register",
                    vhdl::RegisterKind::Output => "output",
                };
                write!(f, "{}_{}{}", prefix, reg.ident.0, RenderBitRange(reg.range),)?;
            }
            vhdl::Lvalue::Bus(bus) => {
                let prefix = match bus.kind {
                    vhdl::BusKind::Intern => "bus",
                    vhdl::BusKind::Input => "input",
                };
                write!(f, "{}_{}{}", prefix, bus.ident.0, RenderBitRange(bus.range),)?;
            }
            vhdl::Lvalue::RegisterArray(_lvalue) => todo!(),
            vhdl::Lvalue::ConcatClocked(_lvalue) => todo!(),
            vhdl::Lvalue::ConcatUnclocked(_lvalue) => todo!(),
        }

        write!(f, " <= {};", Render(&self.operation.rhs))?;

        Ok(())
    }
}
