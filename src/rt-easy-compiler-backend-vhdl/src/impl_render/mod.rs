mod bit_range;
mod criteria_expr;
mod expression;
mod label;
mod operation;

use crate::vhdl::{
    BitRange, BusKind, Declarations, Expression, Operation, RegisterKind, Statement, Vhdl,
};
use indexmap::IndexSet;
use temply::Template;

use self::{
    bit_range::RenderBitRange, criteria_expr::RenderCriteriaExpr, expression::RenderExpression,
    label::RenderLabel, operation::RenderOperation,
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
    statements: &'a [Statement<'a>],
    criteria: &'a IndexSet<Expression<'a>>, // Index = CriterionId
    operations: &'a IndexSet<Operation<'a>>, // Index = OperationId

    declarations: &'a Declarations<'a>,
}
