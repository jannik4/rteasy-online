use logos::Logos;

#[derive(Logos, Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Token {
    #[token(";")]
    Semicolon,
    #[token("::")]
    DoubleColon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("<")]
    LeftAngle,
    #[token(">")]
    RightAngle,
    #[token("=>")]
    FatArrow,
    #[token("->")]
    Arrow,
    #[token("'")]
    SingleQuote,
    #[token("=")]
    Assign,
    #[token("?")]
    QuestionMark,

    #[token("pub")]
    KeywordPublic,
    #[token("use")]
    KeywordUse,
    #[token("as")]
    KeywordAs,

    #[token("|")]
    OperatorChoice,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r#""[^"]*""#)]
    TokenShort,

    #[token("{", parse_rust_block)]
    RustBlock,

    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    #[regex(r"//[^\r\n]*", logos::skip)]
    Error,
}

fn parse_rust_block(lex: &mut logos::Lexer<'_, Token>) -> Result<(), ()> {
    let mut balance = 1;
    let mut bump = 0;

    for c in lex.remainder().chars() {
        bump += c.len_utf8();
        match c {
            '{' => balance += 1,
            '}' => balance -= 1,
            _ => (),
        }

        if balance == 0 {
            lex.bump(bump);
            return Ok(());
        }
    }

    Err(())
}
