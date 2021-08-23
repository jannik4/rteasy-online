#![deny(rust_2018_idioms)]

mod lexer;
mod parser;

pub use self::lexer::Token;

pub fn parse(source: &str) -> Result<rtcore::ast::Ast<'_>, toktok::Error<Token>> {
    use logos::Logos;

    // Lex
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(toktok::SpannedToken { token, span: lexer.span() });
    }

    // Parse
    let state = toktok::State::new(source, &tokens);
    let (_, ast) = parser::ast(state)?;

    Ok(ast)
}

pub fn pretty_print_error(error: &toktok::Error<Token>, source: &str) -> String {
    let options = toktok::PrettyPrintOptions {
        source: Some(source),
        file_name: None,
        rename_token: Some(Box::new(|token: &Token| {
            use Token::*;
            match token {
                Semicolon => "\";\"".to_string(),
                Colon => "\":\"".to_string(),
                Comma => "\",\"".to_string(),
                Pipe => "\"|\"".to_string(),
                Dot => "\".\"".to_string(),
                ParenOpen => "\"(\"".to_string(),
                ParenClose => "\")\"".to_string(),
                Assign => "\"<-\"".to_string(),

                _ => format!("{:?}", token), // TODO: ...
            }
        })),
    };
    error.pretty_print(&options)
}
