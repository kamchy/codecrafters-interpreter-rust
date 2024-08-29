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

fn tokenize(s: &str) -> ExitCode {
    let lexer = lexer::Lexer::new(s);
    let mut exit_code = ExitCode::SUCCESS;
    for token in lexer {
        match token {
            token::Token::Unknown(c) => {
                eprintln!("[line 1] Error: Unexpected character: {}", c);
                exit_code = ExitCode::from(65);
            }
            _ => println!("{}", token),
        }
    }
    exit_code
}
