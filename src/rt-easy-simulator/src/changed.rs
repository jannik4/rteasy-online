use rtcore::value::Value;
use rtprogram::Ident;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Changed {
    /// Registers (name)
    pub registers: HashSet<Ident>,
    /// Register arrays (name + index)
    pub register_arrays: HashSet<(Ident, usize)>,
    /// Memories (name + address)
    pub memories: HashSet<(Ident, Value)>,
}

impl Changed {
    pub fn extend(&mut self, other: Self) {
        self.registers.extend(other.registers);
        self.register_arrays.extend(other.register_arrays);
        self.memories.extend(other.memories);
    }
}
