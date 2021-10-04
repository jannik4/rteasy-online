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
