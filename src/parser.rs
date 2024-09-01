use crate::token;
use core::fmt::Display;
use token::{Token, TokenType};

/// Parser for lox.
/// Initialized with a vector of tokens.
/// Has curr - index of not yer consumed token  in tokens vec.
pub(crate) struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}
/// See https://craftinginterpreters.com/parsing-expressions.html#recursive-descent-parsing
impl Parser {
    pub(crate) fn parse(&mut self) -> Expression {
        self.expression()
    }

    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, curr: 0 }
    }

    fn advance(&mut self) {
        if self.curr + 1 < self.tokens.len() {
            self.curr += 1;
        }
    }

    fn current(&self) -> token::Token {
        self.tokens.get(self.curr).unwrap().clone()
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> crate::parser::Expression {
        let mut expr = self.comparison();
        loop {
            let curr_token = self.current();
            match curr_token.typ {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.comparison()),
                    );
                }
                _ => break,
            }
        }

        expr
    }

    fn comparison(&mut self) -> crate::parser::Expression {
        let mut expr = self.term();
        loop {
            let curr_token = self.current();
            match curr_token.typ {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.term()),
                    );
                }
                _ => break,
            }
        }
        expr
    }

    fn term(&mut self) -> crate::parser::Expression {
        let mut factor = self.factor();
        loop {
            let curr_token = self.current();
            match curr_token.typ {
                TokenType::Minus | TokenType::Plus => {
                    self.advance();
                    factor = Expression::BinaryEx(
                        Box::new(factor),
                        Binary::new(&curr_token),
                        Box::new(self.factor()),
                    );
                }
                _ => break,
            }
        }
        factor
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();
        loop {
            let curr_token = self.current();
            match curr_token.typ {
                TokenType::Slash | TokenType::Star => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.unary()),
                    )
                }
                _ => break,
            }
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        let curr_token = self.current();
        let expr = match curr_token.typ {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                Expression::UnaryEx(Unary::new(&curr_token), Box::new(self.unary()))
            }
            _ => self.primary(),
        };
        expr
    }

    fn primary(&mut self) -> Expression {
        let curr = self.current();
        let prim = match curr.typ {
            TokenType::Number(_)
            | TokenType::StringLiteral
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => Expression::Primary(curr),
            TokenType::LeftParen => {
                self.advance();
                let e = self.expression();
                match self.current().typ {
                    TokenType::RightParen => Expression::Paren(Box::new(e)),
                    _other => Expression::Invalid(
                        format!("[line {} Error at {}: Expected right paren", curr.ln, curr.s).to_owned(),
                    ),
                }
            }
            _other => Expression::Invalid(
                format!("[line {}] Error at {}: Expected primary (number,  string, bool, nil)  or left paren", curr.ln, curr.s),
            ),
        };
        self.advance();
        prim
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Binary {
    Plus,
    Minus,
    Divide,
    Multiply,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    NotEqual,
    InvalidBinary(Token),
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Binary::Plus => "+".to_owned(),
            Binary::Minus => "-".to_owned(),
            Binary::Divide => "/".to_owned(),
            Binary::Multiply => "*".to_owned(),
            Binary::Greater => ">".to_owned(),
            Binary::Less => "<".to_owned(),
            Binary::GreaterEqual => ">=".to_owned(),
            Binary::LessEqual => "<=".to_owned(),
            Binary::EqualEqual => "==".to_owned(),
            Binary::NotEqual => "!=".to_owned(),
            Binary::InvalidBinary(t) => format!("[invalid binary operator: {}]", t),
        };

        f.write_fmt(format_args!("{}", val))
    }
}
impl Binary {
    fn new(t: &Token) -> Self {
        match t.typ {
            TokenType::Plus => Binary::Plus,
            TokenType::Minus => Binary::Minus,
            TokenType::Slash => Binary::Divide,
            TokenType::Star => Binary::Multiply,
            TokenType::Less => Binary::Less,
            TokenType::Greater => Binary::Greater,
            TokenType::LessEqual => Binary::LessEqual,
            TokenType::GreaterEqual => Binary::GreaterEqual,
            TokenType::EqualEqual => Binary::EqualEqual,
            TokenType::BangEqual => Binary::NotEqual,
            _ => Binary::InvalidBinary(t.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Unary {
    Minus,
    Not,
    Invalid(Token),
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Unary::Not => "!".to_owned(),
            Unary::Minus => "-".to_owned(),
            Unary::Invalid(t) => format!("[invalid unary operator: {}]", t),
        };
        f.write_fmt(format_args!("{}", val))
    }
}

impl Unary {
    fn new(t: &Token) -> Self {
        match t.typ {
            TokenType::Minus => Unary::Minus,
            TokenType::Bang => Unary::Not,
            _ => Unary::Invalid(t.clone()),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Expression {
    Primary(Token),
    BinaryEx(Box<Expression>, Binary, Box<Expression>),
    UnaryEx(Unary, Box<Expression>),
    Paren(Box<Expression>),
    Invalid(String),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary(t) => match &t.typ {
                TokenType::True => f.write_str("true"),
                TokenType::False => f.write_str("false"),
                TokenType::Nil => f.write_str("nil"),
                TokenType::Number(v) => f.write_str(&v.to_string()),
                TokenType::StringLiteral => f.write_str(&t.s), //?
                other => f.write_str(&other.to_string()),
            },
            Self::BinaryEx(l, o, r) => f.write_fmt(format_args!("({} {} {})", o, l, r)),
            Self::UnaryEx(o, e) => f.write_fmt(format_args!("({} {})", o, e)),
            Self::Paren(e) => f.write_fmt(format_args!("(group {})", e)),
            Self::Invalid(s) => f.write_fmt(format_args!("Parse error: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexer,
        token::{Numeric, TokenType},
    };

    impl Parser {
        fn new_from_tokentype_vec(v: Vec<TokenType>) -> Parser {
            Parser::new(
                v.into_iter()
                    .map(|tt| Token::new(tt, 0, "".into()))
                    .collect(),
            )
        }
    }
    #[test]
    fn parses_true() {
        let mut p = Parser::new_from_tokentype_vec(vec![TokenType::True, TokenType::Eof]);
        assert_eq!(p.parse().to_string(), "true");
    }

    #[test]
    fn parses_true2() {
        assert_parsed_text_result("true", "true");
    }

    #[test]
    fn parses_false() {
        let mut p = Parser::new_from_tokentype_vec(vec![TokenType::False]);
        assert_eq!(p.parse().to_string(), "false");
    }

    #[test]
    fn parses_false2() {
        assert_parsed_text_result("false", "false");
    }

    #[test]
    fn parses_nil() {
        let mut p = Parser::new_from_tokentype_vec(vec![TokenType::Nil]);
        assert_eq!(p.parse().to_string(), "nil");
    }

    #[test]
    fn parses_nil2() {
        assert_parsed_text_result("nil", "nil");
    }

    #[test]
    fn parses_numeric() {
        let mut p = Parser::new_from_tokentype_vec(vec![TokenType::Number(Numeric(43.47f64))]);
        assert_eq!(p.parse().to_string(), "43.47");
    }

    #[test]
    fn parses_numeric2() {
        assert_parsed_text_result("43.47", "43.47");
    }

    #[test]
    fn parses_numeric3() {
        assert_parsed_text_result("43", "43.0");
    }

    #[test]
    fn parses_literal_2() {
        assert_parsed_text_result("\"foo\"", "foo");
    }

    #[test]
    fn parse_paren_string_2() {
        assert_parsed_text_result("(\"foo\")", "(group foo)");
    }

    #[test]
    fn parses_unary_not_true() {
        let mut p = Parser::new_from_tokentype_vec(vec![TokenType::Bang, TokenType::True]);
        assert_eq!(p.parse().to_string(), "(! true)");
    }

    #[test]
    fn parses_unary_not_true_2() {
        assert_parsed_text_result("!true", "(! true)")
    }

    #[test]
    fn parses_unary_not_false() {
        assert_parsed_text_result("! false", "(! false)")
    }

    #[test]
    fn parses_unary_minus() {
        let mut p = Parser::new_from_tokentype_vec(vec![
            TokenType::Minus,
            TokenType::Number(Numeric(10.0f64)),
        ]);
        assert_eq!(p.parse().to_string(), "(- 10.0)");
    }

    #[test]
    fn parses_unary_minus_2() {
        assert_parsed_text_result("- 3", "(- 3.0)")
    }

    #[test]
    fn parses_unary_minus_in_parens() {
        assert_parsed_text_result("( - 10)", "(group (- 10.0))")
    }

    fn assert_parsed_text_result(text: &str, expected: &str) {
        let s = text;
        let lex = lexer::Lexer::new(s);
        let ts: Vec<_> = lex.collect();

        eprintln!("Test:\ntext: {}\ntokens: {:?}\n ", text, ts);
        let mut p = Parser::new(ts);
        assert_eq!(p.parse().to_string(), expected);
    }

    #[test]
    fn parses_unary_multiple() {
        assert_parsed_text_result("(!!(true))", "(group (! (! (group true))))")
    }

    #[test]
    fn parses_binary() {
        assert_parsed_text_result("16 * 38 / 58", "(/ (* 16.0 38.0) 58.0)")
    }

    #[test]
    fn parses_binary_plus() {
        assert_parsed_text_result("16 + 38 * 58", "(+ 16.0 (* 38.0 58.0))")
    }

    #[test]
    fn parses_comparison_operator() {
        assert_parsed_text_result("83 < 99 < 115", "(< (< 83.0 99.0) 115.0)")
    }

    #[test]
    fn parses_equality_operator() {
        assert_parsed_text_result("\"baz\" == \"baz\"", "(== baz baz)")
    }

    #[test]
    fn parses_inequality_operator() {
        assert_parsed_text_result("\"baz\" != \"baz\"", "(!= baz baz)")
    }

    #[test]
    fn parses_equality_operator2() {
        assert_parsed_text_result(
            "! (\"baz\" == \"baz\") > 5",
            "(> (! (group (== baz baz))) 5.0)",
        )
    }
}
