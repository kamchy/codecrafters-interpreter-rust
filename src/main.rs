use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
mod lexer;
mod parser;
mod token;
use token::{Token, TokenType};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(&contents(&filename)),
        "parse" => parse(&contents(&filename)),
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
}
fn contents(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    })
}
pub(crate) fn tokenize_string(s: &str) -> Vec<token::Token> {
    let lexer = lexer::Lexer::new(s);
    lexer.into_iter().collect()
}

fn tokenize(s: &str) -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;
    for token in tokenize_string(s) {
        match token.typ {
            token::TokenType::Unknown(_) => {
                eprintln!("{}", token);
                exit_code = ExitCode::from(65);
            }
            _ => println!("{}", token),
        }
    }
    exit_code
}

fn parse(s: &str) -> ExitCode {
    let tokens: Vec<Token> = tokenize_string(s);
    let mut parser = parser::Parser::new(tokens);
    println!("{}", parser.parse());
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests_main {

    use super::token::*;
    use super::*;

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
}
