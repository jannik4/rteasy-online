use super::util::slice_idx;
use crate::Error;
use rtcore::{
    common::{BitRange, BusKind},
    value::Value,
};
use std::cell::RefCell;

#[derive(Debug)]
pub struct BusState {
    range: BitRange,
    value: RefCell<Value>,
    kind: BusKind,
}

impl BusState {
    pub fn init(range: Option<BitRange>, kind: BusKind) -> Self {
        let range = range.unwrap_or_default();
        Self { range, value: RefCell::new(Value::zero(range.size())), kind }
    }

    pub fn read(&self, idx: Option<BitRange>) -> Result<Value, Error> {
        let idx = match idx {
            Some(idx) => idx,
            None => return Ok(self.value.borrow().clone()),
        };

        let slice_idx = slice_idx(self.range, idx)?;
        Ok(self.value.borrow()[slice_idx].to_owned())
    }

    pub fn write(&self, idx: Option<BitRange>, value: Value) -> Result<(), Error> {
        let mut target = self.value.borrow_mut();

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

    pub fn range(&self) -> BitRange {
        self.range
    }

    pub fn kind(&self) -> BusKind {
        self.kind
    }
}
