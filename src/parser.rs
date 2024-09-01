use crate::token;
use core::fmt::Display;
use token::Token;
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
            match curr_token {
                Token::BangEqual | Token::EqualEqual => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.comparison()),
                    );
                }
                _ => break,
            }
            self.advance();
        }

        expr
    }

    fn comparison(&mut self) -> crate::parser::Expression {
        let mut expr = self.term();
        loop {
            let curr_token = self.current();
            match curr_token {
                Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.term()),
                    );
                }
                _ => break,
            }
            self.advance();
        }
        expr
    }

    fn term(&mut self) -> crate::parser::Expression {
        let mut expr = self.factor();
        loop {
            let curr_token = self.current();
            match curr_token {
                Token::Minus | Token::Plus => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.factor()),
                    );
                }
                _ => break,
            }
            self.advance();
        }
        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();
        loop {
            let curr_token = self.current();
            match curr_token {
                Token::Slash | Token::Star => {
                    self.advance();
                    expr = Expression::BinaryEx(
                        Box::new(expr),
                        Binary::new(&curr_token),
                        Box::new(self.unary()),
                    )
                }
                _ => break,
            }
            self.advance();
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        let curr_token = self.current();
        let expr = match curr_token {
            Token::Bang | Token::Minus => {
                self.advance();
                Expression::UnaryEx(Unary::new(&curr_token), Box::new(self.unary()))
            }
            _ => self.primary(),
        };
        expr
    }

    fn primary(&mut self) -> Expression {
        let curr = self.current();
        let prim = match curr {
            Token::Number(_, _)
            | Token::StringLiteral(_)
            | Token::True
            | Token::False
            | Token::Nil => Expression::Primary(curr),
            Token::LeftParen => {
                self.advance();
                let e = self.expression();
                match self.current() {
                    Token::RightParen => Expression::Paren(Box::new(e)),
                    other => Expression::Invalid(
                        format!("Expected right paren, found {}", other).to_owned(),
                    ),
                }
            }
            _ => Expression::Invalid(
                "Expected primary (number,  string, bool, nil)  or left paren".into(),
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
    Invalid(Token),
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Binary::Plus => "+".to_owned(),
            Binary::Minus => "-".to_owned(),
            Binary::Divide => "/".to_owned(),
            Binary::Multiply => "*".to_owned(),
            Binary::Invalid(t) => format!("[invalid binary operator: {}]", t),
        };

        f.write_fmt(format_args!("{}", val))
    }
}
impl Binary {
    fn new(t: &Token) -> Self {
        match t {
            Token::Plus => Binary::Plus,
            Token::Minus => Binary::Minus,
            Token::Slash => Binary::Divide,
            Token::Star => Binary::Multiply,
            _ => Binary::Invalid(t.clone()),
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
        match t {
            Token::Minus => Unary::Minus,
            Token::Bang => Unary::Not,
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
            Self::Primary(t) => match t {
                Token::True => f.write_str("true"),
                Token::False => f.write_str("false"),
                Token::Nil => f.write_str("nil"),
                Token::Number(_, v) => f.write_str(&v.to_string()),
                Token::StringLiteral(s) => f.write_str(s),
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
        token::{Numeric, Token},
    };

    #[test]
    fn parses_true() {
        let mut p = Parser::new(vec![Token::True, Token::Eof]);
        assert_eq!(p.parse().to_string(), "true");
    }

    #[test]
    fn parses_true2() {
        assert_parsed_text_result("true", "true");
    }

    #[test]
    fn parses_false() {
        let mut p = Parser::new(vec![Token::False]);
        assert_eq!(p.parse().to_string(), "false");
    }

    #[test]
    fn parses_false2() {
        assert_parsed_text_result("false", "false");
    }

    #[test]
    fn parses_nil() {
        let mut p = Parser::new(vec![Token::Nil]);
        assert_eq!(p.parse().to_string(), "nil");
    }

    #[test]
    fn parses_nil2() {
        assert_parsed_text_result("nil", "nil");
    }

    #[test]
    fn parses_numeric() {
        let mut p = Parser::new(vec![Token::Number("43.47".to_string(), Numeric(43.47f64))]);
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
    fn parses_literal() {
        let mut p = Parser::new(vec![Token::StringLiteral("foo".to_string())]);
        assert_eq!(p.parse().to_string(), "foo");
    }

    #[test]
    fn parses_literal_2() {
        assert_parsed_text_result("\"foo\"", "foo");
    }

    #[test]
    fn parses_paren_string() {
        let mut p = Parser::new(vec![
            Token::LeftParen,
            Token::StringLiteral("foo".to_string()),
            Token::RightParen,
            Token::Eof,
        ]);
        assert_eq!(p.parse().to_string(), "(group foo)");
    }

    #[test]
    fn parse_paren_string_2() {
        assert_parsed_text_result("(\"foo\")", "(group foo)");
    }

    #[test]
    fn parses_unary_not_true() {
        let mut p = Parser::new(vec![Token::Bang, Token::True]);
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
        let mut p = Parser::new(vec![
            Token::Minus,
            Token::Number("10".to_string(), Numeric(10.0f64)),
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
}
