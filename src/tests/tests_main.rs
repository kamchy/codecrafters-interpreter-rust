use core::panic;

use crate::{token::*, *};



#[test]
fn parse_with_exit_0() {
    let (_, code) = parse_with_code("(true)");
    assert_eq!(code, 0)
}

#[test]
fn parse_with_exit_65() {
    let (_, code) = parse_with_code("(true");
    assert_eq!(code, 65)
}


#[derive(Debug)]
#[cfg(test)]
struct Case<'a> {
    inp: &'a str,
    outp: &'a str,
    code: u8,
}

#[test]
fn evaluate_cases() {
    let cases: Vec<Case> = vec![
        Case {
            inp: "-73",
            outp: "-73",
            code: 0,
        },
        Case {
            inp: "!true",
            outp: "false",
            code: 0,
        },
        Case {
            inp: "!10.40",
            outp: "false",

            code: 0,
        },
        Case {
            inp: "!((false))",
            outp: "true",
            code: 0,
        },
        Case {
            inp: "!nil",
            outp: "true",
            code: 0,
        },
        Case {
            inp: "1+3",
            outp: "4",
            code: 0,
        },
        Case {
            inp: "\"hello\" + \"word\"",
            outp: "helloword",
            code: 0,
        },
        Case {
            inp: "\"foo\"* 3",
            outp: "foofoofoo",
            code: 0,
        },
        Case {
            inp: "42 / 5 ",
            outp: "8.4",
            code: 0,
        },
        Case {
            inp: "18 * 3 / (3 * 6) ",
            outp: "3",
            code: 0,
        },
        Case {
            inp: "-\"foo\" ",
            outp: "Operand must be a number.\n[Line 1]",
            code: 70,
        },
        Case {
            inp: "-true",
            outp: "Operand must be a number.\n[Line 1]",
            code: 70,
        },
        Case {
            inp: "-(\"foo\" + \"bar\") ",
            outp: "Operand must be a number.\n[Line 1]",
            code: 70,
        },
        Case {
            inp: " \"foo\n  \n bar\" ",
            outp: "foo\n  \n bar",
            code: 0,
        },
        Case {
            inp: " 234h ",
            outp: "Invalid expresstion: [line 1] Error at h: Expected primary (number,  string, bool, nil)  or left paren",
            code: 65,
        },
        Case {
            inp: " if 3",
            outp: "Invalid expresstion: [line 1] Error at if: Expected primary (number,  string, bool, nil)  or left paren",
            code: 65,
        },
    ];

    for c in cases {

        let (eres, num, actual_code) = evaluate_with_code(c.inp);
        eprint!("|Err: {:?} | Input: {} | Expr: {:?} \n", num, c.inp, eres);
        assert_eq!(actual_code, c.code, "Expected code: {}, got: {}", c.code, actual_code);

        if let Some(err) = num {
            let s = err.to_string();
            assert_eq!(s, c.outp, "Error: should be {}, was {}", c.outp, s);
        } else  if let Some(v)  = eres.first() {
            let actual_expression_string: String = match v {
                StatementEvalResult::PrintStatementResult(fin) => fin.to_string(),
                StatementEvalResult::ExpressionStatementResult(fin) => fin.to_string(),
            };
            assert_eq!(c.outp,actual_expression_string, "Expected: {}, actual {}",  c.outp, actual_expression_string);
        }

        // match (r, num, code) {
        //     (eres, num) => {
        //         let v = eres.first().unwrap();
        //         let outv: String = match v {
        //             StatementEvalResult::PrintStatementResult(fin) => fin.to_string(),
        //             StatementEvalResult::ExpressionStatementResult(fin) => fin.to_string(),
        //         };
        //         assert_eq!(
        //             true,
        //             outv.starts_with(c.outp),
        //             "Testing case {:?} got {}",
        //             c,
        //             outv
        //         );
        //         assert_eq!(num, c.code);
        //     }
        //     (Err(ee), num) => {
        //         if ee.s.starts_with("Operand must be a number.") {
        //             assert_eq!(num, RUNTIME_ERRROR_CODE);
        //             assert_eq!(ee.s, "Operand must be a number.\n[Line 1]");
        //         } else {
        //             assert_eq!(ee.s, c.outp);
        //             assert_eq!(num, c.code, "Case {:?} has code {}", c, num);
        //         }
        //     }
        // }
    }
}
