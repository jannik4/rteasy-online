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
            steps: Spanned { node: Vec::new(), span: Span::dummy() },
            span: trailing_label.span,
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
                steps: Spanned {
                    node: step::build(
                        statement.operations.operations,
                        statement.operations.operations_post,
                        symbols,
                    )?,
                    span: statement.operations.span,
                },
                span: statement.span,
            })
        })
        .collect()
}
