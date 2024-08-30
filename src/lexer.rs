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
    fn match_or_skip(&mut self) -> Option<Token> {
        let p = &mut self.iter;
        let next = p.peek();
        match next {
            Some(w) if *w != '/' => Some(Token::Slash),
            None => Some(Token::Slash),
            _ => {
                loop {
                    match p.next() {
                        None => break,
                        _ => continue,
                    }
                }
                self.at_end = true;
                Some(Token::Eof)
            }
        }
    }
    fn match_next(&mut self, c: char, matching: Token, other: Token) -> Option<Token> {
        let p = &mut self.iter;

        let next = p.peek();
        match next {
            Some(w) if *w == c => {
                p.next();
                Some(matching)
            }
            Some(_) => Some(other),
            None => Some(other),
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
                '=' => self.match_next('=', Token::EqualEqual, Token::Equal),
                '>' => self.match_next('=', Token::GreaterEqual, Token::Greater),
                '<' => self.match_next('=', Token::LessEqual, Token::Less),
                '!' => self.match_next('=', Token::BangEqual, Token::Bang),
                '/' => self.match_or_skip(),

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
