#![deny(rust_2018_idioms)]

mod run;

pub mod parser;
pub mod unit_test;

pub use self::run::run;
