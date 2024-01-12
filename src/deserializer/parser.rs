use crate::{representation::Value, error::{AonError, Result}};

use super::tokens::Token;

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Value> {
    parse_primitive(&mut tokens.clone())
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Value> {
    Ok(Value::Null)
}

fn parse_primitive(tokens: &mut Vec<Token>) -> Result<Value> {
    match first(&tokens) {
        Token::Word(string) => Ok(Value::String(string.to_owned())),
        Token::Number(number) => Ok(Value::Number(number.to_owned())),
        Token::Bool(boolean) => Ok(Value::Bool(*boolean)),
        Token::Null => Ok(Value::Null),
        Token::EOF => Err(AonError::UnexpectedEndOfFile),
        other => Err(AonError::UnexpectedToken(other.clone())),
    }
}

fn first(tokens: &Vec<Token>) -> &Token {
    tokens.get(0).unwrap_or(&Token::EOF)
}

fn is_eof(tokens: &Vec<Token>) -> bool {
    tokens.is_empty() || first(tokens) == &Token::EOF
}
