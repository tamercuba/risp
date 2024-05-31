use std::fmt::Display;

use crate::lexer::{ tokenize, Token };

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    // TODO: Evaluate whether it is really necessary to segregate
    // functions and lambdas in the parser
    Function(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

#[derive(Debug)]
pub struct ParserError {
    err: String,
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ParserError]: {err}", err = self.err)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Void | Object::Lambda(_, _) | Object::Function(_, _) => Ok(()),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
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
        }
    }
}
pub fn parse(program: &str) -> Result<Object, ParserError> {
    let tokens_result = tokenize(program);
    if tokens_result.is_err() {
        return Err(ParserError {
            err: format!("{}", tokens_result.err().unwrap()),
        });
    }
    let mut tokens = tokens_result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens)?;
    return Ok(parsed_list);
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParserError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParserError {
            err: format!("Did not find enough tokens"),
        });
    }

    let mut list: Vec<Object> = vec![];

    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParserError {
                err: format!("Did not find enough tokens"),
            });
        }
        let t = token.unwrap();

        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    return Ok(Object::List(list));
}
