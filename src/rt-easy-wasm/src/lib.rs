#![deny(rust_2018_idioms)]

mod compiler;
mod signals;
mod simulator;
mod span;
mod step_result;
mod util;

use wasm_bindgen::prelude::*;

pub use self::{compiler::*, signals::*, simulator::*, span::*, step_result::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn setPanicHook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
