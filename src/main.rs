use std::env;
use std::io::{stderr, stdout};
use std::process::ExitCode;
mod environment;
mod evaluator;
mod lexer;
mod parser;
pub mod tests;
mod token;
mod utils;
use evaluator::EvalError;
use evaluator::StatementEvalResult;
use evaluator::StatementResult;
use lexer::Lexer;
use parser::{Decl, Stmt};
use token::Token;
use utils::contents;
const RUNTIME_ERRROR_CODE: u8 = 70u8;
const PARSE_ERROR_CODE: u8 = 65u8;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprint!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(&contents(filename)),
        "parse" => parse(&contents(filename)),
        "evaluate" => evaluate(&contents(filename)),
        "run" => run(&contents(filename)),
        _ => {
            eprint!("Unknown command: {}", command);
            ExitCode::FAILURE
        }
    }
}

fn tokenize(s: &str) -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;
    for token in Lexer::new(s).tokens() {
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
fn parse_with_code_and_errstmt(s: &str) -> (parser::Program, u8, Option<Decl>) {
    let tokens: Vec<Token> = Lexer::new(s).tokens();

    let mut parser = parser::Parser::new(tokens);
    let prog = parser.parse();

    let opt_err = prog.syntax_errors();
    (
        prog,
        if opt_err.is_none() {
            0
        } else {
            PARSE_ERROR_CODE
        },
        opt_err,
    )
}
fn parse_with_code(s: &str) -> (parser::Program, u8) {
    let (prog, code, _) = parse_with_code_and_errstmt(s);
    (prog, code)
}

fn print_stmt(s: &Stmt) {
    match s {
        Stmt::Expression(e) => print_expr(e),
        Stmt::Print(e) => print_expr(e),
        Stmt::Block(vec) => print_decls(vec),
        Stmt::Invalid(s) => println!("{}", s)
    }
}
fn print_decl(d: &Decl) {
    match d {
        Decl::VarDecl(token, expression) => match expression {
            Some(e) => println!("var {} = {};", token.s, e),
            None => println!("var {};", token.s),
        },
        Decl::Statement(stmt) => print_stmt(stmt),
    }
}
fn print_stmts(v: &Vec<Stmt>) {
    v.iter().for_each(print_stmt);
}

fn print_decls(v: &Vec<Decl>) {
    v.iter().for_each(print_decl);
}
// TODO Stmt should be a struct with expresion and type
fn parse(s: &str) -> ExitCode {
    let (expr, code, _errstmt) = parse_with_code_and_errstmt(s);

    for d in expr.declarations {
        match d {
            Decl::Statement(s) => print_stmt(&s),
            Decl::VarDecl(t, opt_e) => {
                print!("[ token: [{}], expr: ", t);
                match opt_e {
                    Some(e) => print_expr(&e),
                    None => print!("(only decl.)"),
                }
                print!("]");
            }
        }
    }

    ExitCode::from(code)
}

fn print_expr(e: &parser::Expression) {
    match e {
        parser::Expression::Invalid(ref d) => {
            eprint!("{}", d);
        }
        ref valid => println!("{}", valid),
    }
}

fn evaluate_with_code(s: &str) -> (Vec<StatementEvalResult>, Option<EvalError>, u8) {
    let (prog, code) = parse_with_code(s);
    let mut ev = evaluator::Evaluator::new();

    let mut res = Vec::new();
    let mut opt_err = None;
    let resvec: Vec<StatementResult> = ev.eval(prog);
    for sr in resvec {
        match sr {
            Ok(ser) => res.push(ser),
            Err(ever) => {
                opt_err = Some(ever);
                break;
            }
        }
    }

    (
        res,
        opt_err.clone(),
        if (code == 0) && opt_err.is_some() {
            RUNTIME_ERRROR_CODE
        } else {
            code
        },
    )
}

fn evaluate(s: &str) -> ExitCode {
    let (result, opt_err, code) = evaluate_with_code(s);
    for r in result {
        match r {
            StatementEvalResult::ExpressionStatementResult(er) => println!("{}", er),
            StatementEvalResult::PrintStatementResult(er) => println!("{}", er),
            StatementEvalResult::BlockResult(vec) => vec.iter().for_each(|ser| {println!("{:?}", ser);}),
        }
    }
    if let Some(err) = opt_err {
        eprint!("{}", err);
    }
    ExitCode::from(code)
}

fn print_resw(w: &mut dyn std::io::Write, res: Vec<StatementEvalResult>) {
    for r in res {
        match r {
            StatementEvalResult::PrintStatementResult(er) => {
                let _ = w.write_fmt(format_args!("{}\n", er));
            }
            StatementEvalResult::ExpressionStatementResult(_er) => {}
            StatementEvalResult::BlockResult(vec) => print_resw(w, vec),
        }
    }
}

// fn print_res(res: Vec<StatementEvalResult>) {
//     print_resw(&mut stdout(), res)
// }

fn runw<W: std::io::Write, E: std::io::Write>(out: &mut W, err: &mut E, s: &str) -> ExitCode {
    let (result, opt_err, code) = evaluate_with_code(s);
    print_resw(out, result);
    if let Some(e) = opt_err {
        let _ = err.write_fmt(format_args!("{}", e));
    }
    ExitCode::from(code)
}
fn run(s: &str) -> ExitCode {
    runw(&mut stdout(), &mut stderr(), s)
}
// fn run(s: &str) -> ExitCode {
//     let (result, opt_err, code) = evaluate_with_code(s);
//     print_res(result);
//     if let Some(e)  = opt_err {
//         eprintln!("{}", e);
//     }
//     ExitCode::from(code)
// }
