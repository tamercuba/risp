use risp_eval::Evaluator;

use linefeed::{ Interface, ReadResult };

const PROMPT: &str = "risp> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let mut evaluator = Evaluator::new();

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
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}
