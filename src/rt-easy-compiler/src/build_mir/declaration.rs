use super::{expression::BuildExpr, Result};
use crate::mir::*;
use crate::symbols::Symbols;
use rtast as ast;

pub fn build<'s>(
    declaration: ast::Declaration<'s>,
    symbols: &Symbols<'_>,
) -> Result<Declaration<'s>> {
    match declaration {
        ast::Declaration::Register(declare_register) => {
            Ok(Declaration::Register(DeclareRegister {
                registers: declare_register
                    .registers
                    .into_iter()
                    .map(|reg| Ok(Register::build(reg, symbols)?.inner))
                    .collect::<Result<_>>()?,
                span: declare_register.span,
            }))
        }
        ast::Declaration::Bus(declare_bus) => Ok(Declaration::Bus(DeclareBus {
            buses: declare_bus
                .buses
                .into_iter()
                .map(|bus| Ok(Bus::build(bus, symbols)?.inner))
                .collect::<Result<_>>()?,
            span: declare_bus.span,
        })),
        ast::Declaration::Memory(declare_memory) => Ok(Declaration::Memory(DeclareMemory {
            memories: declare_memory
                .memories
                .into_iter()
                .map(|mem| Memory {
                    ident: mem.ident,
                    range: MemoryRange {
                        address_register: mem.range.address_register,
                        data_register: mem.range.data_register,
                        span: mem.range.span,
                    },
                    span: mem.span,
                })
                .collect(),
            span: declare_memory.span,
        })),
        ast::Declaration::RegisterArray(declare_reg_array) => {
            Ok(Declaration::RegisterArray(DeclareRegisterArray {
                register_arrays: declare_reg_array
                    .register_arrays
                    .into_iter()
                    .map(|declare_register_array_item| DeclareRegisterArrayItem {
                        ident: declare_register_array_item.ident,
                        range: declare_register_array_item.range,
                        len: declare_register_array_item.len,
                    })
                    .collect(),
                span: declare_reg_array.span,
            }))
        }
    }
}
