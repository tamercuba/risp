use std::fmt::Display;

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
        let parsed_list = Self::_parse_list(&mut rev_tokens)?;
        return Ok(parsed_list);
    }

    fn _parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParserError> {
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
                Token::Symbol(s) => {
                    if
                        (s.chars().next() == Some('"') && s.chars().last() == Some('"')) ||
                        (s.chars().next() == Some('\'') && s.chars().last() == Some('\''))
                    {
                        list.push(Object::String(s.as_str()[1..s.len() - 1].to_string()));
                    } else {
                        list.push(Object::Symbol(s));
                    }
                }
                Token::LParen => {
                    tokens.push(Token::LParen);
                    let sub_list = Self::_parse_list(tokens)?;
                    list.push(sub_list);
                }
                Token::RParen => {
                    return Ok(Object::List(list));
                }
            }
        }

        return Ok(Object::List(list));
    }
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
