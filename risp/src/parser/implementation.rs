use std::fmt::Display;

use crate::lexer::{ Content, Token };
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
        let parsed_list = Self::_parse_list(&mut rev_tokens, None)?;
        return Ok(parsed_list);
    }

    fn _parse_list(tokens: &mut Vec<Token>, pb: Option<i32>) -> Result<Object, ParserError> {
        let token_result = tokens.pop();
        if token_result.is_none() {
            return Err(ParserError {
                err: format!("Did not find enough tokens"),
                ch: 0,
                line: 0,
            });
        }

        let token = token_result.unwrap();
        if token != Token::LParen(Content::new((), 0, 0)) {
            return Err(ParserError {
                err: format!("Invalid token found: {:?}, expect (", token),
                ch: 1,
                line: 0,
            });
        }

        let mut parenthesis_balance = match pb {
            Some(pb) => pb,
            None => 1,
        };
        let mut list: Vec<Object> = vec![];

        while !tokens.is_empty() {
            let token = tokens.pop();
            if token == None {
                return Err(ParserError {
                    err: format!("Did not find enough tokens"),
                    ch: 0,
                    line: 0,
                });
            }
            let t = token.unwrap();

            match t {
                Token::Integer(n) => list.push(Object::Integer(n.content)),
                Token::Symbol(s) => {
                    let content = s.content.clone();
                    if
                        (content.chars().next() == Some('"') &&
                            content.chars().last() == Some('"')) ||
                        (content.chars().next() == Some('\'') &&
                            content.chars().last() == Some('\''))
                    {
                        list.push(
                            Object::String(content.as_str()[1..content.len() - 1].to_string())
                        );
                    } else {
                        list.push(Object::Symbol(content));
                    }
                }
                Token::LParen(c) => {
                    tokens.push(Token::LParen(c.clone()));
                    parenthesis_balance += 1;
                    let sub_list = Self::_parse_list(tokens, Some(parenthesis_balance))?;
                    list.push(sub_list);
                }
                Token::RParen(c) => {
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
    ch: usize,
    line: usize,
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{ch} {err}", line = self.line, ch = self.ch, err = self.err)
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
