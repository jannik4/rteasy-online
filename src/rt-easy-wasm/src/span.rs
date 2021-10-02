use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<rt_easy::rtcore::program::Span> for Span {
    fn from(span: rt_easy::rtcore::program::Span) -> Self {
        Self { start: span.start, end: span.end }
    }
}
