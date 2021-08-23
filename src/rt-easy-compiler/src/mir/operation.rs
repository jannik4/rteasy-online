use super::*;

#[derive(Debug)]
pub struct Operation<'s> {
    pub kind: OperationKind<'s>,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum OperationKind<'s> {
    EvalCriterion(EvalCriterion<'s>),
    Nop(Nop),
    Goto(Goto<'s>),
    Write(Write<'s>),
    Read(Read<'s>),
    Assignment(Assignment<'s>),
}

#[derive(Debug)]
pub struct EvalCriterion<'s> {
    pub criterion_id: CriterionId,
    pub condition: Expression<'s>,
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
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
    pub size: usize,
}

#[derive(Debug)]
pub enum Lvalue<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    ConcatClocked(ConcatLvalueClocked<'s>),
    ConcatUnclocked(ConcatLvalueUnclocked<'s>),
}
