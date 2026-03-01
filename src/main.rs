use std::env;
use std::process::ExitCode;

use anix::interpreter::Interpreter;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 2 {
        println!("Usage:");
        println!("- {} [script]", args[0]);
        println!("- {}", args[0]);

        return ExitCode::FAILURE;
    } else if args.len() == 2 {
        if interpreter.run_file(&args[1]) {
            return ExitCode::SUCCESS;
        }

        return ExitCode::FAILURE;
    }

    if interpreter.run_prompt() {
        return ExitCode::SUCCESS;
    }

    return ExitCode::FAILURE;
}
