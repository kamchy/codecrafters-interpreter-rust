use std::char;

use crate::token::Token;
pub(crate) struct Lexer<'a> {
    // s: &'a str,
    iter: Box<dyn Iterator<Item = char> + 'a>,
    at_end: bool,
}
impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Lexer {
            // s,
            iter: Box::new(s.chars()),
            at_end: false,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let chariter = &mut self.iter;
        let mut p = chariter.peekable();
        if let Some(c) = p.next() {
            match c {
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                _ => Some(Token::Unknown),
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
