use risp_eval::{ evaluator::Evaluator, parser::Object, env::Env };

use std::{ cell::RefCell, rc::Rc };
use linefeed::{ Interface, ReadResult };

const PROMPT: &str = "risp> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let env = Rc::new(RefCell::new(Env::new()));
    let mut evaluator = Evaluator::new(env);

    reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        match input.as_str() {
            "exit" => {
                break;
            }
            "\n" | "" => {
                continue;
            }
            _ => {}
        }

        let val = evaluator.eval(input.as_ref());
        match val {
            Ok(Object::Void) => {}
            Ok(Object::Integer(n)) => println!("{}", n),
            Ok(Object::Bool(b)) => println!("{}", b),
            Ok(Object::Symbol(s)) => println!("{}", s),
            Ok(Object::Lambda(params, body)) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            }
            Ok(_) => println!("{}", val.unwrap()),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}
