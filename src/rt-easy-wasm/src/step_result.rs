use crate::Span;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct StepResult {
    pub is_at_statement_start: bool,
    pub span: Span,
    pub condition: Option<StepResultCondition>,
}

impl From<rt_easy::simulator::StepResult> for StepResult {
    fn from(step_result: rt_easy::simulator::StepResult) -> Self {
        Self {
            is_at_statement_start: step_result.is_at_statement_start,
            span: step_result.span.into(),
            condition: step_result
                .condition
                .map(|(value, span)| StepResultCondition { value, span: span.into() }),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct StepResultCondition {
    pub value: bool,
    pub span: Span,
}
