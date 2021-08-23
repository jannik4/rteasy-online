use super::{Bit, Value, ValueSlice};
use std::cmp::{self, Ord, PartialEq, PartialOrd};
use std::ops::{Add, Index, IndexMut, Neg, Not, Sub};

// ------------------------------------------------------------------
// Cmp
// ------------------------------------------------------------------

fn eq(lhs: &ValueSlice, rhs: &ValueSlice) -> bool {
    let len = cmp::max(lhs.bits.len(), rhs.bits.len());

    for i in 0..len {
        if lhs.bits.get(i).copied().unwrap_or_default()
            != rhs.bits.get(i).copied().unwrap_or_default()
        {
            return false;
        }
    }

    true
}

// TODO: This assumes positive values
fn cmp(lhs: &ValueSlice, rhs: &ValueSlice) -> cmp::Ordering {
    let mut idx = cmp::max(lhs.bits.len(), rhs.bits.len()) - 1;

    loop {
        let lhs = lhs.bits.get(idx).copied().unwrap_or_default();
        let rhs = rhs.bits.get(idx).copied().unwrap_or_default();
        let res = lhs.cmp(&rhs);

        if res != cmp::Ordering::Equal {
            break res;
        }

        if idx == 0 {
            break cmp::Ordering::Equal;
        } else {
            idx -= 1;
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        cmp(self, other)
    }
}

impl Ord for ValueSlice {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        cmp(self, other)
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs:ty) => {
        impl PartialEq<$rhs> for $lhs {
            fn eq(&self, other: &$rhs) -> bool {
                eq(self, other)
            }
        }

        impl PartialOrd<$rhs> for $lhs {
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                Some(cmp(self, other))
            }
        }
    };
}

impl_cmp!(Value, Value);
impl_cmp!(Value, ValueSlice);
impl_cmp!(ValueSlice, Value);
impl_cmp!(ValueSlice, ValueSlice);

// ------------------------------------------------------------------
// Binaray Ops
// ------------------------------------------------------------------

fn add(lhs: &ValueSlice, rhs: &ValueSlice) -> Value {
    let mut result = Vec::new();
    let mut carry = Bit::Zero;

    let len = cmp::max(lhs.bits.len(), rhs.bits.len());
    for i in 0..len {
        let lhs = lhs.bits.get(i).copied().unwrap_or_default();
        let rhs = rhs.bits.get(i).copied().unwrap_or_default();

        let (res, carry_next) = match (lhs, rhs, carry) {
            (Bit::Zero, Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::Zero),
            (Bit::Zero, Bit::Zero, Bit::One) => (Bit::One, Bit::Zero),
            (Bit::Zero, Bit::One, Bit::Zero) => (Bit::One, Bit::Zero),
            (Bit::Zero, Bit::One, Bit::One) => (Bit::Zero, Bit::One),
            (Bit::One, Bit::Zero, Bit::Zero) => (Bit::One, Bit::Zero),
            (Bit::One, Bit::Zero, Bit::One) => (Bit::Zero, Bit::One),
            (Bit::One, Bit::One, Bit::Zero) => (Bit::Zero, Bit::One),
            (Bit::One, Bit::One, Bit::One) => (Bit::One, Bit::One),
        };

        result.push(res);
        carry = carry_next;
    }

    Value { bits: result }
}

fn sub(lhs: &ValueSlice, rhs: &ValueSlice) -> Value {
    let mut result = Vec::new();
    let mut carry = Bit::Zero;

    let len = cmp::max(lhs.bits.len(), rhs.bits.len());
    for i in 0..len {
        let lhs = lhs.bits.get(i).copied().unwrap_or_default();
        let rhs = rhs.bits.get(i).copied().unwrap_or_default();

        let (res, carry_next) = match (lhs, rhs, carry) {
            (Bit::Zero, Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::Zero),
            (Bit::Zero, Bit::Zero, Bit::One) => (Bit::One, Bit::One),
            (Bit::Zero, Bit::One, Bit::Zero) => (Bit::One, Bit::One),
            (Bit::Zero, Bit::One, Bit::One) => (Bit::Zero, Bit::One),
            (Bit::One, Bit::Zero, Bit::Zero) => (Bit::One, Bit::Zero),
            (Bit::One, Bit::Zero, Bit::One) => (Bit::Zero, Bit::Zero),
            (Bit::One, Bit::One, Bit::Zero) => (Bit::Zero, Bit::Zero),
            (Bit::One, Bit::One, Bit::One) => (Bit::One, Bit::One),
        };

        result.push(res);
        carry = carry_next;
    }

    Value { bits: result }
}

macro_rules! impl_binary_ops {
    ($lhs:ty, $rhs:ty) => {
        impl Add<$rhs> for $lhs {
            type Output = Value;

            fn add(self, rhs: $rhs) -> Self::Output {
                add(&self, &rhs)
            }
        }

        impl Sub<$rhs> for $lhs {
            type Output = Value;

            fn sub(self, rhs: $rhs) -> Self::Output {
                sub(&self, &rhs)
            }
        }
    };
}

impl_binary_ops!(Value, Value);
impl_binary_ops!(Value, &Value);
impl_binary_ops!(Value, &ValueSlice);
impl_binary_ops!(&Value, Value);
impl_binary_ops!(&Value, &Value);
impl_binary_ops!(&Value, &ValueSlice);
impl_binary_ops!(&ValueSlice, Value);
impl_binary_ops!(&ValueSlice, &Value);
impl_binary_ops!(&ValueSlice, &ValueSlice);

// ------------------------------------------------------------------
// Unary Ops
// ------------------------------------------------------------------

fn neg(self_: &ValueSlice) -> Value {
    !self_ + Value::one(1)
}

fn not(self_: &ValueSlice) -> Value {
    Value { bits: self_.bits.into_iter().copied().map(Not::not).collect() }
}

macro_rules! impl_unary_ops {
    ($self_:ty) => {
        impl Neg for $self_ {
            type Output = Value;

            fn neg(self) -> Self::Output {
                neg(&self)
            }
        }

        impl Not for $self_ {
            type Output = Value;

            fn not(self) -> Self::Output {
                not(&self)
            }
        }
    };
}

impl_unary_ops!(Value);
impl_unary_ops!(&Value);
impl_unary_ops!(&ValueSlice);

// ------------------------------------------------------------------
// Index
// ------------------------------------------------------------------

impl<I> Index<I> for ValueSlice
where
    [Bit]: Index<I, Output = [Bit]>,
{
    type Output = ValueSlice;

    fn index(&self, index: I) -> &Self::Output {
        unsafe { &*(&self.bits[index] as *const [Bit] as *const ValueSlice) }
    }
}

impl<I> IndexMut<I> for ValueSlice
where
    [Bit]: Index<I, Output = [Bit]>,
    [Bit]: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        unsafe { &mut *(&mut self.bits[index] as *mut [Bit] as *mut ValueSlice) }
    }
}
