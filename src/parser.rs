use crate::token;
use core::fmt::Display;
use token::Token;
pub(crate) struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

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
                    expr = Expression::Binary(
                        Box::new(expr),
                        POperator::binary_from(&curr_token),
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
                    expr = Expression::Binary(
                        Box::new(expr),
                        POperator::binary_from(&curr_token),
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
                    expr = Expression::Binary(
                        Box::new(expr),
                        POperator::binary_from(&curr_token),
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
                    expr = Expression::Binary(
                        Box::new(expr),
                        POperator::binary_from(&curr_token),
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
                Expression::Unary(POperator::unary_from(&curr_token), Box::new(self.unary()))
            }
            _ => self.primary(),
        };
        self.advance();
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
                    _ => Expression::Invalid,
                }
            }
            _ => Expression::Invalid,
        };
        self.advance();
        prim
    }
}
#[derive(Debug, Clone)]
pub(crate) enum POperator {
    Bin(Binary),
    Uni(Unary),
}

impl POperator {
    fn binary_from(t: &Token) -> POperator {
        match t {
            Token::Plus => POperator::Bin(Binary::Plus),
            Token::Minus => POperator::Bin(Binary::Minus),
            Token::Slash => POperator::Bin(Binary::Divide),
            Token::Star => POperator::Bin(Binary::Multiply),
            _ => POperator::Bin(Binary::Invalid(t.clone())),
        }
    }

    fn unary_from(t: &Token) -> POperator {
        match t {
            Token::Minus => POperator::Uni(Unary::Minus),
            Token::Bang => POperator::Uni(Unary::Not),
            _ => POperator::Uni(Unary::Invalid(t.clone())),
        }
    }
}

impl Display for POperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bin(b) => f.write_fmt(format_args!("{}", b)),
            Self::Uni(u) => f.write_fmt(format_args!("{}", u)),
        }
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

#[derive(Debug)]
pub(crate) enum Expression {
    Primary(Token),
    Binary(Box<Expression>, POperator, Box<Expression>),
    Unary(POperator, Box<Expression>),
    Paren(Box<Expression>),
    Invalid,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary(t) => match t {
                Token::True => f.write_str("true"),
                Token::False => f.write_str("false"),
                Token::Nil => f.write_str("nil"),
                other => f.write_str(&other.to_string()),
            },
            Self::Binary(l, o, r) => f.write_fmt(format_args!("({} {} {})", o, l, r)),
            Self::Unary(o, e) => f.write_fmt(format_args!("({} {})", o, e)),
            Self::Paren(e) => f.write_fmt(format_args!("({})", e)),
            Self::Invalid => f.write_str("Parse error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_true() {
        let mut p = Parser::new(vec![Token::True]);
        assert_eq!(p.parse().to_string(), "true");
    }
    #[test]
    fn parses_false() {
        let mut p = Parser::new(vec![Token::False]);
        assert_eq!(p.parse().to_string(), "false");
    }
    #[test]
    fn parses_nil() {
        let mut p = Parser::new(vec![Token::Nil]);
        assert_eq!(p.parse().to_string(), "nil");
    }
}
