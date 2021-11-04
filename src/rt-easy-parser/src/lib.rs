#![deny(rust_2018_idioms)]

mod lexer;
mod parser;

pub use self::lexer::Token;

fn lex(source: &str) -> Result<Vec<toktok::SpannedToken<Token>>, toktok::Error<Token>> {
    use logos::Logos;

    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(toktok::SpannedToken { token, span: lexer.span() });
    }

    Ok(tokens)
}

pub fn parse(source: &str) -> Result<rtcore::ast::Ast<'_>, toktok::Error<Token>> {
    let tokens = lex(source)?;
    let state = toktok::State::new(source, &tokens);
    let (_, ast) = parser::ast_eoi(state)?;

    Ok(ast)
}

pub fn parse_assignment(source: &str) -> Result<rtcore::ast::Assignment<'_>, toktok::Error<Token>> {
    let tokens = lex(source)?;
    let state = toktok::State::new(source, &tokens);
    let (_, assignment) = parser::assignment_eoi(state)?;

    Ok(assignment)
}

pub fn parse_assert(source: &str) -> Result<rtcore::ast::Assert<'_>, toktok::Error<Token>> {
    let tokens = lex(source)?;
    let state = toktok::State::new(source, &tokens);
    let (_, assert) = parser::assert_eoi(state)?;

    Ok(assert)
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
                Semicolon => "\";\"",
                Colon => "\":\"",
                Comma => "\",\"",
                Pipe => "\"|\"",
                Dot => "\".\"",
                ParenOpen => "\"(\"",
                ParenClose => "\")\"",
                BracketOpen => "\"[\"",
                BracketClose => "\"]\"",
                BraceOpen => "\"{\"",
                BraceClose => "\"}\"",
                Assign => "\"<-\"",

                LiteralNumberBin => "<NUM_BIN>",
                LiteralNumberHex => "<NUM_HEX>",
                LiteralNumberDec => "<NUM_DEC>",
                LiteralNumberBitString => "<NUM_BIT_STRING>",

                KeywordDeclare => "\"declare\"",
                KeywordInput => "\"input\"",
                KeywordOutput => "\"output\"",
                KeywordRegister => "\"register\"",
                KeywordBus => "\"bus\"",
                KeywordMemory => "\"memory\"",
                KeywordArray => "\"array\"",
                KeywordNop => "\"nop\"",
                KeywordGoto => "\"goto\"",
                KeywordRead => "\"read\"",
                KeywordWrite => "\"write\"",
                KeywordIf => "\"if\"",
                KeywordThen => "\"then\"",
                KeywordElse => "\"else\"",
                KeywordFi => "\"fi\"",
                KeywordSwitch => "\"switch\"",
                KeywordCase => "\"case\"",
                KeywordDefault => "\"default\"",
                KeywordAssert => "\"assert\"",

                OperatorEquality => "\"=\"",
                OperatorInequality => "\"<>\"",
                OperatorLessEquals => "\"<=\"",
                OperatorLess => "\"<\"",
                OperatorGreaterEquals => "\">=\"",
                OperatorGreater => "\">\"",
                OperatorAddition => "\"+\"",
                OperatorSubtraction => "\"-\"",
                OperatorAnd => "\"and\"",
                OperatorNand => "\"nand\"",
                OperatorOr => "\"or\"",
                OperatorNor => "\"nor\"",
                OperatorXor => "\"xor\"",

                OperatorNeg => "\"neg\"",
                OperatorNot => "\"not\"",
                OperatorSxt => "\"sxt\"",

                Identifier => "<ID>",

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
