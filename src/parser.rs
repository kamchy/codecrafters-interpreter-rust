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

///Declaration can be variable declaration or a statement
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Decl {
    VarDecl(Token, Option<Expression>),
    Statement(Stmt),
}
impl Decl {
    fn is_valid(&self) -> bool {
        match self {
            Decl::VarDecl(_token, Some(e)) => e.is_valid(),
            Decl::VarDecl(_token, None) => true,
            Decl::Statement(stmt) => stmt.is_valid(),
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decl::Statement(s) => write!(f, "{}", s),
            Decl::VarDecl(t, opt_e) => match opt_e {
                Some(e) => write!(f, "var {} = {}", t, e),
                None => write!(f, "var {};", t),
            },
        }
    }
}

/// Statement can be either a print statement or expression statement
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Stmt {
    Print(Expression),
    Expression(Expression),
    Block(Vec<Decl>),
    Invalid(String),
}

impl Stmt {
    fn is_valid(&self) -> bool {
        match self {
            Stmt::Print(e) => e.is_valid(),
            Stmt::Expression(e) => e.is_valid(),
            Stmt::Block(v) => v.iter().all(|e| e.is_valid()),
            Stmt::Invalid(_) => false,
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Print(e) => f.write_fmt(format_args!("{}", e)),
            Stmt::Expression(e) => f.write_fmt(format_args!("{}", e)),
            Self::Block(v) => f.write_fmt(format_args!("{:?}", v)),
            Self::Invalid(s) => f.write_str(s),
        }
    }
}

/// Prorgam is a vector of statements
#[derive(Debug)]
pub(crate) struct Program {
    pub declarations: Vec<Decl>,
}

impl Program {
    /// returns optional first syntax error
    pub(crate) fn syntax_errors(&self) -> Option<Decl> {
        self.declarations
            .iter()
            .filter(|s| !s.is_valid())
            .take(1)
            .next()
            .map(Decl::clone)
    }
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            self.declarations
                .iter()
                .map(|s| s.to_string())
                .collect::<String>()
        ))
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
            res.push(self.declaration());
            is_end = self.at_end();
        }
        Program { declarations: res }
    }

    fn declaration(&mut self) -> Decl {
        let c = self.current();
        match c.typ {
            TokenType::Var => self.var_declaration(),
            _ => Decl::Statement(self.statement()),
        }
    }

    fn statement(&mut self) -> Stmt {
        let c = self.current();

        match c.typ {
            TokenType::Print => self.print_statement(),
            TokenType::LeftBrace => self.block(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Stmt {
        self.advance();
        let s = Stmt::Print(self.expression());
        if self.current().typ == TokenType::Semicolon {
            self.advance();
        };
        s
    }

    fn block(&mut self) -> Stmt {
        self.advance();
        let mut statements: Vec<Decl> = Vec::new();
        while (self.current().typ != TokenType::RightBrace) && (!self.at_end()) {
            let d = self.declaration();
            statements.push(d);
        }
        if self.current().typ == TokenType::RightBrace {
            self.advance();
            Stmt::Block(statements)
        } else {
            Stmt::Invalid(format!("[line {}] Error at end: Expect '}}'", self.current().ln))
        }
    }

    fn expression_statement(&mut self) -> Stmt {
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
        self.assignment()
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
        match curr_token.typ {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                Expression::UnaryEx(Unary::new(&curr_token), Box::new(self.unary()))
            }
            _ => self.primary(),
        }
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
            TokenType::Identifier => Expression::Variable(curr),
            _other => Expression::Invalid(
                format!("[line {}] Error at {}: Expected primary (number,  string, bool, nil)  or left paren", curr.ln, curr.s),
            ),
        };
        self.advance();
        prim
    }

    fn var_declaration(&mut self) -> Decl {
        self.advance();
        if self.current().typ != TokenType::Identifier {
            Decl::VarDecl(
                self.current(),
                Some(Expression::Invalid("Expect variable name.".to_string())),
            )
        } else {
            let ident_token = self.current().clone();
            self.advance();
            let initializer = if self.current().typ == TokenType::Equal {
                self.advance();

                Some(self.expression())
            } else {
                None
            };

            if self.current().typ != TokenType::Semicolon {
                Decl::VarDecl(
                    self.current(),
                    Some(Expression::Invalid(
                        "Expect ';' after variable declaration.".to_string(),
                    )),
                )
            } else {
                self.advance();
                Decl::VarDecl(ident_token, initializer)
            }
        }
    }

    fn assignment(&mut self) -> Expression {
        let expr = self.equality();
        //let equals: Token = self.current().clone();
        // see this trick here: https://craftinginterpreters.com/statements-and-state.html#assignment
        if self.current().typ == TokenType::Equal {
            self.advance();
            let value = self.assignment();
            match expr {
                Expression::Variable(tok) => Expression::Assign(tok.clone(), Box::new(value)),
                _ => Expression::Invalid("Invalid assignment target.".to_string()),
            }
        } else {
            expr
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    Invalid(Token),
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
            Binary::Invalid(t) => format!("[invalid binary operator: {}]", t),
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
            _ => Binary::Invalid(t.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Primary(Token),
    BinaryEx(Box<Expression>, Binary, Box<Expression>),
    UnaryEx(Unary, Box<Expression>),
    Paren(Box<Expression>),
    Variable(Token),
    Assign(Token, Box<Expression>),
    Invalid(String),
}
impl Expression {
    fn is_valid(&self) -> bool {
        !matches!(self, Self::Invalid(_))
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
                TokenType::StringLiteral => f.write_str(&t.s),
                other => f.write_str(&other.to_string()),
            },
            Self::BinaryEx(l, o, r) => f.write_fmt(format_args!("({} {} {})", o, l, r)),
            Self::UnaryEx(o, e) => f.write_fmt(format_args!("({} {})", o, e)),
            Self::Paren(e) => f.write_fmt(format_args!("(group {})", e)),
            Self::Variable(e) => f.write_fmt(format_args!("(var {})", e)),
            Self::Assign(t, e) => write!(f, "({} = {})", t, e),
            Self::Invalid(s) => f.write_fmt(format_args!("Parse error: {}", s)),
        }
    }
}
