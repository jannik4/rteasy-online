use crate::{Generate, Result};
use compiler::mir;
use rtcore::program::*;

impl Generate<mir::Declaration<'_>> for Declaration {
    fn generate(declaration: mir::Declaration<'_>) -> Result<Self> {
        match declaration {
            mir::Declaration::Register(declare_register) => {
                Ok(Declaration::Register(DeclareRegister {
                    registers: Generate::generate(declare_register.registers)?,
                }))
            }
            mir::Declaration::Bus(declare_bus) => {
                Ok(Declaration::Bus(DeclareBus { buses: Generate::generate(declare_bus.buses)? }))
            }
            mir::Declaration::Memory(declare_memory) => Ok(Declaration::Memory(DeclareMemory {
                memories: Generate::generate(declare_memory.memories)?,
            })),
            mir::Declaration::RegisterArray(declare_register_array) => {
                Ok(Declaration::RegisterArray(DeclareRegisterArray {
                    register_arrays: Generate::generate(declare_register_array.register_arrays)?,
                }))
            }
        }
    }
}

impl Generate<mir::Memory<'_>> for Memory {
    fn generate(memory: mir::Memory<'_>) -> Result<Self> {
        Ok(Memory {
            ident: memory.ident.node.into(),
            range: MemoryRange {
                address_register: memory.range.address_register.node.into(),
                data_register: memory.range.data_register.node.into(),
            },
        })
    }
}

impl Generate<mir::DeclareRegisterArrayItem<'_>> for DeclareRegisterArrayItem {
    fn generate(declare_register_array_item: mir::DeclareRegisterArrayItem<'_>) -> Result<Self> {
        Ok(DeclareRegisterArrayItem {
            ident: declare_register_array_item.ident.node.into(),
            range: declare_register_array_item.range.map(|s| s.node),
            len: declare_register_array_item.len,
        })
    }
}
