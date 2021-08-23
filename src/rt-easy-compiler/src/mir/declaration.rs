use super::*;

#[derive(Debug, Clone)]
pub enum Declaration<'s> {
    Register(DeclareRegister<'s>),
    Bus(DeclareBus<'s>),
    Memory(DeclareMemory<'s>),
    RegisterArray(DeclareRegisterArray<'s>),
}

#[derive(Debug, Clone)]
pub struct DeclareRegister<'s> {
    pub registers: Vec<Register<'s>>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct DeclareBus<'s> {
    pub buses: Vec<Bus<'s>>,
}

#[derive(Debug, Clone)]
pub struct DeclareMemory<'s> {
    pub memories: Vec<Memory<'s>>,
}

#[derive(Debug, Clone)]
pub struct Memory<'s> {
    pub ident: Ident<'s>,
    pub range: MemoryRange<'s>,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryRange<'s> {
    pub address_register: Ident<'s>,
    pub data_register: Ident<'s>,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArray<'s> {
    pub register_arrays: Vec<DeclareRegisterArrayItem<'s>>,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArrayItem<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub len: usize,
}
