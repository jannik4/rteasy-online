#![deny(rust_2018_idioms)]

mod bit;
mod impl_ops;
mod slice;
mod value;

pub use self::{bit::Bit, slice::ValueSlice, value::Value};
