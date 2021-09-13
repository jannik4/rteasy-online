pub use crate::common::*;
pub use either::Either;

#[derive(Debug, Clone)]
pub struct Ast<'s> {
    pub declarations: Vec<Declaration<'s>>,
    pub statements: Vec<Statement<'s>>,
    pub trailing_label: Option<Spanned<Label<'s>>>,
}

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
    pub registers: Vec<RegBus<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareBus<'s> {
    pub buses: Vec<RegBus<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareMemory<'s> {
    pub memories: Vec<Memory<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArray<'s> {
    pub register_arrays: Vec<DeclareRegisterArrayItem<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RegBus<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: Option<Spanned<BitRange>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Concat<'s> {
    pub parts: Vec<ConcatPart<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ConcatPart<'s> {
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Spanned<Number>),
}

impl ConcatPart<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::RegBus(n) => n.span,
            Self::RegisterArray(n) => n.span,
            Self::Number(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Memory<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: MemoryRange<'s>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclareRegisterArrayItem<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: Option<Spanned<BitRange>>,
    pub len: usize,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RegisterArray<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub index: Box<Expression<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Statement<'s> {
    pub label: Option<Spanned<Label<'s>>>,
    pub operations: Operations<'s>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Operations<'s> {
    pub operations: Vec<Operation<'s>>,
    pub operations_post: Option<Vec<Operation<'s>>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Operation<'s> {
    Nop(Nop),
    Goto(Goto<'s>),
    Write(Write<'s>),
    Read(Read<'s>),
    If(If<'s>),
    Switch(Switch<'s>),
    Assignment(Assignment<'s>),
    Assert(Assert<'s>),
}

impl Operation<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Nop(n) => n.span,
            Self::Goto(n) => n.span,
            Self::Write(n) => n.span,
            Self::Read(n) => n.span,
            Self::If(n) => n.span,
            Self::Switch(n) => n.span,
            Self::Assignment(n) => n.span,
            Self::Assert(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nop {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Goto<'s> {
    pub label: Spanned<Label<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Write<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Read<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct If<'s> {
    pub condition: Expression<'s>,
    pub operations_if: Vec<Operation<'s>>,
    pub operations_else: Option<Vec<Operation<'s>>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Switch<'s> {
    pub expression: Expression<'s>,
    pub clauses: Vec<Clause<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Clause<'s> {
    pub clause: Either<CaseClause<'s>, DefaultClause>,
    pub operations: Vec<Operation<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CaseClause<'s> {
    pub value: Expression<'s>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DefaultClause {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Lvalue<'s> {
    RegBus(RegBus<'s>),
    RegisterArray(RegisterArray<'s>),
    Concat(Concat<'s>),
}

impl Lvalue<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::RegBus(n) => n.span,
            Self::RegisterArray(n) => n.span,
            Self::Concat(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Assert<'s> {
    pub condition: Expression<'s>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expression<'s> {
    Atom(Atom<'s>),
    BinaryTerm(Box<BinaryTerm<'s>>),
    UnaryTerm(Box<UnaryTerm<'s>>),
}

impl Expression<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Atom(n) => n.span(),
            Self::BinaryTerm(n) => n.span,
            Self::UnaryTerm(n) => n.span,
        }
    }
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
    Number(Spanned<Number>),
}

impl Atom<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Concat(n) => n.span,
            Self::RegBus(n) => n.span,
            Self::RegisterArray(n) => n.span,
            Self::Number(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: Spanned<BinaryOperator>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UnaryTerm<'s> {
    pub expression: Expression<'s>,
    pub operator: Spanned<UnaryOperator>,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryRange<'s> {
    pub address_register: Spanned<Ident<'s>>,
    pub data_register: Spanned<Ident<'s>>,
    pub span: Span,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident<'s>(pub &'s str);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Label<'s>(pub &'s str);
