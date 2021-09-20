use std::ops::Range;
use value::Value;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOperator {
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
    Add,
    Sub,
    And,
    Nand,
    Or,
    Nor,
    Xor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    SignNeg,
    Not,
    Sxt,
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: Value,
    pub kind: NumberKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberKind {
    BitString,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterKind {
    Intern,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusKind {
    Intern,
    Input,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BitRange {
    pub msb: usize,
    pub lsb: Option<usize>,
}

impl BitRange {
    pub fn lsb(&self) -> usize {
        self.lsb.unwrap_or(self.msb)
    }

    pub fn msb_lsb(&self) -> (usize, usize) {
        (self.msb, self.lsb())
    }

    pub fn size(&self) -> usize {
        let lsb = self.lsb();
        if self.msb >= lsb {
            self.msb - lsb + 1
        } else {
            lsb - self.msb + 1
        }
    }

    pub fn contains(&self, pos: usize) -> bool {
        let lsb = self.lsb();
        if self.msb >= lsb {
            pos >= lsb && pos <= self.msb
        } else {
            pos >= self.msb && pos <= lsb
        }
    }

    pub fn contains_range(&self, idx: Self) -> bool {
        let contains_msb = self.contains(idx.msb);
        let contains_lsb = self.contains(idx.lsb());
        let msb_lsb_order = if idx.lsb.is_some() {
            (self.msb >= self.lsb()) == (idx.msb >= idx.lsb())
        } else {
            true
        };

        contains_msb && contains_lsb && msb_lsb_order
    }

    pub fn bits(&self) -> impl Iterator<Item = usize> {
        let lsb = self.lsb();
        let msb_ge_lsb = self.msb >= lsb;
        let size = self.size();

        (0..size).into_iter().map(move |idx| if msb_ge_lsb { lsb + idx } else { lsb - idx })
    }

    pub fn intersect(a: Option<Self>, b: Option<Self>) -> bool {
        match (a, b) {
            (None, _) | (_, None) => true,
            (Some(a), Some(b)) => {
                a.contains(b.msb) || a.contains(b.lsb()) || b.contains(a.msb) || b.contains(a.lsb())
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CtxSize {
    Size(usize),
    Inherit,
}

impl CtxSize {
    pub fn calc(self, parent: usize) -> usize {
        match self {
            Self::Size(size) => size,
            Self::Inherit => parent,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn range(self) -> Range<usize> {
        self.start..self.end
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}
