/// Represents environment - variables and their values in Lox program
/// This is part of evaluator
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::{
    evaluator::{EvalError, EvalResult},
    tests,
};
#[derive(Clone)]
pub(crate) struct Environment {
    parent: Option<Box<Environment>>,
    values: HashMap<String, EvalResult>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Environment {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub(crate) fn from(env: Environment) -> Self {
        Environment {
            parent: Some(Box::new(env)),
            values: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: EvalResult) -> EvalResult {
        self.values.insert(name, value.clone());
        value
    }

    pub(crate) fn get_var(&self, s: &str) -> Option<EvalResult> {
        self.values.get(s).map(EvalResult::clone)
    }

    pub(crate) fn assign(
        &mut self,
        t: &crate::token::Token,
        er: EvalResult,
    ) -> std::result::Result<EvalResult, EvalError> {
        if !self.values.contains_key(&t.s) {
            Err(EvalError {
                s: format!("Undefined variable '{}'", &t.s),
            })
        } else {
            self.values.insert(t.s.clone(), er.clone());
            Ok(er)
        }
    }
}

#[cfg(test)]
mod environment_test {
    use crate::{evaluator::EvalResult, tests, token::{Token, TokenType}};

    use super::Environment;

    #[test]
    fn create_empty() {
        let env = Environment::new();
        assert_eq!(env.get_var("foo"), None, "Foo should be none")

   }

   #[test]
   fn create_and_add() {

        let mut env = Environment::new();

        let _er = env.define("fii".to_string(), crate::evaluator::EvalResult::String { value: "abc".to_string(), token: Token::of_string("", 34)} );
        match env.get_var("fii") {
            Some(er) => assert!(matches!(er, EvalResult::String{ value, token} if value == "abc" && token.typ == TokenType::StringLiteral)),
            None => panic!("Environment should get value for fii")
        }
   }
}
