#![deny(rust_2018_idioms)]
//! # VHDL Backend
//!
//! This crate provides a backend for the rt-easy-compiler to generate VHDL code.
//!
//! ## Procedure
//!
//! ### 1. Transform
//!
//! Transform rt code so that goto operations occur only after the pipe.
//! This is necessary for the implementation in VHDL.
//!
//! ### 2. Generate (and canonicalize)
//!
//! Generate the VHDL data structure from the MIR.
//! This step includes canonicalizing `BitRange`s to make sure operations/expressions are considered
//! equal if possible to filter out duplicates.
//!
//! ### 3. Render
//!
//! Render the VHDL data structure to a string.

mod generate;
mod impl_render;
mod render_as_rt;
mod render_as_vhdl;
mod signals;

pub mod vhdl;

pub use self::{signals::Signals, vhdl::Vhdl};

#[derive(Debug)]
pub struct BackendVhdl;

impl compiler::Backend for BackendVhdl {
    type Args = ();
    type Output = vhdl::Vhdl;
    type Error = std::convert::Infallible;

    fn generate(
        &self,
        mir: compiler::mir::Mir<'_>,
        (): Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        Ok(generate::generate(mir))
    }
}
