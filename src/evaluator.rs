use std::{error::Error, fmt::Display};

use crate::{
    environment::Environment,
    parser::{Binary, Decl, Expression, Program, Stmt, Unary},
    token::{Numeric, Token, TokenType},
};

pub type Result = std::result::Result<EvalResult, EvalError>;
pub type StatementResult = std::result::Result<StatementEvalResult, EvalError>;

/// Results of Stmt evaluation
#[derive(Debug, PartialEq)]
pub enum StatementEvalResult {
    ExpressionStatementResult(EvalResult),
    PrintStatementResult(EvalResult),
}
/// Result of expression evaluation
#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Numeric { value: f64, token: Token },
    Boolean { value: bool, token: Token },
    String { value: String, token: Token },
    Reserved { value: String, token: Token },
    Nil,
}

impl EvalResult {
    fn of_boolean(value: bool, token: &Token) -> EvalResult {
        Self::Boolean {
            value,
            token: token.clone(),
        }
    }

    fn of_reserved(arg: &str, token: &Token) -> EvalResult {
        Self::Reserved {
            value: arg.to_string(),
            token: token.clone(),
        }
    }

    fn of_string(arg: String, token: &Token) -> EvalResult {
        Self::String {
            value: arg,
            token: token.clone(),
        }
    }

    fn of_numeric(f: f64, token: &Token) -> EvalResult {
        Self::Numeric {
            value: f,
            token: token.clone(),
        }
    }
}
impl Clone for EvalResult {
    fn clone(&self) -> Self {
        match self {
            Self::Numeric { value, token } => Self::Numeric {
                value: *value,
                token: token.clone(),
            },
            Self::Boolean { value, token } => Self::Boolean {
                value: *value,
                token: token.clone(),
            },
            Self::String { value, token } => Self::String {
                value: value.clone(),
                token: token.clone(),
            },
            Self::Reserved { value, token } => Self::Reserved {
                value: value.clone(),
                token: token.clone(),
            },
            Self::Nil => Self::Nil,
        }
    }
}
impl Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Numeric { value: v, token: _ } => v.to_string(),
            Self::Boolean { value: b, token: _ } => b.to_string(),
            Self::String { value: s, token: _ } => s.to_string(),
            Self::Reserved { value: s, token: _ } => s.to_string(),
            Self::Nil => "nil".to_string(),
        };
        f.write_str(&s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EvalError {
    pub s: String,
}

impl EvalError {
    fn new(s: String) -> EvalError {
        EvalError { s }
    }
}

impl Error for EvalError {}
impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.s)
    }
}

// /// Creates Err variant from statuc string
// fn err(s: &'static str) -> Result {
//     Err(EvalError { s: s.into() })
// }

// /// Creates Ok variant from numeric value
// fn ok_num(n: f64) -> Result {
//     Ok(EvalResult::Numeric { field1: n , ltok))
// }

/// Evaluator of expressions
pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Environment::new(),
        }
    }

    fn eval_primary(&self, token: Token) -> Result {
        let t = &token;
        match token.typ {
            TokenType::True => Ok(EvalResult::of_boolean(true, t)),
            TokenType::False => Ok(EvalResult::of_boolean(false, t)),
            TokenType::Nil => Ok(EvalResult::of_reserved("nil", t)),
            TokenType::StringLiteral => Ok(EvalResult::of_string(t.s.clone(), t)),
            TokenType::Number(Numeric(f)) => Ok(EvalResult::of_numeric(f, t)),
            _ => Err(EvalError::new("unimplemented!".into())),
        }
    }

    fn eval_unary(&mut self, unary: Unary, ex: Expression) -> Result {
        let res = self.eval_expr(ex);
        match res {
            Ok(val) => match val {
                EvalResult::Nil => Ok(EvalResult::Nil),
                EvalResult::Numeric {
                    value: n,
                    token: ref tok,
                } => match unary {
                    Unary::Minus => Ok(EvalResult::of_numeric(-n, tok)),
                    Unary::Not => Ok(EvalResult::of_boolean(n == 0.0, tok)),
                    _ => {
                        runtime_error("Numeric arg can only be used with <minus> operator", tok.ln)
                    }
                },
                EvalResult::Reserved {
                    value: word,
                    token: ref tok,
                } => match unary {
                    Unary::Not => match word.to_lowercase().as_str() {
                        "nil" => Ok(EvalResult::of_boolean(true, tok)),
                        _ => runtime_error("Reselved word has no unary oper", tok.ln),
                    },
                    Unary::Minus => runtime_error("Operand must be a number.", tok.ln),
                    _ => runtime_error("Unary operator not supported", tok.ln),
                },
                EvalResult::Boolean {
                    value: v,
                    token: ref tok,
                } => match unary {
                    Unary::Not => Ok(EvalResult::of_boolean(!v, tok)),
                    Unary::Minus => runtime_error("Operand must be a number.", tok.ln),
                    _ => runtime_error("Bool arg can only be used with negation", tok.ln),
                },
                EvalResult::String {
                    value: _s,
                    token: tok,
                } => match unary {
                    Unary::Minus => runtime_error("Operand must be a number.", tok.ln),
                    _op => runtime_error("Operator cannot be used on string", tok.ln),
                },
            },
            Err(_) => res,
        }
    }

    fn eval_binary(&mut self, lex: Expression, op: Binary, rex: Expression) -> Result {
        let lr = self.eval_expr(lex);
        let rr = self.eval_expr(rex);

        calculate(lr?, op, rr?)
    }

    pub(crate) fn eval(&mut self, p: Program) -> Vec<StatementResult> {
        p.declarations.iter().map(|d| self.eval_decls(d)).collect()
    }

    fn eval_stmt(&mut self, s: Stmt) -> StatementResult {
        match s {
            Stmt::Print(e) => self
                .eval_expr(e)
                .map(StatementEvalResult::PrintStatementResult),
            Stmt::Expression(e) => self
                .eval_expr(e)
                .map(StatementEvalResult::ExpressionStatementResult),
        }
    }

    pub fn eval_expr(&mut self, e: Expression) -> Result {
        match e {
            Expression::Primary(t) => self.eval_primary(t),
            Expression::Paren(e) => self.eval_expr(*e),
            Expression::UnaryEx(unary, ex) => self.eval_unary(unary, *ex),
            Expression::BinaryEx(l, op, r) => self.eval_binary(*l, op, *r),
            Expression::Variable(t) => self.eval_variable(&t.s),
            Expression::Assign(t, e) =>self.eval_assign(&t, e),
            Expression::Invalid(s) => Err(EvalError::new(format!("Invalid expresstion: {}", s))),
        }
    }

    fn eval_variable(&self, s: &str) -> std::result::Result<EvalResult, EvalError> {
        self.env
            .get_var(s)
            .ok_or(EvalError::new(format!("Undefined variable '{}'.", s)))
    }

    fn eval_decls(
        &mut self,
        d: &crate::parser::Decl,
    ) -> std::result::Result<StatementEvalResult, EvalError> {
        match d {
            Decl::Statement(s) => self.eval_stmt(s.clone()),
            Decl::VarDecl(t, opt_e) => {
                let value = match opt_e {
                    None => Ok(EvalResult::Nil),
                    Some(e) => self.eval_expr(e.clone()),
                };
                value.map(|eval_res| {
                    StatementEvalResult::ExpressionStatementResult(
                        self.env.define(t.s.clone(), eval_res),
                    )
                })
            }
        }
    }

    fn eval_assign(&mut self, t: &Token, e: Box<Expression>) -> std::result::Result<EvalResult, EvalError> {
        match self.eval_expr(*e) {
            Ok(er) => self.env.assign(t, er),
            Err(e) => Err(e),
        }
    }
}

fn runtime_error(arg: &str, ln: u64) -> Result {
    Err(EvalError {
        s: format!("{}\n[Line {}]", arg, ln),
    })
}

fn calculate(lv: EvalResult, op: Binary, rv: EvalResult) -> Result {
    match lv {
        EvalResult::Numeric {
            value: l,
            token: ref ltok,
        } => match rv {
            EvalResult::Numeric {
                value: r,
                token: ref _rtok,
            } => match op {
                Binary::Plus => Ok(EvalResult::of_numeric(l + r, ltok)),
                Binary::Minus => Ok(EvalResult::of_numeric(l - r, ltok)),
                Binary::Divide => Ok(EvalResult::of_numeric(l / r, ltok)),
                Binary::Multiply => Ok(EvalResult::of_numeric(l * r, ltok)),
                Binary::Less => Ok(EvalResult::of_boolean(l < r, ltok)),
                Binary::LessEqual => Ok(EvalResult::of_boolean(l <= r, ltok)),
                Binary::Greater => Ok(EvalResult::of_boolean(l > r, ltok)),
                Binary::GreaterEqual => Ok(EvalResult::of_boolean(l >= r, ltok)),
                Binary::EqualEqual => Ok(EvalResult::of_boolean(l == r, ltok)),
                Binary::NotEqual => Ok(EvalResult::of_boolean(l != r, ltok)),
                Binary::Invalid(_) => runtime_error("Invalid binary operator", ltok.ln),
            },
            EvalResult::String {
                value: ref _s,
                token: ref _rtok,
            } => match op {
                Binary::EqualEqual => Ok(EvalResult::of_boolean(false, ltok)),
                Binary::NotEqual => Ok(EvalResult::of_boolean(false, ltok)),
                Binary::Plus => {
                    runtime_error("Operands must be two numbers or two strings.", ltok.ln)
                }
                _ => runtime_error("Only num != str and num == str supported", ltok.ln),
            },
            _ => runtime_error("Operands must be numbers.", ltok.ln),
        },
        EvalResult::String {
            value: ref l,
            token: ref ltok,
        } => match rv {
            EvalResult::String {
                value: ref r,
                token: ref _rtok,
            } => match op {
                Binary::Plus => Ok(EvalResult::of_string(l.to_owned() + r, ltok)),
                Binary::EqualEqual => Ok(EvalResult::of_boolean(l == r, ltok)),
                Binary::NotEqual => Ok(EvalResult::of_boolean(l != r, ltok)),
                _ => runtime_error("Only plus allowed on strings", ltok.ln),
            },
            EvalResult::Numeric {
                value: n,
                token: ref _rtok,
            } => match op {
                Binary::Multiply => Ok(EvalResult::of_string(l.repeat(n.round() as usize), ltok)),
                Binary::Plus => {
                    runtime_error("Operands must be two numbers or two strings.", ltok.ln)
                }
                _ => runtime_error("Only str*num and str+str allowed", ltok.ln),
            },
            _ => runtime_error("No other binary operations on strings", ltok.ln),
        },
        EvalResult::Boolean {
            value: lv,
            token: ltok,
        } => match rv {
            EvalResult::Boolean {
                value: rv,
                token: _rtok,
            } => match op {
                Binary::EqualEqual => Ok(EvalResult::Boolean {
                    value: lv == rv,
                    token: ltok,
                }),
                Binary::NotEqual => Ok(EvalResult::Boolean {
                    value: lv != rv,
                    token: ltok,
                }),
                _ => runtime_error("Bool operators allowed: only == and !=.", ltok.ln),
            },
            _ => match op {
                Binary::EqualEqual => Ok(EvalResult::Boolean {
                    value: false,
                    token: ltok,
                }),
                Binary::NotEqual => Ok(EvalResult::Boolean {
                    value: true,
                    token: ltok,
                }),
                Binary::Multiply => runtime_error("Operands must be numbers.", ltok.ln),
                _ => runtime_error("Operator not supported", ltok.ln),
            },
        },

        _ => runtime_error("Expected numeric arg", 1),
    }
}

#[cfg(test)]
mod test_evaluator {
    use core::panic;

    use crate::{
        evaluator::EvalResult,
        lexer::LineNum,
        token::{self, Numeric, Token, TokenType},
    };

    use super::Evaluator;

    impl Token {
        pub(crate) fn of_bool(b: bool, ln: LineNum) -> Token {
            let tt = if b { TokenType::True } else { TokenType::False };
            Token {
                typ: tt.clone(),
                ln,
                s: tt.to_string(),
            }
        }

        pub(crate) fn of_string(s: &str, ln: LineNum) -> Token {
            Token {
                typ: TokenType::StringLiteral,
                ln,
                s: s.to_string(),
            }
        }

        pub(crate) fn of_numeric(n: Numeric, ln: LineNum) -> Token {
            Token {
                typ: TokenType::Number(n.clone()),
                ln,
                s: n.to_string(),
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
    #[test]
    fn eval_true() {
        simple_eval_value(true);
    }

    #[test]
    fn eval_false() {
        simple_eval_value(false);
    }

    #[test]
    fn eval_nil() {
        let expr = crate::parser::Expression::Primary(Token::nil(1));
        let mut e = Evaluator::new();
        if let Ok(EvalResult::Reserved { value, token }) = e.eval_expr(expr) {
            assert_eq!(value, "nil");
            assert_eq!(token.typ, TokenType::Nil)
        }
    }
    #[test]
    fn eval_string() {
        let expr = crate::parser::Expression::Primary(Token::of_string("hello", 1));
        let mut e = Evaluator::new();
        match e.eval_expr(expr) {
            Ok(EvalResult::String { value, token }) => {
                assert_eq!(value, "hello");
                assert_eq!(token.typ, TokenType::StringLiteral);
            }
            Ok(other_than_str) => panic!("{} should evauate ot string result", other_than_str),
            Err(error) => {
                panic!("EvalError: {}", error)
            }
        }
    }

    #[test]
    fn eval_number() {
        let expr = crate::parser::Expression::Primary(Token::of_numeric(Numeric(12f64), 1));
        let mut e = Evaluator::new();
        match e.eval_expr(expr) {
            Ok(EvalResult::Numeric { value, token }) => {
                assert_eq!(value, 12f64);
                assert_eq!(token.typ, TokenType::Number(token::Numeric(12f64)))
            }
            other => {
                panic!(
                    "Expression should evaluate to numeric, evaluated to {:?}",
                    other
                )
            }
        }
    }

    fn simple_eval_value(b: bool) {
        let expr = crate::parser::Expression::Primary(Token::of_bool(b, 1));
        let mut e = Evaluator::new();
        match e.eval_expr(expr) {
            Ok(EvalResult::Boolean { value, token }) => {
                assert_eq!(value, b);
                assert_eq!(
                    token.typ,
                    if b { TokenType::True } else { TokenType::False }
                )
            }
            _ => (),
        }
    }
}
