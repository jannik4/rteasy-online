use super::{Bit, Value};
use std::borrow::ToOwned;

#[derive(Debug, Eq)]
#[repr(C)]
pub struct ValueSlice {
    pub(crate) bits: [Bit],
}

impl ValueSlice {
    pub fn size(&self) -> usize {
        self.bits.len()
    }

    pub fn write(&mut self, value: &ValueSlice) {
        for (idx, bit) in self.bits.iter_mut().enumerate() {
            *bit = value.bits.get(idx).copied().unwrap_or_default();
        }
    }

    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|b| *b == Bit::Zero)
    }

    pub fn as_dec(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let mut value = self.to_owned();
        let mut result = Vec::new();

        while !value.is_zero() {
            let mut r = 0;
            let mut value_rest = Vec::new();

            for &b in value.bits.iter().rev() {
                r = 2 * r + u32::from(b);
                if r >= 10 {
                    value_rest.push(Bit::One);
                    r -= 10;
                } else {
                    value_rest.push(Bit::Zero);
                }
            }

            value = Value { bits: value_rest.into_iter().rev().collect() };
            value.remove_leading_zeros();
            result.push(char::from_digit(r, 10).unwrap());
        }

        result.into_iter().rev().collect()
    }
}

impl ToOwned for ValueSlice {
    type Owned = Value;

    fn to_owned(&self) -> Self::Owned {
        Value { bits: self.bits.into() }
    }
}
