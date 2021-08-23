use crate::SymbolType;

#[derive(Debug)]
pub enum Error {
    Errors(Vec<CompilerError>),
    Internal(InternalError),
    Backend(Box<dyn std::error::Error + Send + Sync + 'static>),
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
pub enum CompilerError {
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
    AssignmentDoesNotFit(usize, usize),
    RegArrayIndexDoesNotFit(usize, usize),
    ConditionToWide(usize),
    BitRangeToWide(usize),
    CaseValueTooWide,
    DuplicateCaseValue,
    AssignmentLhsContainsClockedAndUnclocked,
    AssignmentLhsContainsANonLvalue,
    RangeMismatch,
    GotoBeforePipe,
    MutateAfterPipe,
    SxtTerm,
    WrongSymbolType { expected: &'static [SymbolType], found: SymbolType },
    DoubleAssign(SymbolType, String),
    FeedbackLoop,
}

#[derive(Debug)]
pub struct InternalError(pub String);

impl From<CompilerError> for InternalError {
    fn from(err: CompilerError) -> Self {
        Self(format!("{:?}", err))
    }
}
