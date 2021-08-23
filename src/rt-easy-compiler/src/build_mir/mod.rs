mod declaration;
mod expression;
mod step;

use crate::mir::*;
use crate::{symbols::Symbols, InternalError};
use rtcore::ast;

type Result<T> = std::result::Result<T, InternalError>;

pub fn build_mir<'s>(ast: ast::Ast<'s>, symbols: &Symbols<'s>) -> Result<Mir<'s>> {
    let mut statements = build_statements(ast.statements, symbols)?;
    if let Some(trailing_label) = ast.trailing_label {
        statements.push(Statement {
            label: Some(trailing_label),
            steps: Vec::new(),
            span: 0..0, // TODO: trailing_label.span(),
        });
    }

    Ok(Mir {
        declarations: ast
            .declarations
            .into_iter()
            .map(|declaration| declaration::build(declaration, symbols))
            .collect::<Result<_>>()?,
        statements,
    })
}

fn build_statements<'s>(
    statements: Vec<ast::Statement<'s>>,
    symbols: &Symbols<'s>,
) -> Result<Vec<Statement<'s>>> {
    statements
        .into_iter()
        .map(|statement| {
            Ok(Statement {
                label: statement.label,
                steps: step::build(statement.operations, statement.operations_post, symbols)?,
                span: statement.span,
            })
        })
        .collect()
}
