use super::State;
use crate::Error;
use rtcore::{program::MemoryRange, value::Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct MemoryState {
    data: HashMap<Value, Value>,
    data_next: RefCell<Option<(Value, Value)>>,
    range: MemoryRange,
    ar_size: usize,
    dr_size: usize,
}

impl MemoryState {
    pub fn init(range: MemoryRange, ar_size: usize, dr_size: usize) -> Self {
        Self { data: HashMap::new(), data_next: RefCell::new(None), range, ar_size, dr_size }
    }

    pub fn read(&self, state: &State) -> Result<(), Error> {
        // Get AR value
        let ar_value = state.register(&self.range.address_register)?.read(None)?;
        debug_assert_eq!(ar_value.size(), self.ar_size);

        // Read from memory
        let value = self.data.get(&ar_value).cloned().unwrap_or_else(|| Value::zero(self.dr_size));

        // Write into data_register
        state.register(&self.range.data_register)?.write(None, value)?;

        Ok(())
    }

    pub fn write(&self, state: &State) -> Result<(), Error> {
        // Get AR value
        let ar_value = state.register(&self.range.address_register)?.read(None)?;
        debug_assert_eq!(ar_value.size(), self.ar_size);

        // Get DR value
        let dr_value = state.register(&self.range.data_register)?.read(None)?;
        debug_assert_eq!(dr_value.size(), self.dr_size);

        // Write to memory
        *self.data_next.borrow_mut() = Some((ar_value, dr_value));

        Ok(())
    }

    pub fn clock(&mut self) {
        if let Some((ar_value, dr_value)) = self.data_next.get_mut().take() {
            self.data.insert(ar_value, dr_value);
        }
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
