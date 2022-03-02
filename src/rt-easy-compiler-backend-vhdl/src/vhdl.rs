use crate::Signals;
use indexmap::{IndexMap, IndexSet};
use vec1::Vec1;

// -------------------------------------------------------------------------------------------------
// Re-export
// -------------------------------------------------------------------------------------------------

pub use rtcore::ast::{Ident, Label as LabelNamed};
pub use rtcore::common::{BinaryOperator, BitRange, BusKind, CtxSize, RegisterKind, UnaryOperator};

// -------------------------------------------------------------------------------------------------
// Top
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Vhdl<'s> {
    pub module_name: String,
    pub statements: Vec<Statement<'s>>,
    pub criteria: IndexSet<Expression<'s>>, // Index = CriterionId
    pub operations: IndexSet<Operation<'s>>, // Index = OperationId

    pub declarations: Declarations<'s>,
}

impl<'s> Vhdl<'s> {
    pub fn signals(&self) -> Signals {
        Signals::new(self)
    }

    pub fn render(&self) -> Result<String, std::fmt::Error> {
        crate::impl_render::render(self)
    }

    // pub fn registers(&self, kind: RegisterKind) -> impl Iterator<Item = &Register<'s>> {
    //     self.declarations.registers.iter().filter(move |reg| reg.kind == kind)
    // }
    //
    // pub fn buses(&self, kind: RegisterKind) -> impl Iterator<Item = &Register<'s>> {
    //     self.declarations.registers.iter().filter(move |reg| reg.kind == kind)
    // }
}

// -------------------------------------------------------------------------------------------------
// Declarations
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Declarations<'s> {
    pub registers: Vec<Register<'s>>,
    pub buses: Vec<Bus<'s>>,
    pub memories: Vec<Memory<'s>>,
    pub register_arrays: Vec<RegisterArray<'s>>,
}

#[derive(Debug)]
pub struct Memory<'s> {
    pub ident: Ident<'s>,
    pub address_register: Ident<'s>,
    pub data_register: Ident<'s>,
}

// -------------------------------------------------------------------------------------------------
// Statement
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Statement<'s> {
    pub label: Label<'s>,

    pub next_state_conditional: IndexMap<Label<'s>, Or<And<Criterion>>>,
    pub next_state_default: Label<'s>,
    pub operations: IndexMap<OperationId, Option<Or<And<Criterion>>>>,
}

#[derive(Debug)]
pub struct Or<T>(pub Vec1<T>);

#[derive(Debug)]
pub struct And<T>(pub Vec1<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CriterionId(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum Criterion {
    True(CriterionId),
    False(CriterionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Label<'s> {
    Named(LabelNamed<'s>),
    Unnamed(usize),
    End,
}

// -------------------------------------------------------------------------------------------------
// Expression
// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Expression<'s> {
    Atom(Atom<'s>),
    BinaryTerm(Box<BinaryTerm<'s>>),
    UnaryTerm(Box<UnaryTerm<'s>>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Atom<'s> {
    Concat(ConcatExpr<'s>),
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: BinaryOperator,
    pub ctx_size: CtxSize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnaryTerm<'s> {
    pub expression: Expression<'s>,
    pub operator: UnaryOperator,
    pub ctx_size: CtxSize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Register<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub kind: RegisterKind,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Bus<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub kind: BusKind,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RegisterArray<'s> {
    pub ident: Ident<'s>,
    pub index: Box<Expression<'s>>,
    pub index_ctx_size: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Number {
    pub value: rtcore::value::Value,
}

// -------------------------------------------------------------------------------------------------
// Operation
// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operation<'s> {
    Write(Write<'s>),
    Read(Read<'s>),
    Assignment(Assignment<'s>),
}

impl Operation<'_> {
    pub fn is_clocked(&self) -> bool {
        match self {
            Operation::Write(_) | Operation::Read(_) => true,
            Operation::Assignment(assignment) => match &assignment.lhs {
                Lvalue::Register(_) | Lvalue::RegisterArray(_) | Lvalue::ConcatClocked(_) => true,
                Lvalue::Bus(_) | Lvalue::ConcatUnclocked(_) => false,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Write<'s> {
    pub ident: Ident<'s>,
    // TODO: ar, dr
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Read<'s> {
    pub ident: Ident<'s>,
    // TODO: ar, dr
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
    pub size: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Lvalue<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    ConcatClocked(ConcatLvalueClocked<'s>),
    ConcatUnclocked(ConcatLvalueUnclocked<'s>),
}

// -------------------------------------------------------------------------------------------------
// Concat
// -------------------------------------------------------------------------------------------------

pub type ConcatLvalueClocked<'s> = Concat<ConcatPartLvalueClocked<'s>>;
pub type ConcatLvalueUnclocked<'s> = Concat<ConcatPartLvalueUnclocked<'s>>;
pub type ConcatExpr<'s> = Concat<ConcatPartExpr<'s>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Concat<P> {
    pub parts: Vec<P>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcatPartLvalueClocked<'s> {
    Register(Register<'s>, usize),
    RegisterArray(RegisterArray<'s>, usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcatPartLvalueUnclocked<'s> {
    Bus(Bus<'s>, usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcatPartExpr<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}
