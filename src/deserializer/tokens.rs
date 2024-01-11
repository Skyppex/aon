use crate::representation::value::Number;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
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
    Word(String),
    EOF,
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