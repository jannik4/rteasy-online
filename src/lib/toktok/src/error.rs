use crate::{Input, Span, State};
use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct StateError<T>(Option<Error<T>>);

impl<T> StateError<T> {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn unwrap(self) -> ParserError<T> {
        ParserError(self.0.unwrap())
    }

    pub fn and(self, err: Error<T>) -> ParserError<T> {
        ParserError(Error::merge(self.0, err))
    }
}

#[derive(Debug)]
pub struct ParserError<T>(Error<T>);

impl<T> ParserError<T> {
    pub fn recover<'s, 't>(self, input: Input<'s, 't, T>) -> Result<State<'s, 't, T>, Self> {
        if self.0.is_fail {
            Err(self)
        } else {
            Ok(State::from_parts(input, StateError(Some(self.0))))
        }
    }

    pub fn inore_fail(mut self) -> Self {
        self.0.is_fail = false;
        self
    }

    pub fn with_is_fail(mut self) -> Self {
        self.0.is_fail = true;
        self
    }

    pub fn is_fail(&self) -> bool {
        self.0.is_fail
    }
}

impl<T> fmt::Display for ParserError<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> StdError for ParserError<T>
where
    T: fmt::Debug,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

#[derive(Debug)]
pub struct Error<T> {
    span: Span,
    is_fail: bool,
    kind: ErrorKind<T>,
}

#[derive(Debug)]
pub enum ErrorKind<T> {
    Expected(Vec<TokenOrEoi<T>>, TokenOrEoi<T>),
    ExpectedNegative(Box<dyn StdError>),
    Custom(Box<dyn StdError>),
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TokenOrEoi<T> {
    Token(T),
    Eoi,
}

impl<T> Error<T> {
    pub fn new_custom(span: impl Into<Span>, err: Box<dyn StdError>) -> Self {
        Self { span: span.into(), is_fail: false, kind: ErrorKind::Custom(err) }
    }

    pub fn new_expected(
        span: impl Into<Span>,
        expected: impl IntoIterator<Item = TokenOrEoi<T>>,
        found: TokenOrEoi<T>,
    ) -> Self {
        Self {
            span: span.into(),
            is_fail: false,
            kind: ErrorKind::Expected(expected.into_iter().collect(), found),
        }
    }

    pub fn new_expected_negative(span: impl Into<Span>, err: Box<dyn StdError>) -> Self {
        Self { span: span.into(), is_fail: false, kind: ErrorKind::ExpectedNegative(err) }
    }

    pub fn with_is_fail(mut self) -> Self {
        self.is_fail = true;
        self
    }

    pub fn is_fail(&self) -> bool {
        self.is_fail
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn kind(&self) -> &ErrorKind<T> {
        &self.kind
    }

    pub fn pretty_print(&self, options: &PrettyPrintOptions<'_, T>) -> String
    where
        T: fmt::Debug,
    {
        let message = match &self.kind {
            ErrorKind::Expected(expected, found) => {
                let mut message = String::new();
                message += "found: ";
                message += &token_str(found, options.rename_token.as_deref());
                message += ", expected one of: ";
                for (idx, token) in expected.iter().enumerate() {
                    message += &token_str(token, options.rename_token.as_deref());
                    if idx != expected.len() - 1 {
                        message += ", ";
                    }
                }
                message
            }
            ErrorKind::ExpectedNegative(err) => err.to_string(),
            ErrorKind::Custom(err) => err.to_string(),
        };

        let mut error = pretty_error::Error::new(&message);
        if let Some(source) = options.source {
            let span = match &self.span {
                Span::Eoi => pretty_error::Span::Eoi,
                Span::Range(range) => pretty_error::Span::Range(range.clone()),
            };
            error = error.with_source(source, span);
        }
        if let Some(file_name) = options.file_name {
            error = error.with_file_name(file_name);
        }

        error.to_string()
    }

    fn merge(e1: Option<Self>, e2: Self) -> Self {
        let e1 = match e1 {
            Some(e1) => e1,
            None => return e2,
        };

        match e1.cmp(&e2) {
            Ordering::Greater => e1,
            Ordering::Less => e2,
            Ordering::Equal => {
                debug_assert_eq!(e1.span, e2.span);
                Self {
                    span: e1.span,
                    kind: match (e1.kind, e2.kind) {
                        (ErrorKind::Custom(err1), ErrorKind::Custom(_err2)) => {
                            ErrorKind::Custom(err1)
                        }
                        (
                            ErrorKind::Expected(expected1, found1),
                            ErrorKind::Expected(expected2, _found2),
                        ) => ErrorKind::Expected(
                            // TODO: Filter duplicates
                            vec![expected1.into_iter(), expected2.into_iter()]
                                .into_iter()
                                .flatten()
                                .collect(),
                            found1,
                        ),
                        (ErrorKind::ExpectedNegative(err1), ErrorKind::ExpectedNegative(_err2)) => {
                            ErrorKind::ExpectedNegative(err1)
                        }
                        (_, ErrorKind::Custom(err)) | (ErrorKind::Custom(err), _) => {
                            ErrorKind::Custom(err)
                        }
                        (_, ErrorKind::ExpectedNegative(err))
                        | (ErrorKind::ExpectedNegative(err), _) => ErrorKind::ExpectedNegative(err),
                    },
                    is_fail: e1.is_fail || e2.is_fail,
                }
            }
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.span, &other.span) {
            (Span::Eoi, Span::Eoi) => Ordering::Equal,
            (Span::Eoi, Span::Range(_)) => Ordering::Greater,
            (Span::Range(_), Span::Eoi) => Ordering::Less,
            (Span::Range(r1), Span::Range(r2)) => match r1.end.cmp(&r2.end) {
                Ordering::Equal => r1.start.cmp(&r2.start),
                ordering => ordering,
            },
        }
    }
}

impl<T> From<ParserError<T>> for Error<T> {
    fn from(e: ParserError<T>) -> Self {
        e.0
    }
}

impl<T> fmt::Display for Error<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print(&Default::default()))
    }
}

impl<T> StdError for Error<T>
where
    T: fmt::Debug,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.kind {
            ErrorKind::Expected(_, _) => None,
            ErrorKind::ExpectedNegative(err) => Some(&**err),
            ErrorKind::Custom(err) => Some(&**err),
        }
    }
}

pub struct PrettyPrintOptions<'a, T> {
    pub source: Option<&'a str>,
    pub file_name: Option<&'a str>,
    pub rename_token: Option<Box<dyn Fn(&T) -> String>>,
}

impl<T> Default for PrettyPrintOptions<'_, T> {
    fn default() -> Self {
        Self { source: None, file_name: None, rename_token: None }
    }
}

fn token_str<T>(token: &TokenOrEoi<T>, rename_token: Option<&dyn Fn(&T) -> String>) -> String
where
    T: fmt::Debug,
{
    match token {
        TokenOrEoi::Eoi => "EOI".to_string(),
        TokenOrEoi::Token(token) => match rename_token {
            Some(rename_token) => rename_token(token),
            None => format!("{:?}", token),
        },
    }
}
