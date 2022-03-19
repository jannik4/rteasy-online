use super::*;

#[derive(Debug)]
pub struct Operation {
    pub kind: OperationKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum OperationKind {
    EvalCriterion(EvalCriterion),
    EvalCriterionGroup(EvalCriterionGroup),
    Nop(Nop),
    Goto(Goto),
    Write(Write),
    Read(Read),
    Assignment(Assignment),
    Assert(Assert),
}

#[derive(Debug)]
pub struct EvalCriterion {
    pub criterion_id: CriterionId,
    pub condition: Expression,
}

#[derive(Debug)]
pub struct EvalCriterionGroup(pub Vec<EvalCriterion>);

#[derive(Debug)]
pub struct Nop;

#[derive(Debug)]
pub struct Goto {
    pub label: Label,
}

#[derive(Debug)]
pub struct Write {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct Read {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct Assignment {
    pub lhs: Lvalue,
    pub rhs: Expression,
    pub size: usize,
}

#[derive(Debug)]
pub enum Lvalue {
    Register(Register),
    Bus(Bus),
    RegisterArray(RegisterArray),
    ConcatClocked(ConcatLvalueClocked),
    ConcatUnclocked(ConcatLvalueUnclocked),
}

#[derive(Debug)]
pub struct Assert {
    pub condition: Expression,
}
