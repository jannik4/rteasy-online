mod expression;
mod operation;
mod statements;

use crate::{symbols::Symbols, Error};

pub fn check<'s>(ast: &rtcore::ast::Ast<'s>) -> Result<Symbols<'s>, Error> {
    // Errors
    let mut errors = Vec::new();
    let mut error_sink = |e| errors.push(e);

    // Build symbols
    let symbols = Symbols::build(ast, &mut error_sink);

    // Check statements
    statements::check(&ast.statements, &symbols, &mut error_sink)?;

    // Check errors
    if errors.is_empty() {
        Ok(symbols)
    } else {
        Err(Error::Errors(errors))
    }
}
