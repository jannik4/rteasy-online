use super::*;

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn precedence(&self) -> u32 {
        match &self.kind {
            ExpressionKind::Atom(_) => u32::MAX,
            ExpressionKind::BinaryTerm(binary) => binary.operator.precedence(),
            ExpressionKind::UnaryTerm(unary) => unary.operator.precedence(),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Atom(Atom),
    BinaryTerm(Box<BinaryTerm>),
    UnaryTerm(Box<UnaryTerm>),
}

#[derive(Debug)]
pub enum Atom {
    Concat(ConcatExpr),
    Register(Register),
    Bus(Bus),
    RegisterArray(RegisterArray),
    Number(Number),
}

#[derive(Debug)]
pub struct BinaryTerm {
    pub lhs: Expression,
    pub rhs: Expression,
    pub operator: BinaryOperator,
    pub ctx_size: CtxSize,
}

#[derive(Debug)]
pub struct UnaryTerm {
    pub expression: Expression,
    pub operator: UnaryOperator,
    pub ctx_size: CtxSize,
}

#[derive(Debug, Clone)]
pub struct Register {
    pub ident: Ident,
    pub range: Option<BitRange>,
    pub kind: RegisterKind,
}

#[derive(Debug)]
pub struct Bus {
    pub ident: Ident,
    pub range: Option<BitRange>,
    pub kind: BusKind,
}

#[derive(Debug)]
pub struct RegisterArray {
    pub ident: Ident,
    pub index: Box<Expression>,
    pub index_ctx_size: usize,
}
