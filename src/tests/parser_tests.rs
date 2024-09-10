use crate::lexer::Lexer;

#[cfg(test)]
fn assert_parsed_text_result(text: &str, expected: &str) {
    use crate::{lexer, parser::Parser};

    let s = text;
    let lex = lexer::Lexer::new(s);
    let ts: Vec<_> = lex.collect();

    eprintln!("Test:\ntext: {}\ntokens: {:?}\n ", text, ts);
    let mut p = Parser::new(ts);
    let prog = p.parse();

    assert_eq!(prog.to_string(), expected);
}
#[test]
fn parses_true2() {
    assert_parsed_text_result("true", "true");
}

#[test]
fn parses_false2() {
    assert_parsed_text_result("false", "false");
}

#[test]
fn parses_nil2() {
    assert_parsed_text_result("nil", "nil");
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
fn parses_unary_not_true_2() {
    assert_parsed_text_result("!true", "(! true)")
}

#[test]
fn parses_unary_not_false() {
    assert_parsed_text_result("! false", "(! false)")
}
#[test]
fn parses_unary_minus_2() {
  assert_parsed_text_result("- 3", "(- 3.0)")
}


#[test]
fn parses_unary_minus_in_parens() {
    assert_parsed_text_result("( - 10)", "(group (- 10.0))")
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
