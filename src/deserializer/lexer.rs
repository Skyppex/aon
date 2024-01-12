use std::vec;

use crate::{
    representation::value::Number,
    error::{AonError, Result}
};

use super::{cursor, tokens::{Token, self}};

pub fn tokenize(aon: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut cursor = cursor::Cursor::new(aon);

    while !cursor.is_eof() {
        let token = tokenize_next(&mut cursor)?;
        tokens.push(token);
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}

fn tokenize_next(cursor: &mut cursor::Cursor) -> Result<Token> {
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
                _ => return Err(AonError::UnexpectedCharacter(cursor.first())),
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
                None => Err(AonError::InvalidNumber(number)),
            }
        }

        other if other == 'n' => {
            cursor.eat_text(tokens::NULL)?;
            Ok(Token::Null)
        }

        other if other == 't' => {
            cursor.eat_text(tokens::TRUE)?;
            Ok(Token::Bool(true))
        }

        other if other == 'f' => {
            cursor.eat_text(tokens::FALSE)?;
            Ok(Token::Bool(false))
        }

        other => Err(AonError::UnexpectedCharacter(other)),
    };
}
