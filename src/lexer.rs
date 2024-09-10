use crate::token::LexicalError;
use crate::token::Numeric;
use crate::token::{Token, TokenType};
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
    pub(crate)  fn tokens(&mut self) -> Vec<Token> {
        self.into_iter().collect()
    }

    fn match_or_skip(&mut self) -> Option<Token> {
        let p = &mut self.iter;
        let next = p.peek();
        let token = |ch: &char| Some(Token::new(TokenType::Slash, self.line, ch.to_string()));
        match next {
            Some(w) if *w != '/' => token(w),
            None => token(&'/'),
            _ => loop {
                match p.next() {
                    None => {
                        self.at_end = true;
                        break Some(Token::new(TokenType::Eof, self.line, "".to_string()));
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
        let unknown = |s: String, l: LineNum| {
            Token::new(TokenType::Unknown(LexicalError::UnterminatedString), l, s)
        };

        loop {
            match p.next() {
                Some('\"') => break Some(Token::new(TokenType::StringLiteral, self.line, literal)),
                Some('\n') => {
                    self.line += 1;
                    //break Some(unknown(literal, self.line));
                    literal.push('\n')
                }
                None => {
                    break Some(unknown(literal, self.line));
                }
                Some(c) => literal.push(c),
            }
        }
    }
    fn try_parse(&self, val_str: &str) -> Option<Token> {
        if let Ok(val) = val_str.parse::<Numeric>() {
            Some(Token::new(
                TokenType::Number(val),
                self.line,
                val_str.to_string(),
            ))
        } else {
            Some(Token::new(
                TokenType::Unknown(LexicalError::InvalidNumber),
                self.line,
                val_str.to_string(),
            ))
        }
    }

    fn parse_number(&mut self, first: char) -> Option<Token> {
        let mut val_str = String::from(first);
        let p = &mut self.iter;
        let mut curr = p.peek();
        loop {
            match curr {
                Some(c) if c.is_digit(10) => val_str.push(*c),
                Some('.') => {
                    if val_str.contains('.') {
                        break Some(Token::new(
                            TokenType::Unknown(LexicalError::InvalidNumber),
                            self.line,
                            val_str,
                        ));
                    } else {
                        val_str.push('.');
                    }
                }
                Some(c) if c.is_whitespace() => {
                    break self.try_parse(&val_str);
                }
                _ => break self.try_parse(&val_str),
            }
            p.next();
            curr = p.peek();
        }
    }
    /// Returns Some(c) where c is a token representing a reserved word
    /// or None if s is not a reserved word
    fn reserved_from_str(&self, s: &str) -> Option<Token> {
        let tokentype = match s {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        };
        tokentype.map(|tt| Token::new(tt, self.line, s.to_string()))
    }

    fn parse_ident(&mut self, first: char) -> Option<Token> {
        let mut val_str = String::from(first);
        let p = &mut self.iter;
        let mut curr = p.peek();
        loop {
            match curr {
                Some(c) if c.is_ascii_alphanumeric() || *c == '_' => val_str.push(*c),
                _ => {
                    let reserved_or_ident = if let Some(t) = self.reserved_from_str(&val_str) {
                        t
                    } else {
                        Token::new(TokenType::Identifier, self.line, val_str)
                    };
                    break Some(reserved_or_ident);
                }
            }
            p.next();
            curr = p.peek();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let p = &mut self.iter;
        if let Some(c) = p.next() {
            match c {
                '(' => Some(Token::of_char(TokenType::LeftParen, self.line, c)),
                ')' => Some(Token::of_char(TokenType::RightParen, self.line, c)),
                '{' => Some(Token::of_char(TokenType::LeftBrace, self.line, c)),
                '}' => Some(Token::of_char(TokenType::RightBrace, self.line, c)),
                '*' => Some(Token::of_char(TokenType::Star, self.line, c)),
                '+' => Some(Token::of_char(TokenType::Plus, self.line, c)),

                '-' => Some(Token::of_char(TokenType::Minus, self.line, c)),
                '.' => Some(Token::of_char(TokenType::Dot, self.line, c)),
                ',' => Some(Token::of_char(TokenType::Comma, self.line, c)),
                ';' => Some(Token::of_char(TokenType::Semicolon, self.line, c)),
                '=' => self.match_next(
                    '=',
                    Token::new(TokenType::EqualEqual, self.line, "==".into()),
                    Token::new(TokenType::Equal, self.line, "=".into()),
                ),
                '>' => self.match_next(
                    '=',
                    Token::new(TokenType::GreaterEqual, self.line, ">=".into()),
                    Token::new(TokenType::Greater, self.line, ">".into()),
                ),
                '<' => self.match_next(
                    '=',
                    Token::new(TokenType::LessEqual, self.line, "<=".into()),
                    Token::new(TokenType::Less, self.line, "<".into()),
                ),
                '!' => self.match_next(
                    '=',
                    Token::new(TokenType::BangEqual, self.line, "!=".into()),
                    Token::new(TokenType::Bang, self.line, "!".into()),
                ),
                '/' => self.match_or_skip(),
                '\"' => self.parse_string(),
                '\n' => {
                    self.line += 1;
                    self.next()
                }
                sp if sp.is_ascii_whitespace() => self.next(),
                d if d.is_digit(10) || d == '.' => self.parse_number(d),
                a if a.is_ascii_alphabetic() || a == '_' => self.parse_ident(a),
                unknown => Some(Token::new(
                    TokenType::Unknown(LexicalError::UnknownToken(unknown)),
                    self.line,
                    c.to_string(),
                )),
            }
        } else {
            if !self.at_end {
                self.at_end = true;
                Some(Token::new(TokenType::Eof, self.line, "".into()))
            } else {
                None
            }
        }
    }
}
