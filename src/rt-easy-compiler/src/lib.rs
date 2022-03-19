#![deny(rust_2018_idioms)]

mod build_mir;
mod check_ast;
mod check_mir;
mod error;
mod symbols;
mod util;

pub mod mir;
pub use self::error::{BackendError, CompilerError, CompilerErrorKind, Error, InternalError};
pub use self::symbols::SymbolType;

pub trait Backend {
    type Args;
    type Output;
    type Error: std::error::Error + Send + Sync + 'static;

    fn generate(&self, mir: mir::Mir<'_>, args: Self::Args) -> Result<Self::Output, Self::Error>;
}

#[derive(Debug, Default)]
pub struct Options {
    pub print_mir_unordered: bool,
    pub print_mir: bool,
}

pub fn compile<B>(
    backend: &B,
    args: B::Args,
    ast: rtast::Ast<'_>,
    options: &Options,
) -> Result<B::Output, Error>
where
    B: Backend,
{
    let (_symbols, mir) = check_(ast, options)?;

    match backend.generate(mir, args) {
        Ok(output) => Ok(output),
        Err(e) => Err(Error::Backend(BackendError(e.into()))),
    }
}

pub fn check(ast: rtast::Ast<'_>, options: &Options) -> Result<(), Error> {
    check_(ast, options)?;
    Ok(())
}

fn check_<'s>(
    ast: rtast::Ast<'s>,
    options: &Options,
) -> Result<(symbols::Symbols<'s>, mir::Mir<'s>), Error> {
    // Check ast
    let symbols = check_ast::check(&ast)?;

    // Build and check mir
    let mut mir = build_mir::build_mir(ast, &symbols)?;
    check_mir::check(&symbols, &mut mir, options)?;

    Ok((symbols, mir))
}
