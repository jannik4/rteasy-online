use super::util::slice_idx;
use crate::Error;
use rtcore::{
    common::{BitRange, RegisterKind},
    value::Value,
};
use std::cell::RefCell;

#[derive(Debug)]
pub struct RegisterState {
    range: BitRange,
    value: Value,
    value_next: RefCell<Option<Value>>,
    kind: RegisterKind,
}

impl RegisterState {
    pub fn init(range: Option<BitRange>, kind: RegisterKind) -> Self {
        let range = range.unwrap_or_default();
        Self { range, value: Value::zero(range.size()), value_next: RefCell::new(None), kind }
    }

    pub fn value_next(&self) -> Option<Value> {
        self.value_next.borrow().clone()
    }

    pub fn read(&self, idx: Option<BitRange>) -> Result<Value, Error> {
        let idx = match idx {
            Some(idx) => idx,
            None => return Ok(self.value.clone()),
        };

        let slice_idx = slice_idx(self.range, idx)?;
        Ok(self.value[slice_idx].to_owned())
    }

    pub fn write(&self, idx: Option<BitRange>, value: Value) -> Result<(), Error> {
        let mut target = self.value_next.borrow_mut();
        let target = target.get_or_insert_with(|| self.value.clone());

        let idx = match idx {
            Some(idx) => idx,
            None => {
                target.write(&value);
                return Ok(());
            }
        };

        let slice_idx = slice_idx(self.range, idx)?;
        target[slice_idx].write(&value);
        Ok(())
    }

    pub fn clock(&mut self) -> bool {
        match self.value_next.get_mut().take() {
            Some(value_next) => {
                self.value = value_next;
                true
            }
            None => false,
        }
    }

    pub fn range(&self) -> BitRange {
        self.range
    }

    pub fn kind(&self) -> RegisterKind {
        self.kind
    }
}
