use std::{error::Error, fmt::Display};

use crate::{
    parser::{Binary, Expression, Program, Stmt, Unary},
    token::{Numeric, Token, TokenType},
};

pub type Result = std::result::Result<EvalResult, EvalError>;
pub type StatementResult = std::result::Result<StatementEvalResult, EvalError>;

/// Results of Stmt evaluation
#[derive(Debug, PartialEq)]
pub enum StatementEvalResult {
    ExpressionStatementResult(EvalResult),
    PrintStatementResult(EvalResult)
}
/// Result of expression evaluation
#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Numeric { value: f64, token: Token },
    Boolean { value: bool, token: Token },
    String { value: String, token: Token },
    Reserved { value: String, token: Token },
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
                value: value.clone(),
                token: token.clone(),
            },
            Self::Boolean { value, token } => Self::Boolean {
                value: value.clone(),
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
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
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

    fn eval_unary(&self, unary: Unary, ex: Expression) -> Result {
        let res = self.eval_expr(ex);
        match res {
            Ok(val) => match val {
                EvalResult::Numeric {
                    value: n,
                    token: ref tok,
                } => match unary {
                    Unary::Minus => Ok(EvalResult::of_numeric(-n, tok)),
                    Unary::Not => Ok(EvalResult::of_boolean(n == 0.0, tok)),
                    _ => runtime_error("Numeric arg can only be used with <minus> operator", tok.ln),
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

    fn eval_binary(&self, lex: Expression, op: Binary, rex: Expression) -> Result {
        let lr = self.eval_expr(lex);
        let rr = self.eval_expr(rex);

        calculate(lr?, op, rr?)
    }

    pub(crate) fn eval(&self, p: Program) -> Vec<StatementResult>{
        p.statements.iter().map(|s| self.eval_stmt(s.clone())).collect()

    }

    fn eval_stmt(&self, s: Stmt) -> StatementResult {
        let res = match s {
            Stmt::Print(e) => {
                self.eval_expr(e).map(|r| StatementEvalResult::PrintStatementResult(r))
                //Ok(StatementEvalResult::PrintStatementResult(self.eval_expr(e)))
            },
            Stmt::Expression(e) => {
                self.eval_expr(e).map(|r| StatementEvalResult::ExpressionStatementResult(r))
                //Ok(StatementEvalResult::ExpressionStatementResult(self.eval_expr(e)))
            },
        };

        res
    }

    pub fn eval_expr(&self, e: Expression) -> Result {
        match e {
            Expression::Primary(t) => self.eval_primary(t),
            Expression::Paren(e) => self.eval_expr(*e),
            Expression::UnaryEx(unary, ex) => self.eval_unary(unary, *ex),
            Expression::BinaryEx(l, op, r) => self.eval_binary(*l, op, *r),
            Expression::Invalid(s) => Err(EvalError::new(format!("Invalid expresstion: {}", s))),
        }
    }
}

fn runtime_error(arg: &str, ln: u64) -> Result {
    Err(EvalError { s: format!("{}\n[Line {}]", arg.to_string(), ln) })
}

fn calculate(lv: EvalResult, op: Binary, rv: EvalResult) -> Result {
    let res = match lv {
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
                Binary::InvalidBinary(_) => runtime_error("Invalid binary operator", ltok.ln),
            },
            EvalResult::String {
                value: ref _s,
                token: ref _rtok,
            } => match op {
                Binary::EqualEqual => Ok(EvalResult::of_boolean(false, ltok)),
                Binary::NotEqual => Ok(EvalResult::of_boolean(false, ltok)),
                Binary::Plus => runtime_error("Operands must be two numbers or two strings.", ltok.ln),
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
                Binary::Plus => runtime_error("Operands must be two numbers or two strings.", ltok.ln),
                _ => runtime_error("Only str*num and str+str allowed", ltok.ln),
            },
            _ => runtime_error("No other binary operations on strings", ltok.ln),
        },
        EvalResult::Boolean { value: lv, token: ltok } => match rv {
            EvalResult::Boolean { value: rv, token: _rtok } => match op {
                Binary::EqualEqual => Ok(EvalResult::Boolean { value: lv == rv, token: ltok }),
                Binary::NotEqual => Ok(EvalResult::Boolean { value: lv != rv, token: ltok }),
                _ => runtime_error("Bool operators allowed: only == and !=.", ltok.ln)
            },
            _ => match op {
                Binary::EqualEqual => Ok(EvalResult::Boolean { value: false, token: ltok }),
                Binary::NotEqual => Ok(EvalResult::Boolean { value: true, token: ltok }),
                Binary::Multiply => runtime_error("Operands must be numbers.", ltok.ln),
                _ => runtime_error("Operator not supported", ltok.ln)
            }
        }

        _ => runtime_error("Expected numeric arg", 1),
    };
    // eprint!(
    //     "left: {} right: {}, op: {}| result: {:?}\n",
    //     lv.clone(),
    //     rv.clone(),
    //     op,
    //     res
    // );
    res
}

#[cfg(test)]
mod test_evaluator {
    use core::panic;

    use crate::{
        evaluator::EvalResult,
        token::{self, Numeric, Token, TokenType},
    };

    use super::Evaluator;

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
        let e = Evaluator::new();
        if let Ok(EvalResult::Reserved { value, token }) = e.eval_expr(expr) {
            assert_eq!(value, "nil");
            assert_eq!(token.typ, TokenType::Nil)
        }
    }
    #[test]
    fn eval_string() {
        let expr = crate::parser::Expression::Primary(Token::of_string("hello", 1));
        let e = Evaluator::new();
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
        let e = Evaluator::new();
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
        let e = Evaluator::new();
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
