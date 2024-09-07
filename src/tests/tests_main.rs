use core::panic;

use crate::{token::*, *};


fn assert_token_vec_lexing_result(s: &str, expected: Vec<TokenType>) {
    assert_eq!(
        tokenize_string(&s)
            .iter()
            .map(|t| t.typ.clone())
            .collect::<Vec<_>>(),
        expected
    );
}
#[test]
fn it_works() {
    assert_token_vec_lexing_result("(", vec![TokenType::LeftParen, TokenType::Eof]);
}

#[test]
fn number12() {
    assert_token_vec_lexing_result(
        "12",
        vec![TokenType::Number(Numeric(12.0f64)), TokenType::Eof],
    );
}

#[test]
fn number1() {
    assert_token_vec_lexing_result("1", vec![TokenType::Number(Numeric(1.0)), TokenType::Eof]);
    let ts = tokenize_string("1");
    assert_eq!(ts.first().unwrap().typ.to_string(), "1.0");
}

#[test]
fn number_12_5() {
    assert_token_vec_lexing_result(
        "12.5",
        vec![TokenType::Number(Numeric(12.5f64)), TokenType::Eof],
    );
}
#[test]
fn number_12_5_3() {
    assert_token_vec_lexing_result(
        "12.5 3",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::Number(Numeric(3f64)),
            TokenType::Eof,
        ],
    )
}

#[test]
fn number_12_str() {
    assert_token_vec_lexing_result(
        "12.5 \"abc\"",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::StringLiteral,
            TokenType::Eof,
        ],
    )
}
#[test]
fn number_12_eol_str() {
    assert_token_vec_lexing_result(
        "12.5\n\"abc\"",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::StringLiteral,
            TokenType::Eof,
        ],
    )
}

#[test]
fn number_12_tab_str() {
    assert_token_vec_lexing_result(
        "12.5\t\"abc\"",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::StringLiteral,
            TokenType::Eof,
        ],
    )
}

#[test]
fn invalid_token() {
    assert_token_vec_lexing_result(
        "%",
        vec![
            TokenType::Unknown(LexicalError::UnknownToken('%')),
            TokenType::Eof,
        ],
    )
}
#[test]
fn invalid_second_line() {
    assert_token_vec_lexing_result(
        "12.5\n%",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::Unknown(LexicalError::UnknownToken('%')),
            TokenType::Eof,
        ],
    )
}

#[test]
fn invalid_2_and_4_line() {
    assert_token_vec_lexing_result(
        "12.5\n%\n23\n6.34f #",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::Unknown(LexicalError::UnknownToken('%')),
            TokenType::Number(Numeric(23f64)),
            TokenType::Number(Numeric(6.34f64)),
            TokenType::Identifier,
            TokenType::Unknown(LexicalError::UnknownToken('#')),
            TokenType::Eof,
        ],
    )
}

#[test]
fn invalid() {
    assert_token_vec_lexing_result(
        "12.5a",
        vec![
            TokenType::Number(Numeric(12.5f64)),
            TokenType::Identifier,
            TokenType::Eof,
        ],
    )
}

#[test]
fn parsing_number_should_display_dotzero() {
    if let Some(v) = tokenize_string("65").first() {
        assert_eq!(v.to_string(), "NUMBER 65 65.0");
    }
}

#[test]
fn parsing_number_should_display_all_decimal_digits() {
    if let Some(v) = tokenize_string("65.1234").first() {
        assert_eq!(v.to_string(), "NUMBER 65.1234 65.1234");
    }
}

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

fn compare(text: &str, result: &str) {
    eprintln!("Compare for text: {}\n", text);
    let actual: String = tokenize_string(text)
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    assert_eq!(result, actual);
}

#[test]
fn shoould_parse_correctly() {
    let text = r#""quz" = "bar" != (71 == 98)"#;
    let result = r#"STRING "quz" quz
EQUAL = null
STRING "bar" bar
BANG_EQUAL != null
LEFT_PAREN ( null
NUMBER 71 71.0
EQUAL_EQUAL == null
NUMBER 98 98.0
RIGHT_PAREN ) null
EOF  null"#;

    compare(text, result);
}

#[test]
fn minimal_problem() {
    compare(
        r#" == 98)"#,
        "EQUAL_EQUAL == null\nNUMBER 98 98.0\nRIGHT_PAREN ) null\nEOF  null",
    )
}

#[test]
fn parse_unterminated() {
    compare(
        "\"bar\" \"unterminated",
        "STRING \"bar\" bar\n[line 1] Error: Unterminated string.\nEOF  null",
    );
}

#[test]
fn parse_ident() {
    compare("bar", "IDENTIFIER bar null\nEOF  null");
}

#[test]
fn parse_ident_invalid_ident() {
    compare(
        "bar fooą",
        "IDENTIFIER bar null\nIDENTIFIER foo null\n[line 1] Error: Unexpected character: ą\nEOF  null"
    );
}

#[test]
fn parse_ident_unterm() {
    compare(
        "bar \"unterminated",
        "IDENTIFIER bar null\n[line 1] Error: Unterminated string.\nEOF  null",
    );
}

#[test]
fn reserved() {
    compare("for fun", "FOR for null\nFUN fun null\nEOF  null");
}

#[derive(Debug)]
struct Case<'a> {
    inp: &'a str,
    outp: &'a str,
}

#[test]
fn evaluate_cases() {
    let cases: Vec<Case> = vec![
        Case {
            inp: "-73",
            outp: "-73",
        },
        Case {
            inp: "!true",
            outp: "false",
        },
        Case {
            inp: "!10.40",
            outp: "false",
        },
        Case {
            inp: "!((false))",
            outp: "true",
        },
        Case {
            inp: "!nil",
            outp: "true",
        },
        Case {
            inp: "1+3",
            outp: "4",
        },
        Case {
            inp: "\"hello\" + \"word\"",
            outp: "helloword",
        },
        Case {
            inp: "\"foo\"* 3",
            outp: "foofoofoo",
        },
        Case {
            inp: "42 / 5 ",
            outp: "8.4",
        },
        Case {
            inp: "18 * 3 / (3 * 6) ",
            outp: "3",
        },
        Case {
            inp: "-\"foo\" ",
            outp: "Operand must be a number.",
        },
        Case {
            inp: "-true",
            outp: "Operand must be a number.",
        },
        Case {
            inp: "-(\"foo\" + \"bar\") ",
            outp: "Operand must be a number.",
        },


    ];

    for c in cases {
        let (r, num) = evaluate_with_code(c.inp);
        match (r, num) {
            (Ok(eres), num) => {
                assert_eq!(
                c.outp,
                eres.to_string(),
                "Testing case {:?} got {}",
                c,
                eres
            );
            //assert_eq!(num, 60)
        },
           (Err(ee), num) => {
            if ee.s == "Operand must be a number." {
                assert_eq!(num, RUNTIME_ERRROR_CODE)
            } else {
              panic!("case: {:?}, error: {}", c, ee)
            }
           },
        }
    }
}