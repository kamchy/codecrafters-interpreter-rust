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
            Self::UnknownToken(c) => f.write_fmt(format_args!("Unexpected character: {}", c)),

            Self::UnterminatedString => f.write_str("Unterminated string."),
            Self::InvalidNumber => f.write_str("Invalid number."),
        }
    }
}

/// newtype for f64 to be used in Token::Number
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Numeric(pub f64);
impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
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
pub(crate) enum TokenType {
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
    Unknown(LexicalError),
    StringLiteral,
    Number(Numeric),
    Identifier,
    Slash,
    Eof,
    /* Reserved words */
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Display for TokenType {
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
            Self::StringLiteral => f.write_str("STRING"),
            Self::Identifier => f.write_str("IDENTIFIER"),
            Self::Number(v) => f.write_fmt(format_args!("{}", v)),
            Self::Unknown(lex_err) => f.write_fmt(format_args!("{}", lex_err)),
            Self::Eof => f.write_str("EOF  null"),
            Self::And => f.write_str("AND and null"),
            Self::Class => f.write_str("CLASS class null"),
            Self::Else => f.write_str("ELSE else null"),
            Self::False => f.write_str("FALSE false null"),
            Self::For => f.write_str("FOR for null"),
            Self::Fun => f.write_str("FUN fun null"),
            Self::If => f.write_str("IF if null"),
            Self::Nil => f.write_str("NIL nil null"),
            Self::Or => f.write_str("OR or null"),
            Self::Print => f.write_str("PRINT print null"),
            Self::Return => f.write_str("RETURN return null"),
            Self::Super => f.write_str("SUPER super null"),
            Self::This => f.write_str("THIS this null"),
            Self::True => f.write_str("TRUE true null"),
            Self::Var => f.write_str("VAR var null"),
            Self::While => f.write_str("WHILE while null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    /// Type of token
    pub typ: TokenType,
    /// line number where the token was seen
    pub ln: LineNum,
    /// parsed input fragment
    pub s: String,
}
impl Token {
    pub fn new(typ: TokenType, ln: LineNum, s: String) -> Token {
        Token { typ, ln, s }
    }
    pub(crate) fn of_char(typ: TokenType, ln: LineNum, c: char) -> Token {
        Token {
            typ,
            ln,
            s: c.to_string(),
        }
    }
    pub(crate) fn of_bool(b: bool, ln: LineNum) -> Token {
        let tt = if b { TokenType::True } else { TokenType::False };
        Token {
            typ: tt.clone(),
            ln,
            s: tt.to_string(),
        }
    }

    pub(crate) fn nil(ln: LineNum) -> Token {
        Token {
            typ: TokenType::Nil,
            ln,
            s: "nil".to_string(),
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.typ {
            TokenType::StringLiteral => f.write_fmt(format_args!("STRING \"{0}\" {0}", self.s)),
            TokenType::Number(v) => f.write_fmt(format_args!("NUMBER {} {}", self.s, v)),
            TokenType::Identifier => f.write_fmt(format_args!("IDENTIFIER {} null", self.s)),
            TokenType::Unknown(err) => {
                f.write_fmt(format_args!("[line {}] Error: {}", self.ln, err))
            }
            other => f.write_fmt(format_args!("{}", other)),
        }
    }
}
