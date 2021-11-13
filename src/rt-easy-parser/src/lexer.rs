use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Token {
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("|")]
    Pipe,
    #[token(".")]
    Dot,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("<-")]
    Assign,

    #[regex("(%|0[bB])[01]+")]
    LiteralNumberBin,
    #[regex(r"(\$|0[xX])[0-9a-fA-F]+")]
    LiteralNumberHex,
    #[regex("[0-9]+")]
    LiteralNumberDec,
    #[regex("\"[01]+\"")]
    LiteralNumberBitString,

    #[token("declare")]
    KeywordDeclare,
    #[token("input")]
    KeywordInput,
    #[token("output")]
    KeywordOutput,
    #[token("register")]
    KeywordRegister,
    #[token("bus")]
    KeywordBus,
    #[token("memory")]
    KeywordMemory,
    #[token("array")]
    KeywordArray,
    #[token("nop")]
    KeywordNop,
    #[token("goto")]
    KeywordGoto,
    #[token("read")]
    KeywordRead,
    #[token("write")]
    KeywordWrite,
    #[token("if")]
    KeywordIf,
    #[token("then")]
    KeywordThen,
    #[token("else")]
    KeywordElse,
    #[token("fi")]
    KeywordFi,
    #[token("switch")]
    KeywordSwitch,
    #[token("case")]
    KeywordCase,
    #[token("default")]
    KeywordDefault,
    #[token("assert")]
    KeywordAssert,

    // Binary Operators
    #[token("=")]
    OperatorEquality,
    #[token("<>")]
    OperatorInequality,
    #[token("<=")]
    OperatorLessEquals,
    #[token("<")]
    OperatorLess,
    #[token(">=")]
    OperatorGreaterEquals,
    #[token(">")]
    OperatorGreater,
    #[token("+")]
    OperatorAddition,
    #[token("-")]
    OperatorSubtraction,
    #[token("and")]
    OperatorAnd,
    #[token("nand")]
    OperatorNand,
    #[token("or")]
    OperatorOr,
    #[token("nor")]
    OperatorNor,
    #[token("xor")]
    OperatorXor,

    // Unary Operators
    // #[token("-")] OperatorSign, <-- Already matched by OperatorSubtraction
    #[token("neg")]
    OperatorNeg,
    #[token("not")]
    OperatorNot,
    #[token("sxt")]
    OperatorSxt,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", is_valid_ident)]
    Identifier,

    #[error]
    #[regex(r"[ \t\r\n]+", logos::skip)]
    #[regex(r"#[^\r\n]*", logos::skip)]
    Error,
}

fn is_valid_ident(lex: &mut Lexer<'_, Token>) -> bool {
    lex.slice().chars().all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
        && lex.slice().chars().any(|c| c != '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let code = r#"A tru TesT TEST iF if ifX ifx"#;

        let _tokens = Token::lexer(code).spanned().collect::<Vec<_>>();
        // panic!("{:#?}", _tokens);
    }

    #[test]
    fn test_all() {
        let code = r#"
        # comment
        #qwertz
        if(x) $$a thenfi fi then(fi); ifx declare(x) # declarex declare
        "#;

        let _tokens = Token::lexer(code).spanned().collect::<Vec<_>>();
        // panic!("{:#?}", _tokens);
    }
}
