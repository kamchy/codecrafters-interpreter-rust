use crate::lexer::LineNum;
use std::{fmt::Display, str::FromStr};
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum LexicalError {
    UnknownToken(char),
    UnterminatedString,
    InvalidNumber,
}
impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownToken(c) => {
                f.write_fmt(format_args!("Error: Unexpected character: {}", c))
            }

            Self::UnterminatedString => f.write_str("Error: Unterminated string."),
            Self::InvalidNumber => f.write_str("Error: Invalid number."),
        }
    }
}
// newtype for float
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Numeric(pub f64);
impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
impl FromStr for Numeric {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>()
            .map(|f| Numeric(f))
            .map_err(|e| e.to_string())
    }
}
/// Lex language token
#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Equal,
    EqualEqual,
    BangEqual,
    Bang,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
    Unknown(LineNum, LexicalError),
    StringLiteral(String),
    Number(String, Numeric),
    Slash,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => f.write_str("LEFT_PAREN ( null"),
            Self::RightParen => f.write_str("RIGHT_PAREN ) null"),
            Self::LeftBrace => f.write_str("LEFT_BRACE { null"),
            Self::RightBrace => f.write_str("RIGHT_BRACE } null"),
            Self::Star => f.write_str("STAR * null"),
            Self::Dot => f.write_str("DOT . null"),
            Self::Comma => f.write_str("COMMA , null"),
            Self::Plus => f.write_str("PLUS + null"),
            Self::Minus => f.write_str("MINUS - null"),
            Self::Semicolon => f.write_str("SEMICOLON ; null"),
            Self::Equal => f.write_str("EQUAL = null"),
            Self::EqualEqual => f.write_str("EQUAL_EQUAL == null"),
            Self::BangEqual => f.write_str("BANG_EQUAL != null"),
            Self::Bang => f.write_str("BANG ! null"),
            Self::LessEqual => f.write_str("LESS_EQUAL <= null"),
            Self::GreaterEqual => f.write_str("GREATER_EQUAL >= null"),
            Self::Less => f.write_str("LESS < null"),
            Self::Greater => f.write_str("GREATER > null"),
            Self::Slash => f.write_str("SLASH / null"),
            Self::StringLiteral(s) => f.write_fmt(format_args!("STRING \"{0}\" {0}", s.as_str())),
            Self::Number(s, v) => f.write_fmt(format_args!("NUMBER {} {}", s.as_str(), v)),
            Self::Unknown(_, lexerr) => f.write_fmt(format_args!("{}", lexerr)),
            Self::Eof => f.write_str("EOF  null"),
        }
    }
}
