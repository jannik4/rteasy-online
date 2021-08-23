#![deny(rust_2018_idioms)]

mod error;
mod evaluate;
mod execute;
mod simulator;
mod state;

pub use self::{
    error::Error,
    simulator::Simulator,
    state::{ChangeSet, State},
};
