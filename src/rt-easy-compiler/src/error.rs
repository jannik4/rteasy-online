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
        let message = self.kind.to_string();
        let error_code = format!("[E{:03}]", self.kind.idx());

        let mut error = pretty_error::Error::new(&message)
            .with_error_code(&error_code)
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
    ConditionTooWide(usize),
    BitRangeTooWide { max_size: usize, size: usize },
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
    DoubleGoto,
    RegisterArrayTooManyReads { name: String, allowed: usize },
    FeedbackLoop,
}

impl CompilerErrorKind {
    fn idx(&self) -> usize {
        use CompilerErrorKind::*;

        match self {
            DuplicateSymbol(_) => 1,
            RegArrayLenNotPowerOfTwo(_) => 2,
            RegArrayMissingIndex(_) => 3,
            DuplicateLabel(_) => 4,
            SymbolNotFound(_, _) => 5,
            LabelNotFound(_) => 6,
            ExpectedFixedSize => 7,
            ExpectedConstantExpression => 8,
            ExpectedExactlyOneDefaultClause => 9,
            ConcatContainsNumberNonBitString => 10,
            AssignmentDoesNotFit { .. } => 11,
            RegArrayIndexDoesNotFit { .. } => 12,
            ConditionTooWide(_) => 13,
            BitRangeTooWide { .. } => 14,
            CaseValueTooWide { .. } => 15,
            DuplicateCaseValue => 16,
            AssignmentLhsContainsClockedAndUnclocked => 17,
            AssignmentLhsContainsANonLvalue => 18,
            AssignmentLhsContainsInput => 19,
            RangeMismatch { .. } => 20,
            GotoBeforePipe => 21,
            MutateAfterPipe => 22,
            SxtTerm => 23,
            WrongSymbolType { .. } => 24,
            DoubleAssign(_, _) => 25,
            DoubleGoto => 26,
            RegisterArrayTooManyReads { .. } => 27,
            FeedbackLoop => 28,
        }
    }
}

impl fmt::Display for CompilerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompilerErrorKind::*;

        struct DisplaySymbolTypes<'a>(&'a [SymbolType]);

        impl fmt::Display for DisplaySymbolTypes<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    [] => Ok(()),
                    [curr] => write!(f, "{}", curr),
                    [curr, last] => write!(f, "{} or {}", curr, last),
                    [curr, rest @ ..] => write!(f, "{}, {}", curr, DisplaySymbolTypes(rest)),
                }
            }
        }

        match self {
            DuplicateSymbol(name) => write!(f, "duplicate symbol \"{}\"", name),
            RegArrayLenNotPowerOfTwo(name) => {
                write!(f, "length of register array \"{}\" must be a power of two", name)
            }
            RegArrayMissingIndex(name) => {
                write!(f, "register array \"{}\" is missing index [...]", name)
            }
            DuplicateLabel(name) => write!(f, "duplicate label \"{}\"", name),
            SymbolNotFound(expected_symbol, name) => {
                write!(f, "no {} named \"{}\" found", DisplaySymbolTypes(expected_symbol), name)
            }
            LabelNotFound(name) => write!(f, "no label named \"{}\" found", name),
            ExpectedFixedSize => write!(f, "expected fixed size expression"),
            ExpectedConstantExpression => write!(f, "expected constant expression"),
            ExpectedExactlyOneDefaultClause => write!(f, "expected exactly one default clause"),
            ConcatContainsNumberNonBitString => {
                write!(f, "concat must not contain numbers other than bit strings")
            }
            AssignmentDoesNotFit { lhs_size, rhs_size } => {
                write!(f, "right-hand side is too wide: {} > {}", rhs_size, lhs_size)
            }
            RegArrayIndexDoesNotFit { index_size, index_expr_size } => {
                write!(f, "index expression is too wide: {} > {}", index_expr_size, index_size)
            }
            ConditionTooWide(condition_size) => {
                write!(
                    f,
                    "condition expression must be exactly one bit wide, but is: {}",
                    condition_size
                )
            }
            BitRangeTooWide { max_size, size } => {
                write!(f, "bit range size exceeds max size: {} > {}", size, max_size)
            }
            CaseValueTooWide { expr_size, case_value_size } => {
                write!(f, "case value is too wide: {} > {}", case_value_size, expr_size)
            }
            DuplicateCaseValue => write!(f, "duplicate case value"),
            AssignmentLhsContainsClockedAndUnclocked => {
                write!(f, "the left-hand side of the assignment may contain either clocked or unclocked variables only")
            }
            AssignmentLhsContainsANonLvalue => {
                write!(f, "the left-hand side of the assignment must be a variable")
            }
            AssignmentLhsContainsInput => {
                write!(f, "cannot assign to input (inputs are read-only)")
            }
            RangeMismatch { range, range_idx } => {
                write!(f, "bit range {} exceeds declaration {}", range_idx, range)
            }
            GotoBeforePipe => write!(f, "no goto statements are allowed before pipe (\"|\")"),
            MutateAfterPipe => write!(f, "no mutating statements allowed after pipe (\"|\")"),
            SxtTerm => write!(f, "sxt operator is not supported for terms"),
            WrongSymbolType { expected, found } => {
                write!(f, "expected {}, found: {}", DisplaySymbolTypes(expected), found)
            }
            DoubleAssign(symbol_type, name) => {
                write!(f, "{} \"{}\" is assigned more than once", symbol_type, name)
            }
            DoubleGoto => {
                write!(
                    f,
                    "statement contains multiple gotos on at least one possible execution path"
                )
            }
            RegisterArrayTooManyReads { name, allowed } => {
                write!(f, "register array \"{}\" is read more than {} times", name, allowed)
            }
            FeedbackLoop => write!(f, "statement has a feedback loop"),
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
