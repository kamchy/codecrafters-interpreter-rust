/// This module tests evaluation cases in "cases.txt file".
/// The file must include only one-liners (cases are read line by line)

#[cfg(test)]
struct Case {
    code: String,
    evaluated: String
}

#[cfg(test)]
fn prepare<'a>(fname: &'a str) -> Vec<Case> {
    use crate::utils;

    let c = utils::contents(fname);
    let mut cases = Vec::new();
    let lines = c.split("\n");
    for line in lines {
        let word_iter = line.split("\t").map(String::from).collect::<Vec<String>>();
        eprint!("Word iter: {:?}\n", word_iter);
        let case = Case { code: word_iter.get(0).unwrap().to_string(), evaluated: word_iter.get(1).unwrap().to_string().replace("\\n", "\n") };
        cases.push(case);
    }
    cases
}

#[cfg(test)]
fn run_case(c: Case) {
    use crate::{evaluate_with_code, evaluator::StatementEvalResult};

    let (ve, opt_err, _code) = evaluate_with_code(&c.code);

    if let Some(err) = opt_err {
        let s = err.to_string();
        assert_eq!(s, c.evaluated, "Error: should be {}, was {}", c.evaluated, s);
    } else  if let Some(v)  = ve.first() {
        let actual_expression_string: String = match v {
            StatementEvalResult::PrintStatementResult(fin) => fin.to_string(),
            StatementEvalResult::ExpressionStatementResult(fin) => fin.to_string(),
            StatementEvalResult::BlockResult(vec) => format!(
                "{:?}", vec
            ),
        };
        assert_eq!(c.evaluated,actual_expression_string, "Expected: {}, actual {}",  c.evaluated, actual_expression_string);
    }

}

#[test]
fn test_cases() {
    for c in prepare("./src/tests/cases.txt") {
        run_case(c);
    }
}
