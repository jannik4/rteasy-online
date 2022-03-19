use super::*;

pub type ConcatLvalueClocked = Concat<ConcatPartLvalueClocked>;
pub type ConcatLvalueUnclocked = Concat<ConcatPartLvalueUnclocked>;
pub type ConcatExpr = Concat<ConcatPartExpr>;

#[derive(Debug)]
pub struct Concat<P> {
    pub parts: Vec<P>,
}

#[derive(Debug)]
pub enum ConcatPartLvalueClocked {
    Register(Register, usize),
    RegisterArray(RegisterArray, usize),
}

#[derive(Debug)]
pub enum ConcatPartLvalueUnclocked {
    Bus(Bus, usize),
}

#[derive(Debug)]
pub enum ConcatPartExpr {
    Register(Register),
    Bus(Bus),
    RegisterArray(RegisterArray),
    Number(Number),
}
