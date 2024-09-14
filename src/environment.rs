/// Represents environment - variables and their values in Lox program
/// This is part of evaluator
use std::collections::HashMap;

use crate::evaluator::EvalResult;

pub(crate) struct Environment {
    values: HashMap<String, EvalResult>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Environment {
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
}
