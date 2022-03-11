use crate::vhdl::*;
use compiler::mir;

pub fn generate_declarations<'s>(mir_declarations: &[mir::Declaration<'s>]) -> Declarations<'s> {
    let mut declarations = Declarations {
        registers: Vec::new(),
        buses: Vec::new(),
        memories: Vec::new(),
        register_arrays: Vec::new(),
    };

    for declaration in mir_declarations {
        match declaration {
            mir::Declaration::Register(declaration) => {
                for register in &declaration.registers {
                    declarations.registers.push((
                        register.ident.node,
                        generate_bit_range(register.range.map(|s| s.node)),
                        register.kind,
                    ));
                }
            }
            mir::Declaration::Bus(declaration) => {
                for bus in &declaration.buses {
                    declarations.buses.push((
                        bus.ident.node,
                        generate_bit_range(bus.range.map(|s| s.node)),
                        bus.kind,
                    ));
                }
            }
            mir::Declaration::Memory(_) => {
                todo!()
            }
            mir::Declaration::RegisterArray(_) => {
                todo!()
            }
        }
    }

    declarations
}

fn generate_bit_range(range: Option<mir::BitRange>) -> BitRange {
    match range {
        Some(mir::BitRange { msb, lsb: Some(lsb) }) => {
            if msb >= lsb {
                BitRange::Downto(msb, lsb)
            } else {
                BitRange::To(msb, lsb)
            }
        }
        Some(mir::BitRange { msb, lsb: None }) => BitRange::Downto(msb, msb),
        None => BitRange::Downto(0, 0),
    }
}
