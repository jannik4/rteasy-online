use super::{Bit, ValueSlice};
use std::borrow::{Borrow, BorrowMut};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Eq)]
pub struct Value {
    pub(crate) bits: Vec<Bit>,
}

impl Value {
    pub fn zero(size: usize) -> Self {
        assert!(size != 0);
        Self { bits: (0..size).map(|_| Bit::Zero).collect() }
    }

    pub fn one(size: usize) -> Self {
        assert!(size != 0);
        Self { bits: (0..size).map(|i| if i == 0 { Bit::One } else { Bit::Zero }).collect() }
    }

    pub fn filled(size: usize) -> Self {
        assert!(size != 0);
        Self { bits: (0..size).map(|_| Bit::One).collect() }
    }

    pub fn remove_leading_zeros(&mut self) {
        while self.bits.len() > 1 {
            if self.bits.last() == Some(&Bit::Zero) {
                self.bits.pop().unwrap();
            } else {
                break;
            }
        }
    }

    pub fn extend_zero(&mut self, size: usize) {
        while size > self.bits.len() {
            self.bits.push(Bit::Zero);
        }
    }

    pub fn extend_sign(&mut self, size: usize) {
        let sign = *self.bits.last().unwrap();
        while size > self.bits.len() {
            self.bits.push(sign);
        }
    }

    pub fn with_size(mut self, size: usize) -> Self {
        assert!(size != 0);
        while self.bits.len() < size {
            self.bits.push(Bit::Zero);
        }
        while self.bits.len() > size {
            self.bits.pop();
        }
        self
    }

    /// Parse from binary string. Leading zeros from input are retained.
    ///
    /// # Errors
    ///
    /// Errors if `bin` is empty or contains any other char then `[01]`.
    pub fn parse_bin(bin: &str) -> Result<Self, ()> {
        if bin.is_empty() {
            return Err(());
        }

        let value = Self {
            bits: bin
                .chars()
                .rev()
                .map(|c| match c {
                    '0' => Ok(Bit::Zero),
                    '1' => Ok(Bit::One),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()?,
        };

        Ok(value)
    }

    /// Parse from decimal string. The result will have no leading zeros.
    ///
    /// # Errors
    ///
    /// Errors if `dec` is empty or contains any other char then `[0-9]`.
    pub fn parse_dec(dec: &str) -> Result<Self, ()> {
        if dec.is_empty() {
            return Err(());
        }

        let mut bits = Vec::new();

        let mut dec =
            dec.chars().map(|c| c.to_digit(10).ok_or(())).collect::<Result<Vec<_>, _>>()?;
        let mut dec = dec.as_mut_slice();

        while !dec.is_empty() {
            bits.push((dec.last().unwrap() % 2 == 1).into());

            let mut additive = 0;
            for i in 0..dec.len() {
                let next_addive = if dec[i] % 2 == 1 { 5 } else { 0 };
                dec[i] = dec[i] / 2 + additive;
                additive = next_addive;
            }

            while !dec.is_empty() && dec[0] == 0 {
                dec = &mut dec[1..];
            }
        }

        let mut value = Self { bits };
        value.remove_leading_zeros();
        Ok(value)
    }

    /// Parse from hexadecimal string. The result will have no leading zeros.
    ///
    /// # Errors
    ///
    /// Errors if `hex` is empty or contains any other char then `[0-9a-fA-F]`.
    pub fn parse_hex(hex: &str) -> Result<Self, ()> {
        if hex.is_empty() {
            return Err(());
        }

        let mut bits = Vec::new();

        for hex in hex.chars().rev() {
            let val = hex.to_digit(16).ok_or(())?;
            bits.push((val & 0b0001 != 0).into());
            bits.push((val & 0b0010 != 0).into());
            bits.push((val & 0b0100 != 0).into());
            bits.push((val & 0b1000 != 0).into());
        }

        let mut value = Self { bits };
        value.remove_leading_zeros();
        Ok(value)
    }

    pub fn concat<'a, I>(slices: I) -> Self
    where
        I: IntoIterator<Item = &'a ValueSlice>,
        I::IntoIter: DoubleEndedIterator,
    {
        let mut bits = Vec::new();

        for slice in slices.into_iter().rev() {
            bits.extend_from_slice(&slice.bits);
        }

        Self { bits }
    }

    pub fn as_slice(&self) -> &ValueSlice {
        &**self
    }

    pub fn as_mut_slice(&mut self) -> &mut ValueSlice {
        &mut **self
    }
}

impl From<Bit> for Value {
    fn from(bit: Bit) -> Self {
        Self { bits: vec![bit] }
    }
}

impl AsRef<ValueSlice> for Value {
    fn as_ref(&self) -> &ValueSlice {
        unsafe { &*(self.bits.as_slice() as *const [Bit] as *const ValueSlice) }
    }
}

impl AsMut<ValueSlice> for Value {
    fn as_mut(&mut self) -> &mut ValueSlice {
        unsafe { &mut *(self.bits.as_mut_slice() as *mut [Bit] as *mut ValueSlice) }
    }
}

impl Deref for Value {
    type Target = ValueSlice;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl Borrow<ValueSlice> for Value {
    fn borrow(&self) -> &ValueSlice {
        &*self
    }
}

impl BorrowMut<ValueSlice> for Value {
    fn borrow_mut(&mut self) -> &mut ValueSlice {
        &mut *self
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bin() {
        assert_eq!(
            Value::parse_bin("1101").unwrap(),
            Value { bits: vec![Bit::One, Bit::Zero, Bit::One, Bit::One] }
        );

        assert_eq!(
            Value::parse_bin("000101").unwrap(),
            Value { bits: vec![Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::Zero, Bit::Zero,] }
        );

        assert_eq!(Value::parse_bin("01").unwrap().size(), 2);
        assert_eq!(Value::parse_bin("0101").unwrap().size(), 4);
        assert_eq!(Value::parse_bin("1010").unwrap().size(), 4);

        assert!(Value::parse_bin("-000101").is_err());
        assert!(Value::parse_bin("0b000101").is_err());
        assert!(Value::parse_bin("").is_err());
        assert!(Value::parse_bin("01020").is_err());
    }

    #[test]
    fn test_parse_dec() {
        assert_eq!(
            Value::parse_dec("791").unwrap(),
            Value {
                bits: vec![
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                ]
            }
        );

        assert_eq!(Value::parse_dec("7").unwrap().size(), 3);
        assert_eq!(Value::parse_dec("007").unwrap().size(), 3);

        assert!(Value::parse_dec("-495783").is_err());
        assert!(Value::parse_dec("FFa").is_err());
        assert!(Value::parse_dec("FF").is_err());
        assert!(Value::parse_dec("").is_err());
        assert!(Value::parse_dec("12 89").is_err());
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(
            Value::parse_hex("ffA1").unwrap(),
            Value {
                bits: vec![
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                ]
            }
        );

        assert_eq!(Value::parse_hex("0").unwrap().size(), 1);
        assert_eq!(Value::parse_hex("a").unwrap().size(), 4);
        assert_eq!(Value::parse_hex("0a").unwrap().size(), 4);

        assert!(Value::parse_hex("-ff").is_err());
        assert!(Value::parse_hex("FFaG").is_err());
        assert!(Value::parse_hex("").is_err());
        assert!(Value::parse_hex("ff 12").is_err());
    }
}
