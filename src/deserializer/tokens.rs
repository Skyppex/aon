use std::fmt::Display;

use crate::representation::value::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Comment(String),
    SingleQuote,
    DoubleQuote,
    Colon,
    Comma,
    Hash,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Null,
    Bool(bool),
    Number(Number),

    // Can be the value of a string, but can also be a key in a struct.
    Word(String),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Comment(c) => write!(f, "//{}", c),
            Token::SingleQuote => write!(f, "'"),
            Token::DoubleQuote => write!(f, "\""),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Hash => write!(f, "#"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Null => write!(f, "null"),
            Token::Bool(bool) => write!(f, "{}", bool),
            Token::Number(number) => write!(f, "{}", number),
            Token::Word(string) => write!(f, "\"{}\"", string),
            Token::EOF => write!(f, ""),
        }
    }
}


pub(crate) const SINGLE_QUOTE: char = '\'';
pub(crate) const DOUBLE_QUOTE: char = '"';
pub(crate) const SLASH: char = '/';

pub(crate) const COLON: char = ':';
pub(crate) const COMMA: char = ',';
pub(crate) const HASH: char = '#';

pub(crate) const LEFT_BRACE: char = '{';
pub(crate) const RIGHT_BRACE: char = '}';

pub(crate) const LEFT_BRACKET: char = '[';
pub(crate) const RIGHT_BRACKET: char = ']';

pub(crate) const NULL: &str = "null";
pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";