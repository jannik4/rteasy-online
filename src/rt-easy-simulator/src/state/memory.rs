use super::State;
use crate::Error;
use rtcore::{program::MemoryRange, value::Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

const MEMORY_PAGE_SIZE_EXP: usize = 5;
const MEMORY_PAGE_SIZE: usize = 2usize.pow(MEMORY_PAGE_SIZE_EXP as u32);

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

    pub fn write_at(&mut self, addr: Value, value: Value) -> Result<(), Error> {
        // Check addr and value
        if addr.size() > self.ar_size || value.size() > self.dr_size {
            return Err(Error::Other);
        }

        // Reset data_next if same address
        if let Some((data_next_addr, _)) = self.data_next.get_mut() {
            if *data_next_addr == addr {
                *self.data_next.get_mut() = None;
            }
        }

        // Insert data
        self.data.insert(addr, value);

        Ok(())
    }

    pub fn page_count(&self) -> Value {
        if self.ar_size <= MEMORY_PAGE_SIZE_EXP {
            Value::one(1)
        } else {
            Value::one(self.ar_size + 1) << (self.ar_size - MEMORY_PAGE_SIZE_EXP)
        }
    }

    pub fn page_prev(&self, page_nr: Value) -> Option<Value> {
        if page_nr <= Value::one(1) {
            return None;
        }

        Some(page_nr - Value::one(1))
    }

    pub fn page_next(&self, page_nr: Value) -> Option<Value> {
        if page_nr >= self.page_count() {
            return None;
        }

        // Prevent overflow
        let size = page_nr.size() + 1;
        let page_nr = page_nr.with_size(size);

        Some(page_nr + Value::one(1))
    }

    pub fn page(&self, page_nr: Value) -> Vec<(Value, Value)> {
        // Check in range (1..=page_count)
        if page_nr < Value::one(1) || page_nr > self.page_count() {
            return Vec::new();
        }

        // Page idx
        let page_idx = page_nr - Value::one(1);

        // Calc addr (with size = ar_size)
        let mut addr = page_idx.with_size(self.ar_size) << MEMORY_PAGE_SIZE_EXP;

        // Read from memory
        let mut result = Vec::new();
        for _ in 0..MEMORY_PAGE_SIZE {
            let addr_next = &addr + Value::one(1);
            let value = self.data.get(&addr).cloned().unwrap_or_else(|| Value::zero(self.dr_size));
            result.push((addr, value));
            addr = addr_next;

            // Break on overflow
            if addr.is_zero() {
                break;
            }
        }
        result
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
