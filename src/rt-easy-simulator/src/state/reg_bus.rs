use crate::Error;
use rtcore::{program::BitRange, value::Value};
use std::ops::Range;

#[derive(Debug)]
pub struct RegBusState {
    value: Value,
    range: BitRange,
}

impl RegBusState {
    pub fn init(range: Option<BitRange>) -> Self {
        let range = range.unwrap_or_default();
        Self { value: Value::zero(range.size()), range }
    }

    pub fn read_full(&self) -> Value {
        self.value.clone()
    }

    pub fn read(&self, idx: Option<BitRange>) -> Result<Value, Error> {
        let idx = match idx {
            Some(idx) => idx,
            None => return Ok(self.read_full().into()),
        };

        let slice_idx = self.slice_idx(idx)?;
        Ok(self.value[slice_idx].to_owned())
    }

    pub fn write(&mut self, idx: Option<BitRange>, value: Value) -> Result<(), Error> {
        let idx = match idx {
            Some(idx) => idx,
            None => {
                self.value.write(&value);
                return Ok(());
            }
        };

        let slice_idx = self.slice_idx(idx)?;
        self.value[slice_idx].write(&value);
        Ok(())
    }

    fn slice_idx(&self, idx: BitRange) -> Result<Range<usize>, Error> {
        if !self.range.contains_range(idx) {
            return Err(Error::Other);
        }

        let (self_msb, self_lsb) = self.range.msb_lsb();
        let (idx_msb, idx_lsb) = idx.msb_lsb();

        let slice_idx = if self_msb >= self_lsb {
            let start = idx_lsb - self_lsb;
            let end = idx_msb - self_lsb + 1;
            start..end
        } else {
            let start = self_lsb - idx_lsb;
            let end = self_lsb - idx_msb + 1;
            start..end
        };

        Ok(slice_idx)
    }

    pub fn range(&self) -> BitRange {
        self.range
    }
}
