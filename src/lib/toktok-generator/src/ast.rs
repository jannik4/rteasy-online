#[derive(Debug)]
pub struct Ast<'a> {
    pub rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
pub struct Rule<'a> {
    pub is_public: bool,
    pub name: Ident<'a>,
    pub ret_type: RetType<'a>,
    pub productions: Vec<Production<'a>>,
}

#[derive(Debug)]
pub struct Production<'a> {
    pub combinators: Vec<Combinator<'a>>,
    pub rust_block: RustBlock<'a>,
    pub is_fallible: bool,
    pub source: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Combinator<'a> {
    pub kind: CombinatorKind<'a>,
    pub source: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum CombinatorKind<'a> {
    Path(Path<'a>),
    TokenShort(TokenShort<'a>),
    FunctionCall(FunctionCall<'a>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall<'a> {
    pub name: Path<'a>,
    pub args: Vec<Combinator<'a>>,
}

#[derive(Debug)]
pub struct RetType<'a>(pub &'a str);

#[derive(Debug)]
pub struct RustBlock<'a>(pub &'a str);

#[derive(Debug, PartialEq)]
pub struct TokenShort<'a>(pub &'a str);

#[derive(Debug, PartialEq)]
pub struct Path<'a>(pub &'a str);

#[derive(Debug)]
pub struct Ident<'a>(pub &'a str);
