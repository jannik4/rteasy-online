use super::*;

#[derive(Debug)]
pub enum Declaration {
    Register(DeclareRegister),
    Bus(DeclareBus),
    Memory(DeclareMemory),
    RegisterArray(DeclareRegisterArray),
}

#[derive(Debug)]
pub struct DeclareRegister {
    pub registers: Vec<Register>,
}

#[derive(Debug)]
pub struct DeclareBus {
    pub buses: Vec<Bus>,
}

#[derive(Debug)]
pub struct DeclareMemory {
    pub memories: Vec<Memory>,
}

#[derive(Debug)]
pub struct Memory {
    pub ident: Ident,
    pub range: MemoryRange,
}

#[derive(Debug, Clone)]
pub struct MemoryRange {
    pub address_register: Ident,
    pub data_register: Ident,
}

#[derive(Debug)]
pub struct DeclareRegisterArray {
    pub register_arrays: Vec<DeclareRegisterArrayItem>,
}

#[derive(Debug)]
pub struct DeclareRegisterArrayItem {
    pub ident: Ident,
    pub range: Option<BitRange>,
    pub len: usize,
}
