use std::fmt::Display;

///
/// Lex language token
pub(crate) enum Token {
    LeftParen,
    RightParen,
    Unknown,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => f.write_str("LEFT_PAREN ( null"),
            Self::RightParen => f.write_str("RIGHT_PAREN ) null"),
            Self::Unknown => f.write_str("UNKNOWN_TOKEN"),
            Self::Eof => f.write_str("EOF  null"),
        }
    }
}
