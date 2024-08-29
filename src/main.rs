use std::env;
use std::fs;
use std::io::{self, Write};
mod lexer;
mod token;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });
            tokenize(&file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(s: &str) {
    let lexer = lexer::Lexer::new(s);
    for token in lexer {
        match token {
            token::Token::Unknown(c) => eprintln!("[line 1] Error: Unexpected character: {}", c),
            _ => println!("{}", token),
        }
    }
}
