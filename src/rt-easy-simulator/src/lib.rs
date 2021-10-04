#![deny(rust_2018_idioms)]

mod changed;
mod error;
mod evaluate;
mod execute;
mod simulator;
mod state;

pub use self::{
    changed::Changed,
    error::{Error, Result},
    simulator::{Simulator, StepResult, StepResultKind},
};
