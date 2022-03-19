use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<rt_easy::rtcore::common::Span> for Span {
    fn from(span: rt_easy::rtcore::common::Span) -> Self {
        Self { start: span.start, end: span.end }
    }
}
