use std::fs;
use std::io;
use std::io::Write;

use crate::scanner::Scanner;

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { had_error: false }
    }

    pub fn run_file(&mut self, file: &String) -> bool {
        let content = fs::read_to_string(file).expect("Error: Could not read the file {}");

        self.run(&content);

        !self.had_error
    }

    pub fn run_prompt(&mut self) -> bool {
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

            self.run(&line);
            self.had_error = false;

            line.clear();
        }

        true
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
        self.had_error = true;
    }

    fn report(&self, line: usize, pos: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, pos, message);
    }

    fn run(&mut self, content: &String) {
        let mut scanner = Scanner::new(content);
        let tokens = scanner.scan_tokens();

        for res in tokens {
            match res {
                Ok(t) => println!("{}", t),
                Err(e) => self.error(e.line, &e.message),
            }
        }
    }
}
