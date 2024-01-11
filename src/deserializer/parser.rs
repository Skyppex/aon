use crate::representation::Value;

use super::tokens::Token;

pub fn parse_tokens(tokens: &mut Vec<Token>) -> Result<Value, String> {
    parse_primitive(tokens)
}

fn parse_primitive(tokens: &mut Vec<Token>) -> Result<Value, String> {
    match first(&tokens) {
        Token::Word(string) => Ok(Value::String(string.to_owned())),
        Token::Number(number) => Ok(Value::Number(number.to_owned())),
        Token::Bool(boolean) => Ok(Value::Bool(*boolean)),
        Token::Null => Ok(Value::Null),
        Token::EOF => Err("Unexpected EOF".to_owned()),
        other => Err(format!("Unexpected token: {:?}", other)),
    }
}

fn first(tokens: &Vec<Token>) -> &Token {
    tokens.get(0).unwrap_or(&Token::EOF)
}

fn is_eof(tokens: &Vec<Token>) -> bool {
    tokens.is_empty() || first(tokens) == &Token::EOF
}
