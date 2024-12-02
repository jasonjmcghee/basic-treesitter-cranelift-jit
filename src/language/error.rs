use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Calculator error: {kind}")]
#[diagnostic(code(calculator::error))]
pub struct CalculatorError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label]
    pub span: SourceSpan,

    pub kind: CalcErrorKind,

    #[help]
    pub help: Option<String>,
}

#[derive(Error, Debug)]
pub enum CalcErrorKind {
    #[error("Invalid syntax")]
    ParseError(String),

    #[error("Type mismatch: {0}")]
    TypeMismatch(String),

    #[error("Invalid operator: {0}")]
    InvalidOperator(String),

    #[error("Compilation error: {0}")]
    CompilationError(String),

    #[error("JIT error: {0}")]
    JitError(String),

    #[error("Invalid number: {0}")]
    NumberError(String),

    #[error("System error: {0}")]
    SystemError(String),
}