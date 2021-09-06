use super::*;

#[derive(Debug, Clone)]
pub enum Expression<'s> {
    Atom(Atom<'s>),
    BinaryTerm(Box<BinaryTerm<'s>>),
    UnaryTerm(Box<UnaryTerm<'s>>),
}

impl Expression<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Atom(n) => n.span(),
            Self::BinaryTerm(n) => n.span,
            Self::UnaryTerm(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Atom<'s> {
    Concat(ConcatExpr<'s>),
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Spanned<Number>),
}

impl Atom<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Concat(n) => n.span,
            Self::Register(n) => n.span,
            Self::Bus(n) => n.span,
            Self::RegisterArray(n) => n.span,
            Self::Number(n) => n.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: Spanned<BinaryOperator>,
    pub ctx_size: CtxSize,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UnaryTerm<'s> {
    pub expression: Expression<'s>,
    pub operator: Spanned<UnaryOperator>,
    pub ctx_size: CtxSize,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Register<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: Option<Spanned<BitRange>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Bus<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub range: Option<Spanned<BitRange>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RegisterArray<'s> {
    pub ident: Spanned<Ident<'s>>,
    pub index: Box<Expression<'s>>,
    pub index_ctx_size: usize,
    pub span: Span,
}
