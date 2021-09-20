use crate::SymbolType;
use rtcore::program::{BitRange, Span};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Errors(Vec<CompilerError>),
    Internal(InternalError),
    Backend(BackendError),
}

impl Error {
    pub fn pretty_print(&self, source: &str, file_name: Option<&str>, ansi_colors: bool) -> String {
        match self {
            Error::Errors(errors) => {
                // Sort errors
                let mut errors = errors.iter().collect::<Vec<_>>();
                errors.sort_by(|a, b| a.span.cmp(&b.span));

                // Pretty print all errors
                let mut result = String::new();
                for (idx, error) in errors.iter().enumerate() {
                    if idx != 0 {
                        result += "\n\n";
                    }
                    result += &error.pretty_print(source, file_name, ansi_colors);
                }
                result
            }
            Error::Internal(internal) => format!("{}", internal),
            Error::Backend(backend) => format!("{}", backend),
        }
    }
}

impl From<Vec<CompilerError>> for Error {
    fn from(errors: Vec<CompilerError>) -> Self {
        Self::Errors(errors)
    }
}

impl From<InternalError> for Error {
    fn from(internal: InternalError) -> Self {
        Self::Internal(internal)
    }
}

#[derive(Debug)]
pub struct CompilerError {
    pub kind: CompilerErrorKind,
    pub span: Span,
}

impl CompilerError {
    pub fn new(kind: CompilerErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn pretty_print(&self, source: &str, file_name: Option<&str>, ansi_colors: bool) -> String {
        let message = self.kind.message();
        let mut error = pretty_error::Error::new(&message)
            .with_source(source, pretty_error::Span::Range(self.span.range()))
            .with_ansi_colors(ansi_colors);
        if let Some(file_name) = file_name {
            error = error.with_file_name(file_name);
        }

        error.to_string()
    }
}

#[derive(Debug)]
pub enum CompilerErrorKind {
    DuplicateSymbol(String),
    RegArrayLenNotPowerOfTwo(String),
    RegArrayMissingIndex(String),
    DuplicateLabel(String),
    SymbolNotFound(&'static [SymbolType], String),
    LabelNotFound(String),
    ExpectedFixedSize,
    ExpectedConstantExpression,
    ExpectedExactlyOneDefaultClause,
    ConcatContainsNumberNonBitString,
    AssignmentDoesNotFit { lhs_size: usize, rhs_size: usize },
    RegArrayIndexDoesNotFit { index_size: usize, index_expr_size: usize },
    ConditionToWide(usize),
    BitRangeToWide { max_size: usize, size: usize },
    CaseValueTooWide { expr_size: usize, case_value_size: usize },
    DuplicateCaseValue,
    AssignmentLhsContainsClockedAndUnclocked,
    AssignmentLhsContainsANonLvalue,
    AssignmentLhsContainsInput,
    RangeMismatch { range: BitRange, range_idx: BitRange },
    GotoBeforePipe,
    MutateAfterPipe,
    SxtTerm,
    WrongSymbolType { expected: &'static [SymbolType], found: SymbolType },
    DoubleAssign(SymbolType, String),
    RegisterArrayTooManyReads { name: String, allowed: usize },
    FeedbackLoop,
}

impl CompilerErrorKind {
    fn message(&self) -> String {
        use CompilerErrorKind::*;

        fn fmt_symbol_types(symbol_types: &[SymbolType]) -> String {
            match symbol_types {
                [] => "".to_string(),
                [curr] => format!("{}", curr),
                [curr, last] => format!("{} or {}", curr, last),
                [curr, rest @ ..] => format!("{}, {}", curr, fmt_symbol_types(rest)),
            }
        }

        match self {
            DuplicateSymbol(name) => format!("duplicate symbol \"{}\"", name),
            RegArrayLenNotPowerOfTwo(name) => {
                format!("length of register array \"{}\" must be a power of two", name)
            }
            RegArrayMissingIndex(name) => {
                format!("register array \"{}\" is missing index [...]", name)
            }
            DuplicateLabel(name) => format!("duplicate label \"{}\"", name),
            SymbolNotFound(expected_symbol, name) => {
                format!("no {} named \"{}\" found", fmt_symbol_types(expected_symbol), name)
            }
            LabelNotFound(name) => format!("no label named \"{}\" found", name),
            ExpectedFixedSize => format!("expected fixed size expression"),
            ExpectedConstantExpression => format!("expected constant expression"),
            ExpectedExactlyOneDefaultClause => format!("expected exactly one default clause"),
            ConcatContainsNumberNonBitString => {
                format!("concat must not contain numbers other than bit strings")
            }
            AssignmentDoesNotFit { lhs_size, rhs_size } => {
                format!("right-hand side is too wide: {} > {}", rhs_size, lhs_size)
            }
            RegArrayIndexDoesNotFit { index_size, index_expr_size } => {
                format!("index expression is too wide: {} > {}", index_expr_size, index_size)
            }
            ConditionToWide(condition_size) => {
                format!(
                    "condition expression must be exactly one bit wide, but is: {}",
                    condition_size
                )
            }
            BitRangeToWide { max_size, size } => {
                format!("bit range size exceeds max size: {} > {}", size, max_size)
            }
            CaseValueTooWide { expr_size, case_value_size } => {
                format!("case value is too wide: {} > {}", case_value_size, expr_size)
            }
            DuplicateCaseValue => format!("duplicate case value"),
            AssignmentLhsContainsClockedAndUnclocked => {
                format!("the left-hand side of the assignment may contain either clocked or unclocked variables only")
            }
            AssignmentLhsContainsANonLvalue => {
                format!("the left-hand side of the assignment must be a variable")
            }
            AssignmentLhsContainsInput => format!("cannot assign to input (inputs are read-only)"),
            RangeMismatch { range, range_idx } => {
                format!("bit range {} exceeds declaration {}", range_idx, range)
            }
            GotoBeforePipe => format!("no goto statements are allowed before pipe (\"|\")"),
            MutateAfterPipe => format!("no mutating statements allowed after pipe (\"|\")"),
            SxtTerm => format!("sxt operator is not supported for terms"),
            WrongSymbolType { expected, found } => {
                format!("expected {}, found: {}", fmt_symbol_types(expected), found)
            }
            DoubleAssign(symbol_type, name) => {
                format!("{} \"{}\" is assigned more than once", symbol_type, name)
            }
            RegisterArrayTooManyReads { name, allowed } => {
                format!("register array \"{}\" is read more than {} times", name, allowed)
            }
            FeedbackLoop => format!("statement has a feedback loop"),
        }
    }
}

#[derive(Debug)]
pub struct InternalError(pub String);

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ICE: {}", self.0)
    }
}

impl From<CompilerError> for InternalError {
    fn from(err: CompilerError) -> Self {
        Self(format!("{:?}", err))
    }
}

#[derive(Debug)]
pub struct BackendError(pub Box<dyn std::error::Error + Send + Sync + 'static>);

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ICE (Backend): {}", self.0)
    }
}
