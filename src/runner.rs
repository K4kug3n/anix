use std::fs;
use std::io;
use std::io::Write;

use crate::expr::print_ast;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::parser::Parser;
use crate::scanner::{Scanner, ScannerError};
use crate::token::Token;

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, pos: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, pos, message);
}

fn runtime_error(error: &RuntimeError) {
    match error {
        RuntimeError::TypeError { message } => println!("Runtime error: {}", message),
        RuntimeError::ParserError => println!("Runtime error: parser error detected."),
    }
}

fn run(content: &String) -> bool {
    let mut scanner = Scanner::new(content);

    let scanned_results: Vec<Result<Token, ScannerError>> = scanner.scan_tokens();

    let mut tokens = Vec::new();
    let mut scanner_errors = Vec::new();

    for result in scanned_results {
        match result {
            Ok(token) => tokens.push(token),
            Err(err) => scanner_errors.push(err),
        }
    }

    for err in &scanner_errors {
        error(err.line, &err.message);
    }

    if !scanner_errors.is_empty() {
        return false;
    }

    for t in &tokens {
        println!("{}", t)
    }

    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    println!("{}", print_ast(&expr));

    let interpreter = Interpreter::new();
    let value = interpreter.evaluate(&expr);

    match value {
        Ok(value) => println!("{}", value),
        Err(e) => runtime_error(&e),
    }

    true
}

pub fn run_file(file: &String) -> bool {
    let content = fs::read_to_string(file).expect("Error: Could not read the file {}");

    run(&content)
}

pub fn run_prompt() -> bool {
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut line)
            .expect("Error: Could not read the line");

        if line == "\n" {
            break;
        }

        run(&line);

        line.clear();
    }

    true
}
