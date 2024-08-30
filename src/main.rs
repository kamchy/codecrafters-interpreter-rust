use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
mod lexer;
mod token;
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });
            tokenize(&file_contents)
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
}

pub(crate) fn tokenize_string(s: &str) -> Vec<token::Token> {
    let lexer = lexer::Lexer::new(s);
    lexer.into_iter().collect()
}

fn tokenize(s: &str) -> ExitCode {
    let lexer = lexer::Lexer::new(s);
    let mut exit_code = ExitCode::SUCCESS;
    for token in lexer {
        match token {
            token::Token::Unknown(_, _) => {
                eprintln!("{}", token);
                exit_code = ExitCode::from(65);
            }
            _ => println!("{}", token),
        }
    }
    exit_code
}
#[cfg(test)]
mod tests {
    use super::token::*;
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(tokenize_string("("), vec![Token::LeftParen, Token::Eof]);
    }

    #[test]
    fn number12() {
        assert_eq!(
            tokenize_string("12"),
            vec![
                Token::Number("12".to_string(), Numeric(12.0f64)),
                Token::Eof
            ]
        )
    }

    #[test]
    fn number1() {
        assert_eq!(
            tokenize_string("1"),
            vec![Token::Number("1".to_string(), Numeric(1.0)), Token::Eof]
        )
    }

    #[test]
    fn number_12_5() {
        assert_eq!(
            tokenize_string("12.5"),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::Eof
            ]
        )
    }
    #[test]
    fn number_12_5_3() {
        assert_eq!(
            tokenize_string("12.5 3"),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::Number("3".to_string(), Numeric(3f64)),
                Token::Eof
            ]
        )
    }

    #[test]
    fn number_12_str() {
        assert_eq!(
            tokenize_string("12.5 \"abc\""),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::StringLiteral("abc".to_string()),
                Token::Eof
            ]
        )
    }
    #[test]
    fn number_12_eol_str() {
        assert_eq!(
            tokenize_string("12.5\n\"abc\""),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::StringLiteral("abc".to_string()),
                Token::Eof
            ]
        )
    }

    #[test]
    fn number_12_tab_str() {
        assert_eq!(
            tokenize_string("12.5\t\"abc\""),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::StringLiteral("abc".to_string()),
                Token::Eof
            ]
        )
    }

    #[test]
    fn invalid_token() {
        assert_eq!(
            tokenize_string("%"),
            vec![
                Token::Unknown(1, LexicalError::UnknownToken('%')),
                Token::Eof
            ]
        )
    }
    #[test]
    fn invalid_second_line() {
        assert_eq!(
            tokenize_string("12.5\n%"),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::Unknown(2, LexicalError::UnknownToken('%')),
                Token::Eof
            ]
        )
    }

    #[test]
    fn invalid_2_and_4_line() {
        assert_eq!(
            tokenize_string("12.5\n%\n23\n6.34f"),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::Unknown(2, LexicalError::UnknownToken('%')),
                Token::Number("23".to_string(), Numeric(23f64)),
                Token::Number("6.34".to_string(), Numeric(6.34f64)),
                Token::Unknown(4, LexicalError::UnknownToken('f')),
                Token::Eof
            ]
        )
    }

    #[test]
    fn invalid() {
        assert_eq!(
            tokenize_string("12.5a"),
            vec![
                Token::Number("12.5".to_string(), Numeric(12.5f64)),
                Token::Unknown(1, LexicalError::UnknownToken('a')),
                Token::Eof
            ]
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
}
