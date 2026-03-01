use std::fs;
use std::io;
use std::io::Write;

use crate::scanner::Scanner;

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, pos: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, pos, message);
}

fn run(content: &String) -> bool {
    let mut scanner = Scanner::new(content);
    let tokens = scanner.scan_tokens();

    let mut has_error = false;
    for res in tokens {
        match res {
            Ok(t) => println!("{}", t),
            Err(e) => {
                error(e.line, &e.message);
                has_error = true;
            }
        }
    }

    !has_error
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
