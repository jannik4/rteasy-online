mod bit_range;
mod criteria_expr;
mod expression;
mod label;
mod operation;

// TODO: Display instead of all RenderXXXX
// --> add necessary data in generate, e.g. add ctx_size for expressions

use crate::vhdl::{
    BitRange, BusKind, Declarations, Expression, NextStateLogic, Operation, RegisterKind,
    Statement, Vhdl,
};
use indexmap::IndexSet;
use temply::Template;

use self::{
    bit_range::RenderBitRange, criteria_expr::RenderCriteriaExpr, operation::RenderOperation,
};
use crate::signals::Fmt;

pub fn render(vhdl: &Vhdl<'_>) -> Result<String, std::fmt::Error> {
    let mut buffer = String::new();
    VhdlTemplate {
        module_name: &vhdl.module_name,
        statements: &vhdl.statements,
        criteria: &vhdl.criteria,
        operations: &vhdl.operations,
        declarations: &vhdl.declarations,
    }
    .render(&mut buffer)?;
    Ok(buffer)
}

#[derive(Debug, Template)]
#[template = "./impl_render/template.vhdl"]
struct VhdlTemplate<'a> {
    module_name: &'a str,
    statements: &'a [Statement],
    criteria: &'a IndexSet<Expression<'a>>, // Index = CriterionId
    operations: &'a IndexSet<Operation<'a>>, // Index = OperationId

    declarations: &'a Declarations<'a>,
}

#[derive(Debug)]
struct Render<T>(T);
