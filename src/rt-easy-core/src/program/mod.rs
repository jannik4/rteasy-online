mod concat;
mod declaration;
mod expression;
mod impl_display;
mod operation;

use std::ops::Range;

pub use self::{concat::*, declaration::*, expression::*, operation::*};
pub use crate::common::*;
pub use split_vec::SplitVec;

#[derive(Debug)]
pub struct Program {
    declarations: Vec<Declaration>,
    statements: Vec<Statement>,
}

impl Program {
    pub fn new_unchecked(declarations: Vec<Declaration>, statements: Vec<Statement>) -> Self {
        Self { declarations, statements }
    }

    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}

#[derive(Debug)]
pub struct Statement {
    pub label: Option<Label>,
    pub steps: SplitVec<Step>,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct Step {
    pub criteria: Vec<Criterion>,
    pub operation: Operation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CriterionId(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum Criterion {
    True(CriterionId),
    False(CriterionId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(pub String);

impl From<crate::ast::Ident<'_>> for Ident {
    fn from(v: crate::ast::Ident<'_>) -> Self {
        Self(v.0.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

impl From<crate::ast::Label<'_>> for Label {
    fn from(v: crate::ast::Label<'_>) -> Self {
        Self(v.0.to_string())
    }
}
