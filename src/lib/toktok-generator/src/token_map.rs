use crate::lexer::Token;
use logos::Logos;
use std::collections::HashMap;
use toktok::{
    combinator::{eoi, exact, many0, opt, pair, slice},
    Parser,
};

type State<'s, 't> = toktok::State<'s, 't, Token>;
type PResult<'s, 't, O> = toktok::PResult<'s, 't, Token, O>;

#[derive(Debug)]
pub struct TokenMap {
    map: HashMap<String, String>,
}

impl TokenMap {
    pub fn map(&self, token_short: &str) -> crate::Result<String> {
        Ok(self
            .map
            .get(token_short)
            .ok_or(format!("could not find token: {}", token_short))?
            .clone())
    }
}

pub fn parse_and_build(source: &str) -> crate::Result<TokenMap> {
    // Lex and parse
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(toktok::SpannedToken { token, span: lexer.span() });
    }
    let state = toktok::State::new(source, &tokens);
    let (_, pairs) = token_map_pairs(state)?;

    // Build map
    let mut token_map = TokenMap { map: HashMap::new() };
    for (token_short, token_path) in pairs {
        if let Some(t) = token_map
            .map
            .insert(token_short.to_string(), format!("__intern__::c::exact({})", token_path))
        {
            return Err(format!("duplicate token short: {}", t).into());
        }
    }
    Ok(token_map)
}

fn token_map_pairs<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, Vec<(&'s str, &'s str)>>
where
    's: 't,
{
    let (state, res) = many0(pair(
        pair(exact(Token::TokenShort), exact(Token::Assign)).map(|(a, _)| a),
        path,
    ))(state)?;
    let (state, _) = eoi(state)?;

    Ok((state, res))
}

fn path<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, &'s str>
where
    's: 't,
{
    let (state, (_, path)) = slice(pair(ident, opt(pair(exact(Token::DoubleColon), path))))(state)?;

    Ok((state, path))
}

fn ident<'s, 't>(state: State<'s, 't>) -> PResult<'s, 't, &'s str>
where
    's: 't,
{
    let (state, ident) = exact(Token::Identifier)(state)?;

    Ok((state, ident))
}
