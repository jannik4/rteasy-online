use super::*;

pub type ConcatLvalueClocked<'s> = Concat<ConcatPartLvalueClocked<'s>>;
pub type ConcatLvalueUnclocked<'s> = Concat<ConcatPartLvalueUnclocked<'s>>;
pub type ConcatExpr<'s> = Concat<ConcatPartExpr<'s>>;

#[derive(Debug)]
pub struct Concat<P> {
    pub parts: Vec<P>,
}

#[derive(Debug)]
pub enum ConcatPartLvalueClocked<'s> {
    Register(Register<'s>, usize),
    RegisterArray(RegisterArray<'s>, usize),
}

#[derive(Debug)]
pub enum ConcatPartLvalueUnclocked<'s> {
    Bus(Bus<'s>, usize),
}

#[derive(Debug)]
pub enum ConcatPartExpr<'s> {
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}
