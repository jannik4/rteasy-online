use crate::{ast, lexer::Token};
use logos::Logos;
use toktok::{
    combinator::{alt, delimited, eoi, exact, many0, many1, opt, pair, sep0, sep1, slice},
    Parser,
};

type State<'s, 't> = toktok::State<'s, 't, Token>;
type PResult<'s, 't, O> = toktok::PResult<'s, 't, Token, O>;

pub fn parse(source: &str) -> Result<ast::Ast<'_>, toktok::Error<Token>> {
    // Lex and parse
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(toktok::SpannedToken { token, span: lexer.span() });
    }
    let state = toktok::State::new(source, &tokens);
    let (_, ast) = ast(state)?;

    Ok(ast)
}

fn ast<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::Ast<'s>>
where
    's: 't,
{
    let (state, rules) = many0(rule)(state)?;
    let (state, _) = eoi(state)?;

    Ok((state, ast::Ast { rules }))
}

fn rule<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::Rule<'s>>
where
    's: 't,
{
    let (state, public) = opt(exact(Token::KeywordPublic))(state)?;
    let (state, name) = ident(state)?;
    let (state, _) = exact(Token::Arrow)(state)?;
    let (state, ret_type) = ret_type(state)?;
    let (state, _) = exact(Token::Colon)(state)?;
    let (state, productions) = sep1(production, exact(Token::OperatorChoice))(state)?;
    let (state, _) = exact(Token::Semicolon)(state)?;

    Ok((state, ast::Rule { is_public: public.is_some(), name, ret_type, productions }))
}

fn production<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::Production<'s>>
where
    's: 't,
{
    let span_start = state.input().positioned_start();

    let (state, combinators) = many1(combinator)(state)?;
    let (state, rust_block) = rust_block(state)?;
    let (state, question_mark) = opt(exact(Token::QuestionMark))(state)?;

    let span_end = state.input().positioned_end(span_start);
    let source = &state.input().source()[span_start..span_end];

    Ok((
        state,
        ast::Production { combinators, rust_block, is_fallible: question_mark.is_some(), source },
    ))
}

fn combinator<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::Combinator<'s>>
where
    's: 't,
{
    let span_start = state.input().positioned_start();

    let (state, combinator_kind) = alt(
        alt(
            function_call.map(ast::CombinatorKind::FunctionCall),
            token_short.map(ast::CombinatorKind::TokenShort),
        ),
        path.map(ast::CombinatorKind::Path),
    )(state)?;

    let span_end = state.input().positioned_end(span_start);
    let source = &state.input().source()[span_start..span_end];

    Ok((state, ast::Combinator { kind: combinator_kind, source }))
}

fn function_call<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::FunctionCall<'s>>
where
    's: 't,
{
    let (state, name) = path(state)?;
    let (state, args) = delimited(
        exact(Token::LeftParen),
        sep0(combinator, exact(Token::Comma)),
        exact(Token::RightParen),
    )(state)?;

    Ok((state, ast::FunctionCall { name, args }))
}

fn ret_type<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::RetType<'s>>
where
    's: 't,
{
    let (state, (_, ret_type)) = slice(many1(alt(
        alt(
            alt(
                alt(exact(Token::LeftAngle), exact(Token::RightAngle)).map(|_| ()),
                path.map(|_| ()),
            ),
            alt(exact(Token::SingleQuote), exact(Token::Comma)).map(|_| ()),
        ),
        alt(exact(Token::LeftParen), exact(Token::RightParen)).map(|_| ()),
    )))(state)?;

    Ok((state, ast::RetType(ret_type)))
}

fn rust_block<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::RustBlock<'s>>
where
    's: 't,
{
    let (state, rust_block) = exact(Token::RustBlock)(state)?;

    Ok((state, ast::RustBlock(rust_block)))
}

fn token_short<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::TokenShort<'s>>
where
    's: 't,
{
    let (state, token_short) = exact(Token::TokenShort)(state)?;

    Ok((state, ast::TokenShort(token_short)))
}

fn path<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::Path<'s>>
where
    's: 't,
{
    let (state, (_, path)) = slice(pair(ident, opt(pair(exact(Token::DoubleColon), path))))(state)?;

    Ok((state, ast::Path(path)))
}

fn ident<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, ast::Ident<'s>>
where
    's: 't,
{
    let (state, ident) = exact(Token::Identifier)(state)?;

    Ok((state, ast::Ident(ident)))
}
