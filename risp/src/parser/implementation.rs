use crate::lexer::Token;
use crate::evaluator::SysCallWrapper;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    String(String),
    Lambda(Vec<String>, Vec<Object>),
    // TODO: Evaluate whether it is really necessary to segregate
    // functions and lambdas in the parser
    Function(Vec<String>, Vec<Object>),
    SysCall(SysCallWrapper),
    List(Vec<Object>),
}

impl Object {
    pub fn from_tokens(tokens: Vec<Token>) -> Result<Object, ParserError> {
        let mut rev_tokens = tokens.into_iter().rev().collect::<Vec<_>>();
        let parsed_list = Self::parse_list(&mut rev_tokens)?;
        Ok(parsed_list)
    }

    fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParserError> {
        let mut parenthesis_counter = ParenthesisCounter::new();
        let mut stack: Vec<Vec<Object>> = vec![vec![]];

        while let Some(token) = tokens.pop() {
            match token {
                Token::LParen(c) => {
                    parenthesis_counter.compute(Token::LParen(c.clone()))?;
                    // Create a new list and push to the stack
                    stack.push(vec![]);
                }
                Token::RParen(c) => {
                    parenthesis_counter.compute(Token::RParen(c.clone()))?;
                    // Pop the completed list
                    if let Some(completed_list) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            last.push(Object::List(completed_list));
                        } else {
                            return Ok(Object::List(completed_list));
                        }
                    } else {
                        // Unreachable code
                        return Ok(Object::Void);
                    }
                }
                Token::Integer(n) => {
                    if let Some(last) = stack.last_mut() {
                        last.push(Object::Integer(n.content));
                    }
                }
                Token::Symbol(s) => {
                    let content = s.content.clone();
                    if
                        (content.chars().next() == Some('"') &&
                            content.chars().last() == Some('"')) ||
                        (content.chars().next() == Some('\'') &&
                            content.chars().last() == Some('\''))
                    {
                        if let Some(last) = stack.last_mut() {
                            last.push(Object::String(content[1..content.len() - 1].to_string()));
                        }
                    } else {
                        if let Some(last) = stack.last_mut() {
                            last.push(Object::Symbol(content));
                        }
                    }
                }
            }
        }

        if !parenthesis_counter.is_balanced() {
            let (ch, line) = parenthesis_counter.last_ch_and_line();
            return Err(ParserError {
                err: "Unmatched opening parenthesis".to_string(),
                ch: ch,
                line: line,
            });
        }

        let mut final_list = stack.pop();
        match final_list {
            Some(ref mut list) => {
                if list.len() == 1 {
                    return Ok(list[0].clone());
                } else {
                    Ok(Object::Void)
                }
            }
            None => Ok(Object::Void),
        }
    }
}

struct ParenthesisCounter {
    parens: Vec<Token>,
}

impl ParenthesisCounter {
    pub fn new() -> Self {
        ParenthesisCounter { parens: vec![] }
    }

    pub fn compute(&mut self, token: Token) -> Result<(), ParserError> {
        match token {
            Token::LParen(c) => {
                self.parens.push(Token::LParen(c));
            }
            Token::RParen(c) => {
                if self.parens.is_empty() {
                    return Err(ParserError {
                        err: "Unmatched closing parenthesis".to_string(),
                        ch: c.ch,
                        line: c.line,
                    });
                }
                self.parens.pop();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn last_ch_and_line(&self) -> (usize, usize) {
        match self.parens.last() {
            Some(Token::LParen(c)) => (c.ch, c.line),
            Some(_) => (0, 0), // Unreachable code
            None => (0, 0),
        }
    }

    pub fn is_balanced(&self) -> bool {
        self.parens.is_empty()
    }
}

#[derive(Debug)]
pub struct ParserError {
    err: String,
    ch: usize,
    line: usize,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{ch} {err}", line = self.line, ch = self.ch, err = self.err)
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Integer(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::List(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
            _ => Ok(()),
        }
    }
}
