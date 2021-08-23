use std::ops::Range;

pub use crate::common::*;
pub use either::Either;

#[derive(Debug)]
pub struct Ast<'s> {
    pub declarations: Vec<Declaration<'s>>,
    pub statements: Vec<Statement<'s>>,
    pub trailing_label: Option<Label<'s>>,
}

#[derive(Debug)]
pub enum Declaration<'s> {
    Register(DeclareRegister<'s>),
    Bus(DeclareBus<'s>),
    Memory(DeclareMemory<'s>),
    RegisterArray(DeclareRegisterArray<'s>),
}

#[derive(Debug)]
pub struct DeclareRegister<'s> {
    pub registers: Vec<RegBus<'s>>,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct DeclareBus<'s> {
    pub buses: Vec<RegBus<'s>>,
}

#[derive(Debug)]
pub struct DeclareMemory<'s> {
    pub memories: Vec<Memory<'s>>,
}

#[derive(Debug)]
pub struct DeclareRegisterArray<'s> {
    pub register_arrays: Vec<DeclareRegisterArrayItem<'s>>,
}

#[derive(Debug)]
pub struct RegBus<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
}

#[derive(Debug)]
pub struct Concat<'s> {
    pub parts: Vec<ConcatPart<'s>>,
}

#[derive(Debug)]
pub enum ConcatPart<'s> {
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug)]
pub struct Memory<'s> {
    pub ident: Ident<'s>,
    pub range: MemoryRange<'s>,
}

#[derive(Debug)]
pub struct DeclareRegisterArrayItem<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub len: usize,
}

#[derive(Debug)]
pub struct RegisterArray<'s> {
    pub ident: Ident<'s>,
    pub index: Box<Expression<'s>>,
}

#[derive(Debug)]
pub struct Statement<'s> {
    pub label: Option<Label<'s>>,
    pub operations: Vec<Operation<'s>>,
    pub operations_post: Option<Vec<Operation<'s>>>,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct Operation<'s> {
    pub kind: OperationKind<'s>,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum OperationKind<'s> {
    Nop(Nop),
    Goto(Goto<'s>),
    Write(Write<'s>),
    Read(Read<'s>),
    If(If<'s>),
    Assignment(Assignment<'s>),
}

#[derive(Debug)]
pub struct Nop;

#[derive(Debug)]
pub struct Goto<'s> {
    pub label: Label<'s>,
}

#[derive(Debug)]
pub struct Write<'s> {
    pub ident: Ident<'s>,
}

#[derive(Debug)]
pub struct Read<'s> {
    pub ident: Ident<'s>,
}

#[derive(Debug)]
pub struct If<'s> {
    pub condition: Expression<'s>,
    pub operations_if: Vec<Operation<'s>>,
    pub operations_else: Option<Vec<Operation<'s>>>,
}

#[derive(Debug)]
pub enum Lvalue<'s> {
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Concat(Concat<'s>),
}

#[derive(Debug)]
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
}

#[derive(Debug)]
pub enum Expression<'s> {
    Atom(Atom<'s>),
    BinaryTerm(Box<BinaryTerm<'s>>),
    UnaryTerm(Box<UnaryTerm<'s>>),
}

impl<'s> From<Atom<'s>> for Expression<'s> {
    fn from(v: Atom<'s>) -> Self {
        Self::Atom(v)
    }
}

impl<'s> From<BinaryTerm<'s>> for Expression<'s> {
    fn from(v: BinaryTerm<'s>) -> Self {
        Self::BinaryTerm(Box::new(v))
    }
}

impl<'s> From<UnaryTerm<'s>> for Expression<'s> {
    fn from(v: UnaryTerm<'s>) -> Self {
        Self::UnaryTerm(Box::new(v))
    }
}

#[derive(Debug)]
pub enum Atom<'s> {
    Concat(Concat<'s>),
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: BinaryOperator,
}

#[derive(Debug)]
pub struct UnaryTerm<'s> {
    pub expression: Expression<'s>,
    pub operator: UnaryOperator,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryRange<'s> {
    pub address_register: Ident<'s>,
    pub data_register: Ident<'s>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident<'s>(pub &'s str);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Label<'s>(pub &'s str);
