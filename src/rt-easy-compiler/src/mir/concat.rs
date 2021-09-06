use super::*;

pub type ConcatLvalueClocked<'s> = Concat<ConcatPartLvalueClocked<'s>>;
pub type ConcatLvalueUnclocked<'s> = Concat<ConcatPartLvalueUnclocked<'s>>;
pub type ConcatExpr<'s> = Concat<ConcatPartExpr<'s>>;

#[derive(Debug, Clone)]
pub struct Concat<P> {
    pub parts: Vec<P>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ConcatPartLvalueClocked<'s> {
    Register(Register<'s>, usize),
    RegisterArray(RegisterArray<'s>, usize),
}

impl ConcatPartLvalueClocked<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Register(n, _) => n.span,
            Self::RegisterArray(n, _) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConcatPartLvalueUnclocked<'s> {
    Bus(Bus<'s>, usize),
}

impl ConcatPartLvalueUnclocked<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Bus(n, _) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConcatPartExpr<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Spanned<Number>),
}

impl ConcatPartExpr<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Register(n) => n.span,
            Self::Bus(n) => n.span,
            Self::RegisterArray(n) => n.span,
            Self::Number(n) => n.span,
        }
    }
}
