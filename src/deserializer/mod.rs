use crate::{representation::Value, error::AonError};

pub mod cursor;
pub mod tokens;
pub mod lexer;
pub mod parser;

pub fn deserialize(aon: &str) -> Result<Value, AonError> {
    let tokens = lexer::tokenize(aon)?;
    parser::parse_tokens(&tokens)
}