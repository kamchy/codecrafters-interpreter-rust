use crate::token::Token;
use std::{iter::Peekable, str::Chars};
pub(crate) struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    at_end: bool,
}
impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Lexer {
            iter: s.chars().peekable(),
            at_end: false,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let p = &mut self.iter;
        if let Some(c) = p.next() {
            match c {
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                '*' => Some(Token::Star),
                '+' => Some(Token::Plus),

                '-' => Some(Token::Minus),
                '.' => Some(Token::Dot),
                ',' => Some(Token::Comma),
                ';' => Some(Token::Semicolon),
                '=' => {
                    let next = p.peek();
                    match next {
                        Some('=') => Some(Token::EqualEqual),
                        Some(_) => Some(Token::Equal),
                        None => Some(Token::Equal),
                    }
                }

                _ => Some(Token::Unknown(c)),
            }
        } else {
            if !self.at_end {
                self.at_end = true;
                Some(Token::Eof)
            } else {
                None
            }
        }
    }
}
