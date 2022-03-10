use crate::vhdl::{
    BitRange, BusKind, Declarations, Expression, NextStateLogic, Operation, RegisterKind,
    Statement, Vhdl,
};
use crate::{render_as_rt::RenderAsRt, render_as_vhdl::RenderAsVhdl};
use indexmap::IndexSet;
use temply::Template;

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
