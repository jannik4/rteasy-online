use crate::gen_ident;
use compiler::mir;
use rtvhdl::*;

pub fn generate_declarations<'s>(mir_declarations: &[mir::Declaration<'s>]) -> Declarations {
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
                        gen_ident(register.ident.node),
                        generate_bit_range(register.range.map(|s| s.node)),
                        register.kind,
                    ));
                }
            }
            mir::Declaration::Bus(declaration) => {
                for bus in &declaration.buses {
                    declarations.buses.push((
                        gen_ident(bus.ident.node),
                        generate_bit_range(bus.range.map(|s| s.node)),
                        bus.kind,
                    ));
                }
            }
            mir::Declaration::Memory(declaration) => {
                for memory in &declaration.memories {
                    let (ar_name, ar_range, ar_kind) = declarations
                        .registers
                        .iter()
                        .find(|(name, _, _)| name.0 == memory.range.address_register.node.0)
                        .unwrap();
                    let (dr_name, dr_range, dr_kind) = declarations
                        .registers
                        .iter()
                        .find(|(name, _, _)| name.0 == memory.range.data_register.node.0)
                        .unwrap();

                    declarations.memories.push((
                        gen_ident(memory.ident.node),
                        (ar_name.clone(), *ar_range, *ar_kind),
                        (dr_name.clone(), *dr_range, *dr_kind),
                    ));
                }
            }
            mir::Declaration::RegisterArray(declaration) => {
                for register_array in &declaration.register_arrays {
                    declarations.register_arrays.push((
                        gen_ident(register_array.ident.node),
                        generate_bit_range(register_array.range.map(|s| s.node)),
                        register_array.len,
                    ));
                }
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
