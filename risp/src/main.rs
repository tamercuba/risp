use lib::Interpreter;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: risp <file>");
        return ExitCode::FAILURE;
    }

    let path = &args[1];
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: could not read '{path}': {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut interpreter = Interpreter::new(true);
    match interpreter.run(&source) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
