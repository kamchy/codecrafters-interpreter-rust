use crate::token::LexicalError;
use crate::token::Token;
use std::{iter::Peekable, str::Chars};
pub type LineNum = u64;
pub(crate) struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    at_end: bool,
    line: LineNum,
}
impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Lexer {
            iter: s.chars().peekable(),
            at_end: false,
            line: 1,
        }
    }
    fn match_or_skip(&mut self) -> Option<Token> {
        let p = &mut self.iter;
        let next = p.peek();
        match next {
            Some(w) if *w != '/' => Some(Token::Slash),
            None => Some(Token::Slash),
            _ => loop {
                match p.next() {
                    None => {
                        self.at_end = true;
                        break Some(Token::Eof);
                    }
                    Some('\n') => {
                        self.line += 1;
                        break self.next();
                    }
                    _ => continue,
                }
            },
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

    fn parse_string(&mut self) -> Option<Token> {
        let mut literal = String::new();
        let p = &mut self.iter;

        loop {
            match p.next() {
                Some('\"') => break Some(Token::StringLiteral(literal)),
                Some('\n') => {
                    self.line += 1;
                    break Some(Token::Unknown(self.line, LexicalError::UnterminatedString));
                }
                None => {
                    break Some(Token::Unknown(self.line, LexicalError::UnterminatedString));
                }
                Some(c) => literal.push(c),
            }
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
                '\"' => self.parse_string(),
                '\n' => {
                    self.line += 1;
                    self.next()
                }
                sp if sp.is_ascii_whitespace() => self.next(),
                unknown => Some(Token::Unknown(
                    self.line,
                    LexicalError::UnknownToken(unknown),
                )),
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
