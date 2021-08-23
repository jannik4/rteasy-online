use std::cmp::{self, Ord, PartialOrd};
use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bit {
    Zero,
    One,
}

impl Default for Bit {
    fn default() -> Self {
        Self::Zero
    }
}

impl PartialOrd for Bit {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bit {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Bit::Zero, Bit::Zero) | (Bit::One, Bit::One) => cmp::Ordering::Equal,
            (Bit::Zero, Bit::One) => cmp::Ordering::Less,
            (Bit::One, Bit::Zero) => cmp::Ordering::Greater,
        }
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}

impl From<bool> for Bit {
    fn from(val: bool) -> Self {
        if val {
            Self::One
        } else {
            Self::Zero
        }
    }
}

impl From<Bit> for bool {
    fn from(val: Bit) -> Self {
        match val {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

macro_rules! impl_from_bit_into_num {
    ($num:ty) => {
        impl From<Bit> for $num {
            fn from(val: Bit) -> Self {
                match val {
                    Bit::Zero => 0,
                    Bit::One => 1,
                }
            }
        }
    };
}

impl_from_bit_into_num!(u8);
impl_from_bit_into_num!(u16);
impl_from_bit_into_num!(u32);
impl_from_bit_into_num!(u64);
impl_from_bit_into_num!(u128);
impl_from_bit_into_num!(i8);
impl_from_bit_into_num!(i16);
impl_from_bit_into_num!(i32);
impl_from_bit_into_num!(i64);
impl_from_bit_into_num!(i128);
