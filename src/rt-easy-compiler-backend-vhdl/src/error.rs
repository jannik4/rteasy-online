use thiserror::Error;

pub type Result<T> = std::result::Result<T, SynthError>;

#[derive(Debug, Error)]
pub enum SynthError {
    #[error("next state depends on an unclocked item")]
    UnclockedGotoDependency,
    #[error("conditional goto in first state")]
    ConditionalGotoInFirstState,
}
