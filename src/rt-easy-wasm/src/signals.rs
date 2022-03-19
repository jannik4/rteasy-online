use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Signals(pub(crate) rt_easy::rtprogram::Signals);

#[wasm_bindgen]
impl Signals {
    pub fn condition_signals(&self) -> Vec<JsValue> {
        self.0.condition_signals.iter().map(Into::into).collect()
    }

    pub fn control_signals(&self) -> Vec<JsValue> {
        self.0.control_signals.iter().map(Into::into).collect()
    }
}
