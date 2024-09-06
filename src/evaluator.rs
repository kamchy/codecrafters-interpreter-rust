use std::{
    error::Error,
    f64,
    fmt::{Display, Write},
    str::FromStr,
};

use crate::{
    parser::{Binary, Expression, Unary},
    token::{Numeric, Token, TokenType},
};

pub type Result = std::result::Result<EvalResult, EvalError>;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Numeric(f64),
    Boolean(bool),
    String(String),
    Reserved(String),
}
impl Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Numeric(v) => v.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::String(s) => s.to_string(),
            Self::Reserved(s) => s.to_string(),
        };
        f.write_str(&s)
    }
}
// trait ResultBox<T> {
//     fn get(&self) -> T;
// }
// impl<T> ResultBox<T> for EvalResult {
//     fn get(&self) -> T {
//         match *self {
//             Numeric(n) => n,
//             Boolean(b) => b,
//             String(s) => s,
//             Reserved(s) => s,
//         }
//     }

#[derive(Debug)]
pub struct EvalError {
    s: String,
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

/// Creates Err variant from statuc string
fn err(s: &'static str) -> Result {
    Err(EvalError { s: s.into() })
}

/// Creates Ok variant from numeric value
fn ok_num(n: f64) -> Result {
    Ok(EvalResult::Numeric(n))
}

/// Evaluator of expressions
pub struct Evaluator {}

impl Evaluator {
    fn new() -> Self {
        Evaluator {}
    }

    fn eval_primary(&self, t: Token) -> Result {
        match t.typ {
            TokenType::True => Ok(EvalResult::Boolean(true)),
            TokenType::False => Ok(EvalResult::Boolean(false)),
            TokenType::Nil => Ok(EvalResult::Reserved("nil".to_string())),
            TokenType::StringLiteral => Ok(EvalResult::String(t.s.clone())),
            TokenType::Number(Numeric(f)) => Ok(EvalResult::Numeric(f)),
            _ => Err(EvalError::new("unimplemented!".into())),
        }
    }

    fn eval_unary(&self, unary: Unary, ex: Expression) -> Result {
        let res = self.eval(ex);
        match res {
            Ok(val) => match val {
                EvalResult::Numeric(n) => match unary {
                    Unary::Minus => Ok(EvalResult::Numeric(-n)),
                    Unary::Not => Ok(EvalResult::Boolean(n == 0.0)),
                    _ => err("Numeric arg can only be used with <minus> operator"),
                },
                EvalResult::Reserved(word) => match unary {
                    Unary::Not => match word.to_lowercase().as_str() {
                        "nil" => Ok(EvalResult::Boolean(true)),
                        _ => err("Reselved word has no unary oper"),
                    },
                    _ => err("Unary operator not supported"),
                },
                EvalResult::Boolean(v) => match unary {
                    Unary::Not => Ok(EvalResult::Boolean(!v)),
                    _ => Err(EvalError {
                        s: "Bool arg can only be used with negation".into(),
                    }),
                },
                _ => Err(EvalError {
                    s: "No unary operators for string".into(),
                }),
            },
            Err(_) => res,
        }
    }

    fn eval_binary(&self, lex: Expression, op: Binary, rex: Expression) -> Result {
        let lr = self.eval(lex);
        let rr = self.eval(rex);

        calculate(lr?, op, rr?)
    }

    pub fn eval(&self, e: Expression) -> Result {
        match e {
            Expression::Primary(t) => self.eval_primary(t),
            Expression::Paren(e) => self.eval(*e),
            Expression::UnaryEx(unary, ex) => self.eval_unary(unary, *ex),
            Expression::BinaryEx(l, op, r) => self.eval_binary(*l, op, *r),
            Expression::Invalid(s) => Err(EvalError::new(format!("Invalid expresstion: {}", s))),
        }
    }
}

fn calculate(lv: EvalResult, op: Binary, rv: EvalResult) -> Result {

    let res = match lv {
        EvalResult::Numeric(l) => match rv {
            EvalResult::Numeric(r) => match op {
                Binary::Plus => Ok(EvalResult::Numeric(l + r)),
                Binary::Minus => Ok(EvalResult::Numeric(l - r)),
                Binary::Divide => Ok(EvalResult::Numeric(l / r)),
                Binary::Multiply => Ok(EvalResult::Numeric(l * r)),
                Binary::Less => Ok(EvalResult::Boolean(l < r)),
                Binary::LessEqual => Ok(EvalResult::Boolean(l <= r)),
                Binary::Greater => Ok(EvalResult::Boolean(l > r)),
                Binary::GreaterEqual => Ok(EvalResult::Boolean(l >= r)),
                Binary::EqualEqual => Ok(EvalResult::Boolean(l == r)),
                Binary::NotEqual => Ok(EvalResult::Boolean(l != r)),
                Binary::InvalidBinary(_) => err("Invalid binary operator"),
            },
            EvalResult::String(ref r) => match op {

                Binary::EqualEqual =>  Ok(EvalResult::Boolean(false)),
                Binary::NotEqual => Ok(EvalResult::Boolean(false)),
                _=> err("Only num != str and num == str supported")
            }
            _ => err("Right arg should be numeric"),
        },
        EvalResult::String(ref l) => match rv {

            EvalResult::String(ref r) => match op {
                Binary::Plus => Ok(EvalResult::String(l.to_owned() + r)),
                Binary::EqualEqual => Ok(EvalResult::Boolean(l == r)),
                Binary::NotEqual => Ok(EvalResult::Boolean(l != r)),
                _ => err("Only plus allowed on strings")
            }
            EvalResult::Numeric(n) => match op {
                Binary::Multiply => Ok(EvalResult::String(l.repeat(n.round() as usize))),
                _ => err("Only str*num and str+str allowed")
            },
            _ => err("No other binary operations on strings")
        }
        _ => err("Expected numeric arg"),
    };
    eprint!("left: {} right: {}, op: {}| result: {:?}\n", lv, rv, op, res);
    res
}

#[cfg(test)]
mod test_evaluator {
    use crate::{
        evaluator::EvalResult,
        token::{Numeric, Token},
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
        if let Ok(res) = e.eval(expr) {
            assert_eq!(res, EvalResult::Reserved("nil".to_owned()))
        }
    }
    #[test]
    fn eval_string() {
        let expr = crate::parser::Expression::Primary(Token::of_string("hello", 1));
        let e = Evaluator::new();
        if let Ok(res) = e.eval(expr) {
            assert_eq!(res, EvalResult::String("hello".to_string()))
        }
    }

    #[test]
    fn eval_number() {
        let expr = crate::parser::Expression::Primary(Token::of_numeric(Numeric(12f64), 1));
        let e = Evaluator::new();
        if let Ok(res) = e.eval(expr) {
            assert_eq!(res, EvalResult::Numeric(12f64))
        }
    }

    fn simple_eval_value(b: bool) {
        let expr = crate::parser::Expression::Primary(Token::of_bool(b, 1));
        let e = Evaluator::new();
        if let Ok(res) = e.eval(expr) {
            assert_eq!(res, EvalResult::Boolean(b))
        }
    }
}
