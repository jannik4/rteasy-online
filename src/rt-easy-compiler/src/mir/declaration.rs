use super::*;

#[derive(Debug, Clone)]
pub enum Declaration<'s> {
    Register(DeclareRegister<'s>),
    Bus(DeclareBus<'s>),
    Memory(DeclareMemory<'s>),
    RegisterArray(DeclareRegisterArray<'s>),
}

impl Declaration<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Register(n) => n.span,
            Self::Bus(n) => n.span,
            Self::Memory(n) => n.span,
            Self::RegisterArray(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclareRegister<'s> {
    pub registers: Vec<Register<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareBus<'s> {
    pub buses: Vec<Bus<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareMemory<'s> {
    pub memories: Vec<Memory<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Memory<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: MemoryRange<'s>,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryRange<'s> {
    pub address_register: Spanned<Ident<'s>>,
    pub data_register: Spanned<Ident<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArray<'s> {
    pub register_arrays: Vec<DeclareRegisterArrayItem<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArrayItem<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: Option<Spanned<BitRange>>,
    pub len: usize,
}
