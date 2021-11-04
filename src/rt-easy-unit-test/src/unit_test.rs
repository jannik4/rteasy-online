pub use rtcore::common::{Span, Spanned};
pub use rtcore::value::Value;

#[derive(Debug)]
pub struct UnitTest {
    pub operations: Vec<Operation>,
}

#[derive(Debug)]
pub struct Operation {
    pub kind: OperationKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum OperationKind {
    Step(Step),
    MicroStep(MicroStep),
    Run(Run),
    Reset(Reset),
    SetBreakpoint(SetBreakpoint),
    RemoveBreakpoint(RemoveBreakpoint),
    Assignment(Assignment),
    Assert(Assert),
}

#[derive(Debug)]
pub struct Step {
    pub amount: Option<usize>,
}

#[derive(Debug)]
pub struct MicroStep {
    pub amount: Option<usize>,
}

#[derive(Debug)]
pub struct Run;

#[derive(Debug)]
pub struct Reset;

#[derive(Debug)]
pub struct Assignment {
    pub assignment: String,
}

#[derive(Debug)]
pub struct Assert {
    pub assert: String,
}

#[derive(Debug)]
pub struct SetBreakpoint {
    pub label: Label,
}

#[derive(Debug)]
pub struct RemoveBreakpoint {
    pub label: Label,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label(pub String);
