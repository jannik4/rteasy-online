use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynthError {
    #[error("next state depends on an unclocked item")]
    NextStateUnclockedDependency,
}

