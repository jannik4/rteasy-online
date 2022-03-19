use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynthError {
    #[error("next state depends on an unclocked item")]
    UnclockedGotoDependency,
    #[error("conditional goto in first state")]
    ConditionalGotoInFirstState,
}

#[derive(Debug, Error)]
pub enum RenderError {}
