use crate::Error;
use rtcore::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct RegisterArrayState {
    data: HashMap<Value, Value>,
    data_next: RefCell<Option<(Value, Value)>>,
    index_size: usize,
    data_size: usize,
}

impl RegisterArrayState {
    pub fn init(index_size: usize, data_size: usize) -> Self {
        Self { data: HashMap::new(), data_next: RefCell::new(None), index_size, data_size }
    }

    pub fn read(&self, idx: Value) -> Result<Value, Error> {
        debug_assert_eq!(idx.size(), self.index_size);

        let value = self.data.get(&idx).cloned().unwrap_or_else(|| Value::zero(self.data_size));
        Ok(value)
    }

    pub fn write(&self, idx: Value, value: Value) -> Result<(), Error> {
        debug_assert_eq!(idx.size(), self.index_size);
        debug_assert_eq!(value.size(), self.data_size);

        *self.data_next.borrow_mut() = Some((idx, value));
        Ok(())
    }

    pub fn clock(&mut self) {
        if let Some((idx, value)) = self.data_next.get_mut().take() {
            self.data.insert(idx, value);
        }
    }
}

impl fmt::Display for RegisterArrayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indexes = self.data.keys().collect::<Vec<_>>();
        indexes.sort();

        write!(f, "[\n")?;
        for idx in indexes {
            write!(f, "  {} = {}\n", idx.as_dec(), self.data.get(idx).unwrap().as_dec())?;
        }
        write!(f, "]")?;

        Ok(())
    }
}
