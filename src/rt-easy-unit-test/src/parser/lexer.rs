use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Token {
    #[regex(r"[A-Z][^<\r\n]*<-[^\r\n]*")]
    OperationAssignment,
    #[regex(r"assert[^\r\n]*")]
    OperationAssert,

    #[regex("[0-9]+")]
    LiteralNumberDec,

    #[token("step")]
    KeywordStep,
    #[token("microStep")]
    KeywordMicroStep,
    #[token("run")]
    KeywordRun,
    #[token("reset")]
    KeywordReset,
    #[token("set")]
    KeywordSet,
    #[token("remove")]
    KeywordRemove,
    #[token("breakpoint")]
    KeywordBreakpoint,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", is_valid_ident)]
    Identifier,

    #[regex(r"[\r\n]+")]
    Newline,

    #[error]
    #[regex(r"[ \t]+", logos::skip)]
    #[regex(r"#[^\r\n]*", logos::skip)]
    Error,
}

fn is_valid_ident(lex: &mut Lexer<'_, Token>) -> bool {
    lex.slice().chars().all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
        && lex.slice().chars().any(|c| c != '_')
}
