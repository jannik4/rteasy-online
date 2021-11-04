use crate::unit_test::UnitTest;

mod lexer;
mod parser;

pub use self::lexer::Token;

pub fn parse(source: &str) -> Result<UnitTest, toktok::Error<Token>> {
    use logos::Logos;

    // Lex
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(toktok::SpannedToken { token, span: lexer.span() });
    }

    // Parse
    let state = toktok::State::new(source, &tokens);
    let (_, unit_test) = parser::unit_test(state)?;

    Ok(unit_test)
}

pub fn pretty_print_error(
    error: &toktok::Error<Token>,
    source: &str,
    file_name: Option<&str>,
    ansi_colors: bool,
) -> String {
    let options = toktok::PrettyPrintOptions {
        source: Some(source),
        file_name,
        ansi_colors,
        rename_token: Some(Box::new(|token: &toktok::TokenOrEoi<Token>| {
            use Token::*;

            let token = match token {
                toktok::TokenOrEoi::Eoi => return "<EOI>".to_string(),
                toktok::TokenOrEoi::Token(token) => token,
            };

            match token {
                OperationAssignment => "<ASSIGNMENT>",
                OperationAssert => "<ASSERT>",

                LiteralNumberDec => "<NUM_DEC>",

                KeywordStep => "\"step\"",
                KeywordMicroStep => "\"microStep\"",
                KeywordRun => "\"run\"",
                KeywordReset => "\"reset\"",
                KeywordSet => "\"set\"",
                KeywordRemove => "\"remove\"",
                KeywordBreakpoint => "\"breakpoint\"",

                Identifier => "<ID>",

                Newline => "<NEWLINE>",

                Error => "<ERROR>",
            }
            .to_string()
        })),
        filter_expected: Some(Box::new(|expected: &[toktok::TokenOrEoi<Token>]| {
            use std::collections::HashSet;

            // Filter duplicates
            let expected = expected.into_iter().copied().collect::<HashSet<_>>();

            // Sort
            let mut expected = expected.into_iter().collect::<Vec<_>>();
            expected.sort();

            expected
        })),
    };
    error.pretty_print(&options)
}
