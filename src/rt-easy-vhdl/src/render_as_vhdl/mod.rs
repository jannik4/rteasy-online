mod bit_range;
mod concat;
mod criteria;
mod expression;
mod operation;

#[derive(Debug)]
pub struct RenderAsVhdl<T>(pub T);
