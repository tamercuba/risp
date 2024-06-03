use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl Token {
    pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
        let parsed_program = program.replace("(", " ( ").replace(")", " ) ");
        let words = parsed_program.split_whitespace();
        let mut tokens: Vec<Token> = vec![];
        let mut parentheses_count = 0;
        for word in words {
            match word {
                "(" => {
                    parentheses_count += 1;
                    tokens.push(Token::LParen);
                    continue;
                }
                ")" => {
                    parentheses_count -= 1;
                    tokens.push(Token::RParen);
                    continue;
                }
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
        if parentheses_count == 0 {
            Ok(tokens)
        } else if parentheses_count < 0 {
            Err(TokenError { ch: ')' })
        } else {
            Err(TokenError { ch: '(' })
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ch {
            '(' => write!(f, "Unmatched closing parenthesis"),
            ')' => write!(f, "Unmatched opening parenthesis"),
            _ => write!(f, "Unexpected character: {}", self.ch),
        }
    }
}
