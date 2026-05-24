use std::fs;
use std::io;
use std::io::Write;

use crate::scanner::Scanner;
use crate::scanner::ScannerError;
use crate::token::Token;

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, pos: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, pos, message);
}

fn run(content: &String) -> bool {
    let mut scanner = Scanner::new(content);

    let scanned_results: Vec<Result<Token, ScannerError>> = scanner.scan_tokens();

    let mut valid_tokens = Vec::new();
    let mut scanner_errors = Vec::new();

    for result in scanned_results {
        match result {
            Ok(token) => valid_tokens.push(token),
            Err(err) => scanner_errors.push(err),
        }
    }

    for err in &scanner_errors {
        error(err.line, &err.message);
    }

    if !scanner_errors.is_empty() {
        return false;
    }

    for t in valid_tokens {
        println!("{}", t)
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
