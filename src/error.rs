use std::fmt::Display;

use crate::deserializer::tokens::Token;

pub type Result<T> = std::result::Result<T, AonError>;

#[derive(Debug, Clone, PartialEq)]
pub enum AonError {
    UnexpectedCharacter(char),
    InvalidNumber(String),
    UnexpectedEndOfFile,
    UnexpectedToken(Token)
}

impl Display for AonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AonError::UnexpectedCharacter(char) => write!(f, "Unexpected character: {}", char),
            AonError::InvalidNumber(number) => write!(f, "Invalid number format: {}", number),
            AonError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            AonError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            
        }
    }
}