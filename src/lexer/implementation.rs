use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected character: {}", self.ch)
    }
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let parsed_program = program.replace("(", " ( ").replace(")", " ) ");
    let words = parsed_program.split_whitespace();
    let mut tokens: Vec<Token> = vec![];

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let i = word.parse::<i64>();
                if i.is_ok() {
                    tokens.push(Token::Integer(i.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }
    Ok(tokens)
}
