mod env;
mod evaluator;
mod lexer;
mod parser;

use std::{ cell::RefCell, rc::Rc };
use evaluator::Evaluator;
use linefeed::{ Interface, ReadResult };
use parser::Object;

const PROMPT: &str = "risp> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let env = Rc::new(RefCell::new(env::Env::new()));
    let mut evaluator = Evaluator::new(env);

    reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }

        let val = evaluator.eval(input.as_ref())?;
        match val {
            Object::Void => {}
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            }
            _ => println!("{}", val),
        }
    }

    println!("Good bye");
    Ok(())
}
