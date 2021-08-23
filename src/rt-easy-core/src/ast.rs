use std::ops::Range;

pub use crate::common::*;
pub use either::Either;

#[derive(Debug, Clone)]
pub struct Ast<'s> {
    pub declarations: Vec<Declaration<'s>>,
    pub statements: Vec<Statement<'s>>,
    pub trailing_label: Option<Label<'s>>,
}

#[derive(Debug, Clone)]
pub enum Declaration<'s> {
    Register(DeclareRegister<'s>),
    Bus(DeclareBus<'s>),
    Memory(DeclareMemory<'s>),
    RegisterArray(DeclareRegisterArray<'s>),
}

#[derive(Debug, Clone)]
pub struct DeclareRegister<'s> {
    pub registers: Vec<RegBus<'s>>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct DeclareBus<'s> {
    pub buses: Vec<RegBus<'s>>,
}

#[derive(Debug, Clone)]
pub struct DeclareMemory<'s> {
    pub memories: Vec<Memory<'s>>,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArray<'s> {
    pub register_arrays: Vec<DeclareRegisterArrayItem<'s>>,
}

#[derive(Debug, Clone)]
pub struct RegBus<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
}

#[derive(Debug, Clone)]
pub struct Concat<'s> {
    pub parts: Vec<ConcatPart<'s>>,
}

#[derive(Debug, Clone)]
pub enum ConcatPart<'s> {
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug, Clone)]
pub struct Memory<'s> {
    pub ident: Ident<'s>,
    pub range: MemoryRange<'s>,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArrayItem<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub len: usize,
}

#[derive(Debug, Clone)]
pub struct RegisterArray<'s> {
    pub ident: Ident<'s>,
    pub index: Box<Expression<'s>>,
}

#[derive(Debug, Clone)]
pub struct Statement<'s> {
    pub label: Option<Label<'s>>,
    pub operations: Vec<Operation<'s>>,
    pub operations_post: Option<Vec<Operation<'s>>>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct Operation<'s> {
    pub kind: OperationKind<'s>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum OperationKind<'s> {
    Nop(Nop),
    Goto(Goto<'s>),
    Write(Write<'s>),
    Read(Read<'s>),
    If(If<'s>),
    Switch(Switch<'s>),
    Assignment(Assignment<'s>),
}

#[derive(Debug, Clone)]
pub struct Nop;

#[derive(Debug, Clone)]
pub struct Goto<'s> {
    pub label: Label<'s>,
}

#[derive(Debug, Clone)]
pub struct Write<'s> {
    pub ident: Ident<'s>,
}

#[derive(Debug, Clone)]
pub struct Read<'s> {
    pub ident: Ident<'s>,
}

#[derive(Debug, Clone)]
pub struct If<'s> {
    pub condition: Expression<'s>,
    pub operations_if: Vec<Operation<'s>>,
    pub operations_else: Option<Vec<Operation<'s>>>,
}

#[derive(Debug, Clone)]
pub struct Switch<'s> {
    pub expression: Expression<'s>,
    pub clauses: Vec<Either<CaseClause<'s>, DefaultClause<'s>>>,
}

#[derive(Debug, Clone)]
pub struct CaseClause<'s> {
    pub value: Expression<'s>,
    pub operations: Vec<Operation<'s>>,
}

#[derive(Debug, Clone)]
pub struct DefaultClause<'s> {
    pub operations: Vec<Operation<'s>>,
}

#[derive(Debug, Clone)]
pub enum Lvalue<'s> {
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Concat(Concat<'s>),
}

#[derive(Debug, Clone)]
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Atom<'s> {
    Concat(Concat<'s>),
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug, Clone)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: BinaryOperator,
}

#[derive(Debug, Clone)]
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
