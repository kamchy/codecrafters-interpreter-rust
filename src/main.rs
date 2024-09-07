use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
mod evaluator;
mod lexer;
mod parser;
mod utils;
pub mod tests;
mod token;
use evaluator::EvalError;
use evaluator::EvalResult;
use token::Token;
use utils::contents;
const RUNTIME_ERRROR_CODE: u8 = 70u8;
const PARSE_ERROR_CODE: u8 = 65u8;

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
        "evaluate" => evaluate(&contents(&filename)),
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
    let mut exit_code = ExitCode::SUCCESS;
    for token in tokenize_string(s) {
        match token.typ {
            token::TokenType::Unknown(_) => {
                eprintln!("{}", token);
                exit_code = ExitCode::from(PARSE_ERROR_CODE);
            }
            _ => println!("{}", token),
        }
    }
    exit_code
}

fn parse_with_code(s: &str) -> (parser::Expression, u8) {
    let tokens: Vec<Token> = tokenize_string(s);
    let mut exit_code = 0;
    let mut parser = parser::Parser::new(tokens);
    let expr = parser.parse();
    match expr {
        parser::Expression::Invalid(_) => exit_code = PARSE_ERROR_CODE,
        _ => (),
    }
    (expr, exit_code)
}

fn parse(s: &str) -> ExitCode {
    let (expr, code) = parse_with_code(s);

    match expr {
        parser::Expression::Invalid(ref d) => {
            eprint!("{}", d);
        }
        ref valid => println!("{}", valid),
    }
    ExitCode::from(code)
}

fn evaluate_with_code(s: &str) -> (Result<EvalResult, EvalError>, u8) {
    let (expr, code) = parse_with_code(s);
    let ev = evaluator::Evaluator {};
    let result = ev.eval(expr);
    match result {
        Ok(res) => (Ok(res), code),
        Err(runtime_error) => (Err(runtime_error), RUNTIME_ERRROR_CODE)
    }

}

fn evaluate(s: &str) -> ExitCode {
    let (result, code) = evaluate_with_code(s);
    match result {
        Ok(res) => println!("{}", res),
        Err(e) => eprint!("{}", e),
    }
    ExitCode::from(code)
}
