use crate::Span;
use rt_easy::simulator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct StepResult {
    pub statement: usize,
    pub span: Span,
    kind: simulator::StepResultKind,
}

#[wasm_bindgen]
impl StepResult {
    pub fn is_void(&self) -> bool {
        matches!(self.kind, simulator::StepResultKind::Void)
    }

    pub fn is_condition(&self) -> bool {
        matches!(self.kind, simulator::StepResultKind::Condition { .. })
    }

    pub fn is_pipe(&self) -> bool {
        matches!(self.kind, simulator::StepResultKind::Pipe(..))
    }

    pub fn is_statement_end(&self) -> bool {
        matches!(self.kind, simulator::StepResultKind::StatementEnd(..))
    }

    pub fn is_breakpoint(&self) -> bool {
        matches!(self.kind, simulator::StepResultKind::Breakpoint)
    }

    pub fn is_assert_error(&self) -> bool {
        matches!(self.kind, simulator::StepResultKind::AssertError)
    }

    pub fn as_condition(&self) -> Option<StepResultCondition> {
        match self.kind {
            simulator::StepResultKind::Condition { result, span } => {
                Some(StepResultCondition { result, span: span.into() })
            }
            _ => None,
        }
    }

    pub fn changed_registers(&self) -> Vec<JsValue> {
        let changed = match &self.kind {
            simulator::StepResultKind::Pipe(changed)
            | simulator::StepResultKind::StatementEnd(changed) => &changed.registers,
            _ => return Vec::new(),
        };
        changed.iter().map(|reg| JsValue::from_str(&reg.0)).collect()
    }

    pub fn changed_register_arrays(&self) -> Vec<JsValue> {
        let changed = match &self.kind {
            simulator::StepResultKind::Pipe(changed)
            | simulator::StepResultKind::StatementEnd(changed) => &changed.register_arrays,
            _ => return Vec::new(),
        };
        changed
            .iter()
            .map(|(reg_array, idx)| {
                [JsValue::from_str(&reg_array.0), JsValue::from_f64(*idx as f64)]
            })
            .flatten()
            .collect()
    }

    pub fn changed_memories(&self) -> Vec<JsValue> {
        let changed = match &self.kind {
            simulator::StepResultKind::Pipe(changed)
            | simulator::StepResultKind::StatementEnd(changed) => &changed.memories,
            _ => return Vec::new(),
        };
        changed
            .iter()
            .map(|(mem, addr)| [JsValue::from_str(&mem.0), JsValue::from_str(&addr.as_hex())])
            .flatten()
            .collect()
    }
}

impl From<rt_easy::simulator::StepResult> for StepResult {
    fn from(step_result: rt_easy::simulator::StepResult) -> Self {
        Self {
            statement: step_result.statement,
            span: step_result.span.into(),
            kind: step_result.kind,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct StepResultCondition {
    pub result: bool,
    pub span: Span,
}
