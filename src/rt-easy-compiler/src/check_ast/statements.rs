use super::operation::CheckOp;
use crate::{symbols::Symbols, CompilerError};
use rtcore::ast;

pub fn check(
    statements: &[ast::Statement<'_>],
    symbols: &Symbols<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) {
    for statement in statements {
        check_statement(statement, symbols, error_sink);
    }
}

fn check_statement(
    statement: &ast::Statement<'_>,
    symbols: &Symbols<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) {
    let res = statement.operations.check_op(symbols, error_sink);

    if let Some(operations_post) = &statement.operations_post {
        let res_post = operations_post.check_op(symbols, error_sink);

        if res.contains_goto {
            error_sink(CompilerError::GotoBeforePipe);
        }
        if res_post.contains_mutate {
            error_sink(CompilerError::MutateAfterPipe);
        }
    }
}
