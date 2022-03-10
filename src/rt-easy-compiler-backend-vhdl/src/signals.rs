use crate::render_as_rt::RenderAsRt;

#[derive(Debug)]
pub struct Signals {
    pub condition_signals: Vec<String>,
    pub control_signals: Vec<String>,
}

impl Signals {
    pub(crate) fn new(vhdl: &crate::Vhdl<'_>) -> Self {
        Self {
            condition_signals: vhdl
                .criteria
                .iter()
                .map(|expression| RenderAsRt(expression).to_string())
                .collect(),
            control_signals: vhdl
                .operations
                .iter()
                .map(|operation| RenderAsRt(operation).to_string())
                .collect(),
        }
    }
}
