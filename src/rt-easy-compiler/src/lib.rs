#![deny(rust_2018_idioms)]

mod build_mir;
mod check_ast;
mod check_mir;
mod error;
mod symbols;
mod util;

pub mod mir;
pub use self::error::{CompilerError, Error, InternalError};
pub use self::symbols::SymbolType;

pub trait Backend {
    type Output;
    type Error: std::error::Error + Send + Sync + 'static;

    fn generate(&self, mir: mir::Mir<'_>) -> Result<Self::Output, Self::Error>;
}

pub fn compile<B>(backend: &B, ast: rtcore::ast::Ast<'_>) -> Result<B::Output, Error>
where
    B: Backend,
{
    let (_symbols, mir) = check_(ast)?;

    match backend.generate(mir) {
        Ok(output) => Ok(output),
        Err(e) => Err(Error::Backend(e.into())),
    }
}

pub fn check(ast: rtcore::ast::Ast<'_>) -> Result<(), Error> {
    check_(ast)?;
    Ok(())
}

fn check_(ast: rtcore::ast::Ast<'_>) -> Result<(symbols::Symbols<'_>, mir::Mir<'_>), Error> {
    // Check ast
    let symbols = check_ast::check(&ast)?;

    // Build and check mir
    let mut mir = build_mir::build_mir(ast, &symbols)?;
    check_mir::check(&symbols, &mut mir)?;

    Ok((symbols, mir))
}
