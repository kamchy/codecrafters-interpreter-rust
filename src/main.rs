use std::env;
use std::error;
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
use parser::Stmt;
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
        "run" => run(&contents(&filename)),
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
fn parse_with_code_and_errstmt(s: &str) -> (parser::Program, u8, Option<Stmt>) {
    let tokens: Vec<Token> = tokenize_string(s);

    let mut parser = parser::Parser::new(tokens);
    let prog = parser.parse();


    let opt_err = prog.syntax_errors();
    (prog, if opt_err.is_none() { 0 } else  { PARSE_ERROR_CODE }, opt_err )

}
fn parse_with_code(s: &str) -> (parser::Program, u8) {
    let (prog, code, _) = parse_with_code_and_errstmt(s);
    (prog, code)

}

fn parse(s: &str) -> ExitCode {
    let (expr, code, errstmt) = parse_with_code_and_errstmt(s);

    for s in expr.statements {
        match s {
            Stmt::Expression(e) => print_expr(e),
            Stmt::Print(e) => print_expr(e)
        }

    }

    ExitCode::from(code)
}

fn print_expr(e: parser::Expression) {
    match e {
        parser::Expression::Invalid(ref d) => {
            eprint!("{}", d);
        }
        ref valid => println!("{}", valid),
    }
}

fn evaluate_with_code(s: &str) -> (Result<EvalResult, EvalError>, u8) {
    let (prog, code) = parse_with_code(s);
    let ev = evaluator::Evaluator {};

    let resvec = ev.eval(prog);
    let (results, errors) : ( Vec<Result<EvalResult, EvalError>>,  Vec<Result<EvalResult, EvalError>>) = resvec.into_iter().partition(|w| w.is_ok());
    if let Some(first_err) = errors.into_iter().take(1).next() {
        (first_err, RUNTIME_ERRROR_CODE)
    } else {
        if let Some(first_res) = results.into_iter().take(1).next() {
            (first_res, code)
        } else {
            panic!("unexpected")
        }
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

fn run(s: &str) -> ExitCode {
    let (_result, code) = evaluate_with_code(s);
    ExitCode::from(code)
}

