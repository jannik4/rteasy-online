use crate::Ident;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RenderError>;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("memory `{0}` not found")]
    MemoryNotFound(Ident),
    #[error(
        "invalid memory size for `{name}`. expected: (AR: {}, DR: {}), actual: (AR: {}, DR: {})",
        expected.0,
        expected.1,
        actual.0,
        actual.1,
    )]
    InvalidMemorySize { name: Ident, expected: (usize, usize), actual: (usize, usize) },
}
