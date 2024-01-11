use std::vec;

use crate::representation::value::Number;

use super::{cursor, tokens::{Token, self}};

pub fn parse_node(aon: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut cursor = cursor::Cursor::new(aon);

    while !cursor.is_eof() {
        let token = tokenize_next(&mut cursor)?;
        tokens.push(token);
    }

    Ok(tokens)
}

fn tokenize_next(cursor: &mut cursor::Cursor) -> Result<Token, String> {
    return match cursor.first() {
        tokens::SLASH => {
            cursor.bump();

            match cursor.first() {
                tokens::SLASH => {
                    cursor.bump();
                    let mut comment = String::new();
                    cursor.eat_while(|c| {
                        if c == '\n' {
                            false
                        } else {
                            comment.push(c);
                            true
                        }
                    });

                    Ok(Token::Comment(comment))
                }
                _ => return Err(format!("Unexpected character: {}", cursor.first())),
            }
        }

        tokens::SINGLE_QUOTE => {
            cursor.bump();
            let mut string = String::new();
            cursor.eat_while(|c| {
                if c == tokens::SINGLE_QUOTE {
                    false
                } else {
                    string.push(c);
                    true
                }
            });

            cursor.bump();

            Ok(Token::Word(string))
        }

        tokens::DOUBLE_QUOTE => {
            cursor.bump();
            let mut string = String::new();
            cursor.eat_while(|c| {
                if c == tokens::DOUBLE_QUOTE {
                    false
                } else {
                    string.push(c);
                    true
                }
            });

            cursor.bump();

            Ok(Token::Word(string))
        }

        tokens::COLON => {
            cursor.bump();
            Ok(Token::Colon)
        }

        tokens::COMMA => {
            cursor.bump();
            Ok(Token::Comma)
        }

        tokens::HASH => {
            cursor.bump();
            Ok(Token::Hash)
        }

        tokens::LEFT_BRACE => {
            cursor.bump();
            Ok(Token::LeftBrace)
        }

        tokens::RIGHT_BRACE => {
            cursor.bump();
            Ok(Token::RightBrace)
        }

        tokens::LEFT_BRACKET => {
            cursor.bump();
            Ok(Token::LeftBracket)
        }

        tokens::RIGHT_BRACKET => {
            cursor.bump();
            Ok(Token::RightBracket)
        }
        
        other if other.is_whitespace() => {
            cursor.bump();
            return tokenize_next(cursor);
        }

        other if other.is_numeric() => {
            let mut number = String::new();
            cursor.eat_while(|c| {
                if c.is_numeric() || c == '.' {
                    number.push(c);
                    true
                } else {
                    false
                }
            });

            match Number::new(&number) {
                Some(n) => Ok(Token::Number(n)),
                None => Err(format!("Invalid number: {}", number)),
            }
        }

        other => Err("Unexpected character: ".to_owned() + &other.to_string()),
    };
}
