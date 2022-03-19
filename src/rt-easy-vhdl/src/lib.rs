#![deny(rust_2018_idioms)]

mod impl_render;
mod render_as_rt;
mod render_as_vhdl;
mod signals;

pub mod error;

use crate::error::RenderError;

// -------------------------------------------------------------------------------------------------
// Re-export
// -------------------------------------------------------------------------------------------------

pub use self::signals::Signals;
pub use indexmap::{IndexMap, IndexSet};
pub use rtcore::common::{BinaryOperator, BusKind, NumberKind, RegisterKind, UnaryOperator};
pub use vec1::{vec1, Vec1};

// -------------------------------------------------------------------------------------------------
// Top
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Vhdl {
    pub statements: Vec<Statement>,
    pub criteria: IndexSet<Expression>,  // Index = CriterionId
    pub operations: IndexSet<Operation>, // Index = OperationId

    pub declarations: Declarations,
}

impl Vhdl {
    pub fn signals(&self) -> Signals {
        Signals::new(self)
    }

    pub fn render(&self, module_name: &str) -> Result<String, RenderError> {
        crate::impl_render::render(self, module_name)
    }
}

// -------------------------------------------------------------------------------------------------
// Declarations
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Declarations {
    pub registers: Vec<(Ident, BitRange, RegisterKind)>, // (Name, Range, Kind)
    pub buses: Vec<(Ident, BitRange, BusKind)>,          // (Name, Range, Kind)
    pub register_arrays: Vec<(Ident, BitRange, usize)>,  // (Name, Range, Length)
    pub memories: Vec<(Ident, (Ident, BitRange, RegisterKind), (Ident, BitRange, RegisterKind))>, // (Name, AR, DR)
}

// -------------------------------------------------------------------------------------------------
// Statement
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Statement {
    pub label: Label,
    pub operations: IndexMap<OperationId, Option<Or<And<Criterion>>>>,
    pub next_state_logic: NextStateLogic,
}

#[derive(Debug, Clone)]
pub enum NextStateLogic {
    Label(Label),
    Cond { conditional: Vec1<(Or<And<Criterion>>, NextStateLogic)>, default: Box<NextStateLogic> },
}

impl NextStateLogic {
    pub fn as_label(&self) -> Option<&Label> {
        if let Self::Label(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Or<T>(pub Vec1<T>);

#[derive(Debug, Clone)]
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

impl Criterion {
    pub fn id(self) -> CriterionId {
        match self {
            Criterion::True(id) => id,
            Criterion::False(id) => id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

impl Label {
    pub fn terminated() -> Self {
        Self("TERMINATED".to_string())
    }

    pub fn named(name: &str) -> Self {
        Self(format!("NAMED_{}", name))
    }

    pub fn unnamed(idx: usize) -> Self {
        Self(format!("UNNAMED_{}", idx))
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// -------------------------------------------------------------------------------------------------
// Expression
// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub extend_to: Extend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Extend {
    Zero(usize),
    Sign(usize),
}

impl Extend {
    pub fn size(&self) -> usize {
        match *self {
            Extend::Zero(size) => size,
            Extend::Sign(size) => size,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExpressionKind {
    Atom(Atom),
    BinaryTerm(Box<BinaryTerm>),
    UnaryTerm(Box<UnaryTerm>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Atom {
    Concat(ConcatExpr),
    Register(Register),
    Bus(Bus),
    RegisterArray(RegisterArray),
    Number(Number),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BinaryTerm {
    pub lhs: Expression,
    pub rhs: Expression,
    pub operator: BinaryOperator,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnaryTerm {
    pub expression: Expression,
    pub operator: UnaryOperator,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Register {
    pub ident: Ident,
    pub range: Option<BitRange>,
    pub kind: RegisterKind,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Bus {
    pub ident: Ident,
    pub range: Option<BitRange>,
    pub kind: BusKind,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RegisterArray {
    pub ident: Ident,
    pub index: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Number {
    pub value: rtcore::value::Value,
    pub kind: DebugInfo<NumberKind>,
}

// -------------------------------------------------------------------------------------------------
// Operation
// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Write(Write),
    Read(Read),
    Assignment(Assignment),
}

impl Operation {
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
pub struct Write {
    pub memory: Ident,
    pub ar: Register,
    pub dr: Register,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Read {
    pub memory: Ident,
    pub ar: Register,
    pub dr: Register,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Assignment {
    pub lhs: Lvalue,
    pub rhs: Expression,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Lvalue {
    Register(Register),
    Bus(Bus),
    RegisterArray(RegisterArray),
    ConcatClocked(ConcatLvalueClocked),
    ConcatUnclocked(ConcatLvalueUnclocked),
}

// -------------------------------------------------------------------------------------------------
// Concat
// -------------------------------------------------------------------------------------------------

pub type ConcatLvalueClocked = Concat<ConcatPartLvalueClocked>;
pub type ConcatLvalueUnclocked = Concat<ConcatPartLvalueUnclocked>;
pub type ConcatExpr = Concat<ConcatPartExpr>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Concat<P> {
    pub parts: Vec<P>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcatPartLvalueClocked {
    Register(Register, usize),
    RegisterArray(RegisterArray, usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcatPartLvalueUnclocked {
    Bus(Bus, usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConcatPartExpr {
    Register(Register),
    Bus(Bus),
    RegisterArray(RegisterArray),
    Number(Number),
}

// -------------------------------------------------------------------------------------------------
// Bit Range
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitRange {
    Downto(usize, usize),
    To(usize, usize),
}

impl BitRange {
    pub fn size(&self) -> usize {
        match *self {
            BitRange::Downto(a, b) | BitRange::To(b, a) => a - b + 1,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Debug Info
// -------------------------------------------------------------------------------------------------

/// Additional information that should not affect equality.
#[derive(Debug, Clone, Copy)]
pub struct DebugInfo<T>(pub T);

impl<T> PartialEq for DebugInfo<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl<T> Eq for DebugInfo<T> {}
impl<T> std::hash::Hash for DebugInfo<T> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}
