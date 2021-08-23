use super::registers::RegistersState;
use crate::{ChangeSet, Error};
use rtcore::{
    program::{Declaration, Ident, MemoryRange, Program, Register},
    value::Value,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct MemoriesState {
    memories: HashMap<Ident, MemoryState>,
}

impl MemoriesState {
    pub fn init(program: &Program, registers: &RegistersState) -> Self {
        let mut memories = HashMap::new();

        for declaration in program.declarations() {
            if let Declaration::Memory(declare_memory) = declaration {
                for mem in &declare_memory.memories {
                    let ar_size = registers.range_of(&mem.range.address_register).unwrap().size();
                    let dr_size = registers.range_of(&mem.range.data_register).unwrap().size();
                    memories.insert(
                        mem.ident.clone(),
                        MemoryState::init(mem.range.clone(), ar_size, dr_size),
                    );
                }
            }
        }

        Self { memories }
    }

    pub fn read(
        &self,
        name: &Ident,
        registers: &RegistersState,
        change_set: &mut ChangeSet,
    ) -> Result<(), Error> {
        match self.memories.get(name) {
            Some(state) => state.read(registers, change_set),
            None => Err(Error::Other),
        }
    }

    pub fn write(&mut self, name: &Ident, registers: &RegistersState) -> Result<(), Error> {
        match self.memories.get_mut(name) {
            Some(state) => state.write(registers),
            None => Err(Error::Other),
        }
    }
}

impl fmt::Display for MemoriesState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut memories = self.memories.keys().collect::<Vec<_>>();
        memories.sort();

        for (idx, mem) in memories.into_iter().enumerate() {
            write!(
                f,
                "{}{} = {}",
                if idx != 0 { "\n" } else { "" },
                mem.0,
                self.memories.get(mem).unwrap()
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct MemoryState {
    data: HashMap<Value, Value>,
    range: MemoryRange,
    ar_size: usize,
    dr_size: usize,
}

impl MemoryState {
    fn init(range: MemoryRange, ar_size: usize, dr_size: usize) -> Self {
        Self { data: HashMap::new(), range, ar_size, dr_size }
    }

    fn read(&self, registers: &RegistersState, change_set: &mut ChangeSet) -> Result<(), Error> {
        // Get AR value
        let ar_value = registers.read_full(&self.range.address_register)?;
        debug_assert_eq!(ar_value.size(), self.ar_size);

        // Read from memory
        let value = self.data.get(&ar_value).cloned().unwrap_or_else(|| Value::zero(self.dr_size));

        // Write into data_register
        change_set.write_into_register(
            Register { ident: self.range.data_register.clone(), range: None },
            value,
        )?;

        Ok(())
    }

    fn write(&mut self, registers: &RegistersState) -> Result<(), Error> {
        // Get AR value
        let ar_value = registers.read_full(&self.range.address_register)?;
        debug_assert_eq!(ar_value.size(), self.ar_size);

        // Get DR value
        let dr_value = registers.read_full(&self.range.data_register)?;
        debug_assert_eq!(dr_value.size(), self.dr_size);

        // Write to memory
        self.data.insert(ar_value, dr_value);

        Ok(())
    }
}

impl fmt::Display for MemoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut addresses = self.data.keys().collect::<Vec<_>>();
        addresses.sort();

        write!(f, "[\n")?;
        for addr in addresses {
            write!(f, "  {} = {}\n", addr.as_dec(), self.data.get(addr).unwrap().as_dec())?;
        }
        write!(f, "]")?;

        Ok(())
    }
}
