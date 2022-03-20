use crate::{error::RenderError, Declarations, Ident};
use memory_file::{MemoryFile, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MemoryData {
    pub data: Vec<(Value, Value)>,
}

pub fn memories(
    memories: HashMap<Ident, MemoryFile>,
    declarations: &Declarations,
) -> Result<HashMap<Ident, MemoryData>, RenderError> {
    memories
        .into_iter()
        .map(|(name, file)| {
            // Find memory in declarations
            let (_, ar, dr) = declarations
                .memories
                .iter()
                .find(|(n, _, _)| *n == name)
                .ok_or_else(|| RenderError::MemoryNotFound(name.clone()))?;

            // Check size
            if file.ar_size() > ar.1.size() || file.dr_size() > dr.1.size() {
                return Err(RenderError::InvalidMemorySize {
                    name,
                    expected: (ar.1.size(), dr.1.size()),
                    actual: (file.ar_size(), file.dr_size()),
                });
            }

            // Sort data (by address ASC)
            let mut data = file.into_data().into_iter().collect::<Vec<_>>();
            data.sort_by(|a, b| a.0.cmp(&b.0));

            // Memory data
            Ok((name, MemoryData { data }))
        })
        .collect()
}
