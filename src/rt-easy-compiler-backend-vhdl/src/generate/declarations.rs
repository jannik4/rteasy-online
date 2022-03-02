use super::expression::{generate_bus, generate_register};
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
                    declarations.registers.push(generate_register(register));
                }
            }
            mir::Declaration::Bus(declaration) => {
                for bus in &declaration.buses {
                    declarations.buses.push(generate_bus(bus));
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
