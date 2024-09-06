use crate::{
    lexer, parser::Parser, token::{Numeric, Token, TokenType}
};

impl Parser {
    fn new_from_tokentype_vec(v: Vec<TokenType>) -> Parser {
        Parser::new(
            v.into_iter()
                .map(|tt| Token::new(tt, 0, "".into()))
                .collect(),
        )
    }
}
#[test]
fn parses_true() {
    let mut p = Parser::new_from_tokentype_vec(vec![TokenType::True, TokenType::Eof]);
    assert_eq!(p.parse().to_string(), "true");
}

#[test]
fn parses_true2() {
    assert_parsed_text_result("true", "true");
}

#[test]
fn parses_false() {
    let mut p = Parser::new_from_tokentype_vec(vec![TokenType::False]);
    assert_eq!(p.parse().to_string(), "false");
}

#[test]
fn parses_false2() {
    assert_parsed_text_result("false", "false");
}

#[test]
fn parses_nil() {
    let mut p = Parser::new_from_tokentype_vec(vec![TokenType::Nil]);
    assert_eq!(p.parse().to_string(), "nil");
}

#[test]
fn parses_nil2() {
    assert_parsed_text_result("nil", "nil");
}

#[test]
fn parses_numeric() {
    let mut p = Parser::new_from_tokentype_vec(vec![TokenType::Number(Numeric(43.47f64))]);
    assert_eq!(p.parse().to_string(), "43.47");
}

#[test]
fn parses_numeric2() {
    assert_parsed_text_result("43.47", "43.47");
}

#[test]
fn parses_numeric3() {
    assert_parsed_text_result("43", "43.0");
}

#[test]
fn parses_literal_2() {
    assert_parsed_text_result("\"foo\"", "foo");
}

#[test]
fn parse_paren_string_2() {
    assert_parsed_text_result("(\"foo\")", "(group foo)");
}

#[test]
fn parses_unary_not_true() {
    let mut p = Parser::new_from_tokentype_vec(vec![TokenType::Bang, TokenType::True]);
    assert_eq!(p.parse().to_string(), "(! true)");
}

#[test]
fn parses_unary_not_true_2() {
    assert_parsed_text_result("!true", "(! true)")
}

#[test]
fn parses_unary_not_false() {
    assert_parsed_text_result("! false", "(! false)")
}

#[test]
fn parses_unary_minus() {
    let mut p = Parser::new_from_tokentype_vec(vec![
        TokenType::Minus,
        TokenType::Number(Numeric(10.0f64)),
    ]);
    assert_eq!(p.parse().to_string(), "(- 10.0)");
}

#[test]
fn parses_unary_minus_2() {
    assert_parsed_text_result("- 3", "(- 3.0)")
}

#[test]
fn parses_unary_minus_in_parens() {
    assert_parsed_text_result("( - 10)", "(group (- 10.0))")
}

fn assert_parsed_text_result(text: &str, expected: &str) {
    let s = text;
    let lex = lexer::Lexer::new(s);
    let ts: Vec<_> = lex.collect();

    eprintln!("Test:\ntext: {}\ntokens: {:?}\n ", text, ts);
    let mut p = Parser::new(ts);
    assert_eq!(p.parse().to_string(), expected);
}

#[test]
fn parses_unary_multiple() {
    assert_parsed_text_result("(!!(true))", "(group (! (! (group true))))")
}

#[test]
fn parses_binary() {
    assert_parsed_text_result("16 * 38 / 58", "(/ (* 16.0 38.0) 58.0)")
}

#[test]
fn parses_binary_plus() {
    assert_parsed_text_result("16 + 38 * 58", "(+ 16.0 (* 38.0 58.0))")
}

#[test]
fn parses_comparison_operator() {
    assert_parsed_text_result("83 < 99 < 115", "(< (< 83.0 99.0) 115.0)")
}

#[test]
fn parses_equality_operator() {
    assert_parsed_text_result("\"baz\" == \"baz\"", "(== baz baz)")
}

#[test]
fn parses_inequality_operator() {
    assert_parsed_text_result("\"baz\" != \"baz\"", "(!= baz baz)")
}

#[test]
fn parses_equality_operator2() {
    assert_parsed_text_result(
        "! (\"baz\" == \"baz\") > 5",
        "(> (! (group (== baz baz))) 5.0)",
    )
}
