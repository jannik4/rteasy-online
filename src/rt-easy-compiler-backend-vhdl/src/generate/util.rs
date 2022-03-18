use crate::vhdl::*;
use compiler::mir;
use std::collections::HashMap;

/// Since all criteria are combined in the VHDL code,
/// the MIR criteria IDs must be mapped to the new ones.
#[derive(Debug)]
pub struct CriteriaMapping(HashMap<mir::CriterionId, CriterionId>);

impl CriteriaMapping {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, from: mir::CriterionId, to: CriterionId) {
        self.0.insert(from, to);
    }

    /// All `mir_criteria` must be in the CriteriaMapping.
    ///
    /// # Panics
    ///
    /// Panics if any criterion in `mir_criteria` is not in the CriteriaMapping.
    pub fn map(&self, mir_criteria: &[mir::Criterion]) -> Vec<Criterion> {
        mir_criteria
            .iter()
            .map(|criterion| match criterion {
                mir::Criterion::True(id) => Criterion::True(*self.0.get(id).unwrap()),
                mir::Criterion::False(id) => Criterion::False(*self.0.get(id).unwrap()),
            })
            .collect()
    }
}
