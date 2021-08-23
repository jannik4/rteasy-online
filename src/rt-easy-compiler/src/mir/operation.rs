use super::*;

#[derive(Debug, Clone)]
pub struct Operation<'s> {
    pub kind: OperationKind<'s>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum OperationKind<'s> {
    EvalCriterion(EvalCriterion<'s>),
    Nop(Nop),
    Goto(Goto<'s>),
    Write(Write<'s>),
    Read(Read<'s>),
    Assignment(Assignment<'s>),
}

#[derive(Debug, Clone)]
pub struct EvalCriterion<'s> {
    pub criterion_id: CriterionId,
    pub condition: Expression<'s>,
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
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum Lvalue<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    ConcatClocked(ConcatLvalueClocked<'s>),
    ConcatUnclocked(ConcatLvalueUnclocked<'s>),
}
