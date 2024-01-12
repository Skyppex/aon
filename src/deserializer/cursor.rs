use std::str::Chars;

use crate::AonError;

pub(crate) struct Cursor<'a> {
    pub aon: Chars<'a>,
}

pub(crate) const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn new(aon: &'a str) -> Cursor<'a> {
        Cursor {
            aon: aon.chars(),
        }
    }

    pub(crate) fn first(&mut self) -> char {
        self.aon.clone().next().unwrap_or(EOF)
    }

    pub(crate) fn second(&mut self) -> char {
        let mut clone = self.aon.clone();
        clone.next();
        clone.next().unwrap_or(EOF)
    }

    pub(crate) fn is_eof(&mut self) -> bool {
        self.aon.as_str().is_empty()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        self.aon.next()
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub(crate) fn eat_text(&mut self, expected_text: &str) -> Result<(), AonError> {
        for c in expected_text.chars() {
            if self.first() != c {
                return Err(AonError::UnexpectedCharacter(self.first()))
            }
        }

        Ok(())
    }
}
