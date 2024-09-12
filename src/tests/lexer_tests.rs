use crate::{
    lexer::Lexer,
    parser::Parser,
    token::{LexicalError, Numeric, Token, TokenType},
};

// single token

#[cfg(test)]
fn new_from_tokentype_vec(v: Vec<TokenType>) -> Parser {
    Parser::new(
        v.into_iter()
            .map(|tt| Token::new(tt, 0, "".into()))
            .collect(),
    )
}

#[test]
fn parses_true() {
    let mut p = new_from_tokentype_vec(vec![TokenType::True, TokenType::Eof]);
    assert_eq!(p.parse().to_string(), "true");
}

#[test]
#[ignore = "reason"]
fn parses_false() {
    let mut p = new_from_tokentype_vec(vec![TokenType::False]);
    assert_eq!(p.parse().to_string(), "false");
}

#[test]
#[ignore = "I don;t know why empty stmt"]
fn parses_nil() {
    let mut p = new_from_tokentype_vec(vec![TokenType::Nil]);
    let prog = p.parse();
    eprint!("Program in parses_nil is: {:?}", prog);
    assert_eq!(prog.to_string(), "nil");
}
#[test]
#[ignore = "reason"]
fn parses_numeric() {
    let mut p = new_from_tokentype_vec(vec![TokenType::Number(Numeric(43.47f64))]);
    assert_eq!(p.parse().to_string(), "43.47");
}

#[test]
fn parses_unary_not_true() {
    let mut p = new_from_tokentype_vec(vec![TokenType::Bang, TokenType::True]);
    assert_eq!(p.parse().to_string(), "(! true)");
}

#[test]
fn parsing_number_should_display_dotzero() {
    if let Some(v) = Lexer::new("65").tokens().first() {
        assert_eq!(v.to_string(), "NUMBER 65 65.0");
    }
}

#[test]
fn parsing_number_should_display_all_decimal_digits() {
    if let Some(v) = Lexer::new("65.1234").tokens().first() {
        assert_eq!(v.to_string(), "NUMBER 65.1234 65.1234");
    }
}

#[test]
fn parses_unary_minus() {
    let mut p = new_from_tokentype_vec(vec![TokenType::Minus, TokenType::Number(Numeric(10.0f64))]);
    assert_eq!(p.parse().to_string(), "(- 10.0)");
}

/// Testing token types of tokens resulting  from parsing given string
#[cfg(test)]
fn assert_token_vec_lexing_result(s: &str, expected: Vec<TokenType>) {
    assert_eq!(
        Lexer::new(&s)
            .tokens()
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
    let ts = Lexer::new("1").tokens();
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

///
/// Lexer's tokens gathered in to_string should give string result
#[cfg(test)]
fn compare(text: &str, result: &str) {
    eprintln!("Compare for text: {}\n", text);
    let actual: String = Lexer::new(text)
        .tokens()
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
