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
        let parsed_list = Self::_parse_list(&mut rev_tokens)?;
        return Ok(parsed_list);
    }

    fn _parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParserError> {
        let mut parenthesis_count: Vec<Token> = vec![];
        let result = Self::_parse_list_and_count_parenthesis(tokens, &mut parenthesis_count);

        if result.is_err() {
            return result;
        }
        let list = result.unwrap();

        return Ok(list);
    }

    fn _parse_list_and_count_parenthesis(
        tokens: &mut Vec<Token>,
        pc: &mut Vec<Token>
    ) -> Result<Object, ParserError> {
        if tokens.is_empty() {
            return Err(ParserError {
                err: format!("Did not find enough tokens"),
                ch: 0,
                line: 0,
            });
        }

        let t = tokens.pop().unwrap();
        let statment = t != Token::LParen(Content::new((), 0, 0));
        if statment {
            return Err(ParserError {
                err: format!("Invalid token found: {:?}, expect (", t),
                ch: 1,
                line: 0,
            });
        }
        pc.push(t.clone());
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
                    let sub_list = Self::_parse_list_and_count_parenthesis(tokens, pc)?;
                    pc.push(Token::LParen(c));
                    list.push(sub_list);
                }
                Token::RParen(c) => {
                    if pc.is_empty() {
                        return Err(ParserError {
                            err: format!("Unmatched closing parenthesis"),
                            ch: c.ch,
                            line: c.line,
                        });
                    }
                    pc.pop();
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
