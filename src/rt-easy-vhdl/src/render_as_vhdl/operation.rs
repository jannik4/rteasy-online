use super::RenderAsVhdl;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<(&Operation, usize)> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (op, idx) = self.0;
        match op {
            Operation::Write(write) => write!(f, "{}", RenderAsVhdl(write)),
            Operation::Read(read) => write!(f, "{}", RenderAsVhdl(read)),
            Operation::Assignment(assignment) => {
                write!(f, "{}", RenderAsVhdl((assignment, idx)))
            }
        }
    }
}

impl Display for RenderAsVhdl<&Write> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "memory_{}(to_integer({})) <= {};",
            self.0.memory.0,
            RenderAsVhdl(&self.0.ar),
            RenderAsVhdl(&self.0.dr),
        )
    }
}

impl Display for RenderAsVhdl<&Read> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} <= memory_{}(to_integer({}));",
            RenderAsVhdl(&self.0.dr),
            self.0.memory.0,
            RenderAsVhdl(&self.0.ar),
        )
    }
}

impl Display for RenderAsVhdl<(&Assignment, usize)> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (assignment, idx) = self.0;
        match &assignment.lhs {
            Lvalue::Register(reg) => {
                write!(f, "{} <= {};", RenderAsVhdl(reg), RenderAsVhdl(&assignment.rhs))
            }
            Lvalue::Bus(bus) => {
                write!(f, "{} <= {};", RenderAsVhdl(bus), RenderAsVhdl(&assignment.rhs))
            }
            Lvalue::RegisterArray(reg_array) => {
                write!(f, "{} <= {};", RenderAsVhdl(reg_array), RenderAsVhdl(&assignment.rhs))
            }
            Lvalue::ConcatClocked(concat) => {
                write!(f, "tmp_c_{} := {};", idx, RenderAsVhdl(&assignment.rhs))?;

                let mut pos = 0;
                for part in concat.parts.iter().rev() {
                    let size = match part {
                        ConcatPartLvalueClocked::Register(register, size) => {
                            write!(f, " {}", RenderAsVhdl(register))?;
                            size
                        }
                        ConcatPartLvalueClocked::RegisterArray(reg_array, size) => {
                            write!(f, " {}", RenderAsVhdl(reg_array))?;
                            size
                        }
                    };

                    let range = BitRange::Downto(pos + size - 1, pos);
                    write!(f, " <= tmp_c_{}{};", idx, RenderAsVhdl(range))?;

                    pos += size;
                }

                Ok(())
            }
            Lvalue::ConcatUnclocked(concat) => {
                write!(f, "tmp_c_{} := {};", idx, RenderAsVhdl(&assignment.rhs))?;

                let mut pos = 0;
                for part in concat.parts.iter().rev() {
                    let size = match part {
                        ConcatPartLvalueUnclocked::Bus(bus, size) => {
                            write!(f, " {}", RenderAsVhdl(bus))?;
                            size
                        }
                    };

                    let range = BitRange::Downto(pos + size - 1, pos);
                    write!(f, " <= tmp_c_{}{};", idx, RenderAsVhdl(range))?;

                    pos += size;
                }

                Ok(())
            }
        }
    }
}
