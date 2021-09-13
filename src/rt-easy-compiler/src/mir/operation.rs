use super::*;

#[derive(Debug, Clone)]
pub enum Operation<'s> {
    EvalCriterion(EvalCriterion<'s>),
    EvalCriterionSwitchGroup(EvalCriterionSwitchGroup<'s>),
    Nop(Nop),
    Goto(Goto<'s>),
    Write(Write<'s>),
    Read(Read<'s>),
    Assignment(Assignment<'s>),
    Assert(Assert<'s>),
}

impl Operation<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::EvalCriterion(n) => n.span(),
            Self::EvalCriterionSwitchGroup(n) => n.span,
            Self::Nop(n) => n.span,
            Self::Goto(n) => n.span,
            Self::Write(n) => n.span,
            Self::Read(n) => n.span,
            Self::Assignment(n) => n.span,
            Self::Assert(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalCriterion<'s> {
    pub criterion_id: CriterionId,
    pub condition: Expression<'s>,
}

impl EvalCriterion<'_> {
    pub fn span(&self) -> Span {
        self.condition.span()
    }
}

#[derive(Debug, Clone)]
pub struct EvalCriterionSwitchGroup<'s> {
    pub eval_criteria: Vec<EvalCriterion<'s>>,
    pub switch_expression_size: usize,
    pub span: Span,
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
pub struct Assignment<'s> {
    pub lhs: Lvalue<'s>,
    pub rhs: Expression<'s>,
    pub size: usize,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Lvalue<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    ConcatClocked(ConcatLvalueClocked<'s>),
    ConcatUnclocked(ConcatLvalueUnclocked<'s>),
}

#[derive(Debug, Clone)]
pub struct Assert<'s> {
    pub condition: Expression<'s>,
    pub span: Span,
}
