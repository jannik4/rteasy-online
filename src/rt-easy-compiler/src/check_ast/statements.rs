use super::operation::CheckOp;
use crate::{symbols::Symbols, CompilerError, CompilerErrorKind, InternalError};
use rtcore::ast;

pub type Result = std::result::Result<(), InternalError>;

pub fn check(
    statements: &[ast::Statement<'_>],
    symbols: &Symbols<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result {
    for statement in statements {
        check_statement(statement, symbols, error_sink)?;
    }

    Ok(())
}

fn check_statement(
    statement: &ast::Statement<'_>,
    symbols: &Symbols<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result {
    let res = statement.operations.operations.check_op(symbols, error_sink)?;

    if let Some(operations_post) = &statement.operations.operations_post {
        let res_post = operations_post.check_op(symbols, error_sink)?;

        if res.contains_goto {
            error_sink(CompilerError::new(
                CompilerErrorKind::GotoBeforePipe,
                statement.operations.span,
            ));
        }
        if res_post.contains_mutate {
            error_sink(CompilerError::new(
                CompilerErrorKind::MutateAfterPipe,
                statement.operations.span,
            ));
        }
    }

    Ok(())
}
