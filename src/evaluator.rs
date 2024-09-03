use std::{
    error::Error,
    fmt::{Display, Write},
    str::FromStr,
};

use crate::{
    parser::{Binary, Expression, Unary},
    token::{Token, TokenType},
};

pub type Result = std::result::Result<EvalResult, EvalError>;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Numeric(f64),
    Boolean(bool),
    String(String),
}
impl Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Numeric(v) => v.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::String(s) => s.to_string(),
        };
        f.write_str(&s)
    }
}

#[derive(Debug)]
pub struct EvalError {
    s: String,
}

impl EvalError {
    fn new(s: String) -> EvalError {
        EvalError { s }
    }
}
// impl From<&'static str> for EvalError {
//     fn from(s: &'static str) -> EvalError {
//         EvalError { s: s.to_string() }
//     }
// }

// impl From<&str> for EvalError {
//     fn from(s: &str) -> EvalError {
//         EvalError { s: String::from(s) }
//     }
// }
impl Error for EvalError {}
impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.s)
    }
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
            _ => Err(EvalError::new("unimplemented!".into())),
        }
    }

    fn eval_unary(&self, unary: Unary, ex: Expression) -> Result {
        let res = self.eval(ex);
        match res {
            Ok(val) => match val {
                EvalResult::Numeric(n) => match unary {
                    Unary::Minus => Ok(EvalResult::Numeric(-n)),
                    _ => Err(EvalError {
                        s: "Numeric arg can only be used with <minus> operator".into(),
                    }),
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

    fn eval_binary(&self, _lex: Expression, _op: Binary, _rex: Expression) -> Result {
        Err(EvalError::new("unimplemented!()".into()))
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

#[cfg(test)]
mod test_evaluator {
    use crate::{evaluator::EvalResult, token::Token};

    use super::Evaluator;

    #[test]
    fn eval_true() {
        simple_eval_value(true);
    }

    #[test]
    fn eval_false() {
        simple_eval_value(false);
    }

    fn simple_eval_value(b: bool) {
        let expr = crate::parser::Expression::Primary(Token::of_bool(b, 1));
        let e = Evaluator::new();
        if let Ok(res) = e.eval(expr) {
            assert_eq!(res, EvalResult::Boolean(b))
        }
    }
}
