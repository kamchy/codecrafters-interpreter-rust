use crate::token;
use core::fmt::Display;
use std::thread::current;
use token::{Token, TokenType};

/// Parser for lox.
/// Initialized with a vector of tokens.
/// Has curr - index of not yer consumed token  in tokens vec.
pub(crate) struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}
/// Statement can be either a print statement or expression statement
#[derive(Debug, Clone)]
pub(crate)  enum Stmt {
    Print(Expression),
    Expression(Expression)
}
impl Stmt {
    fn is_valid(&self)->bool {
        match self {
            Stmt::Print(e) => e.is_valid(),
            Stmt::Expression(e) => e.is_valid()
        }
    }

    fn to_string(&self) -> String {
        match self {
            Stmt::Print(e) => e.to_string(),
            Stmt::Expression(e) => e.to_string()
        }
    }
}
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Print(e) => f.write_fmt(format_args!("{}", e)),
            Stmt::Expression(e) => f.write_fmt(format_args!("{}", e))
        }
    }
}


/// Prorgam is a vector of statements
#[derive(Debug)]
pub(crate) struct Program {
    pub statements: Vec<Stmt>,
}


impl Program {
    fn new(v: Vec<Stmt>) -> Self {
        eprint!("Constructed Program with statements: \n{:?}", v);
        Program {statements: v }
    }
      /// returns optional first syntax error
    pub(crate) fn syntax_errors(&self) -> Option<Stmt> {
        self.statements.iter().filter(|s| !s.is_valid()).take(1).next().map(|s| s.to_owned())
    }
    pub(crate) fn to_string(&self) -> String {
        self.statements.iter().map(|s| s.to_string()).collect()
    }
}

/// See https://craftinginterpreters.com/parsing-expressions.html#recursive-descent-parsing
impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        //eprint!("new parser: tokens:{:?}", tokens);
        Parser { tokens, curr: 0 }
    }

    pub(crate) fn parse(&mut self) -> Program {
        let mut res = Vec::new();
        let mut is_end: bool = false;
        while !is_end {
                res.push(self.statement());
                is_end = self.at_end();

        }
        Program { statements: res }
    }

    fn statement(&mut self) -> Stmt {
        let c = self.current();
        let s = match c.typ{
            TokenType::Print => self.print_statement(),
            _ => self.expression_statement()
        };
        s
    }

    fn print_statement(&mut self) -> Stmt {
        self.advance();
        let s = Stmt::Print(self.expression());
        if self.current().typ == TokenType::Semicolon {
            self.advance();
        };
        s
    }

    fn expression_statement(&mut self) -> Stmt{
        let s = Stmt::Expression(self.expression());
        if self.current().typ == TokenType::Semicolon {
            self.advance();
        };
        s
    }

    fn at_end(&self) -> bool {
        self.curr == self.tokens.len() - 1
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

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Primary(Token),
    BinaryEx(Box<Expression>, Binary, Box<Expression>),
    UnaryEx(Unary, Box<Expression>),
    Paren(Box<Expression>),
    Invalid(String),
}
impl Expression {
    fn is_valid(&self) -> bool {
        match self {
            Self::Invalid(_) => false,
            _ => true
       }
    }
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
