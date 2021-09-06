mod concat;
mod declaration;
mod expression;
mod impl_display;
mod operation;

pub use self::{concat::*, declaration::*, expression::*, operation::*};
pub use rtcore::ast::{Ident, Label};
pub use rtcore::common::{BinaryOperator, BitRange, CtxSize, Number, Span, Spanned, UnaryOperator};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Mir<'s> {
    pub declarations: Vec<Declaration<'s>>,
    pub statements: Vec<Statement<'s>>,
}

#[derive(Debug, Clone)]
pub struct Statement<'s> {
    pub label: Option<Spanned<Label<'s>>>,
    pub steps: Spanned<Vec<Step<'s>>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Step<'s> {
    pub id: StepId,
    pub criteria: Vec<Criterion>,
    pub operation: Operation<'s>,
    pub annotation: Annotation,
}

impl Step<'_> {
    pub fn span(&self) -> Span {
        self.operation.span()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StepId(pub usize);

#[derive(Debug, Clone)]
pub struct Annotation {
    pub is_unclocked_assign: bool,
    pub is_post_pipe: bool,
    pub dependencies: HashSet<StepId>,
}

impl Annotation {
    pub fn new(is_unclocked_assign: bool, is_post_pipe: bool) -> Self {
        Self { is_unclocked_assign, is_post_pipe, dependencies: HashSet::new() }
    }
}

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
