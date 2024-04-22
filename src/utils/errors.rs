use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScriptingError {
    #[error("Invalid Syntax: {0}")]
    InvalidSyntax(String),
    #[error("Invalid Token: {0}")]
    InvalidToken(String),
    #[error("Error while parsing: {0}")]
    ParsingError(#[from] std::num::ParseFloatError),
    #[error("Unexpected token")]
    UnexpectedToken,
}

pub type Result<T> = std::result::Result<T, ScriptingError>;
