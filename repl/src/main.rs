use lib::Interpreter;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Context, Editor, Helper};

const PROMPT: &str = "risp> ";
const HISTORY_FILE: &str = ".risp_history";

struct RispHelper {
    completions: Vec<String>,
}

impl Helper for RispHelper {}
impl Highlighter for RispHelper {}
impl Hinter for RispHelper {
    type Hint = String;
}
impl Validator for RispHelper {}

impl Completer for RispHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let word_start = line[..pos]
            .rfind(['(', ' ', '\t'])
            .map(|i| i + 1)
            .unwrap_or(0);

        let word = &line[word_start..pos];

        let matches = self
            .completions
            .iter()
            .filter(|s| s.starts_with(word))
            .map(|s| Pair {
                display: s.clone(),
                replacement: s.clone(),
            })
            .collect();

        Ok((word_start, matches))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut interpreter = Interpreter::new(true);

    let helper = RispHelper {
        completions: interpreter.completions(),
    };

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(helper));
    let _ = rl.load_history(HISTORY_FILE);

    loop {
        match rl.readline(PROMPT) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "exit" {
                    break;
                }

                rl.add_history_entry(input)?;

                match interpreter.run(input) {
                    Ok(v) => println!("{v}"),
                    Err(e) => println!("{e}"),
                }

                if let Some(helper) = rl.helper_mut() {
                    helper.completions = interpreter.completions();
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("error: {e}");
                break;
            }
        }
    }

    let _ = rl.save_history(HISTORY_FILE);
    Ok(())
}
