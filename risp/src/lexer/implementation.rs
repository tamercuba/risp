use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Content<T> {
    content: T,
    ch: usize,
    line: usize,
}

impl<T> Display for Content<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{ch} {content}", line = self.line, ch = self.ch, content = self.content)
    }
}

impl<T> PartialEq for Content<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl<T> Content<T> {
    pub fn new(content: T, ch: usize, line: usize) -> Self {
        Content {
            content,
            ch,
            line,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(Content<i64>),
    Symbol(Content<String>),
    LParen(Content<()>),
    RParen(Content<()>),
}

impl Token {
    pub fn tokenize(program: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut line: usize = 0;
        let mut ch: usize = 0;

        let mut current_buffer = String::new();
        let mut chars = program.chars().peekable();

        while let Some(&current) = chars.peek() {
            match current {
                '(' => {
                    tokens.push(Token::LParen(Content::new((), ch, line)));
                    ch += 1;
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RParen(Content::new((), ch, line)));
                    ch += 1;
                    chars.next();
                }
                ' ' => {
                    let value = current_buffer.clone();
                    Self::parse_buffer(&value, &mut tokens, &mut ch, &mut line);
                    current_buffer.clear();
                    ch += 1;
                    chars.next();
                }
                '\n' => {
                    let value = current_buffer.clone();
                    Self::parse_buffer(&value, &mut tokens, &mut ch, &mut line);
                    current_buffer.clear();
                    line += 1;
                    ch = 0;
                    chars.next();
                }
                '\r' => {
                    if let Some(&'\n') = chars.peek() {
                        chars.next();
                    }
                    let value = current_buffer.clone();
                    Self::parse_buffer(&value, &mut tokens, &mut ch, &mut line);
                    current_buffer.clear();
                    line += 1;
                    ch = 0;
                    chars.next();
                }
                _ => {
                    current_buffer.push(current);
                    if ch == program.len() - 1 {
                        tokens.push(Token::Symbol(Content::new(current_buffer.clone(), ch, line)));
                        break;
                    }
                }
            }
        }
        tokens
    }
    fn parse_buffer(buffer: &str, tokens: &mut Vec<Token>, ch: &mut usize, line: &mut usize) {
        match buffer.is_empty() {
            true => (),
            false =>
                match buffer.parse::<i64>() {
                    Ok(value) => tokens.push(Token::Integer(Content::new(value, *ch, *line))),
                    Err(_) =>
                        tokens.push(Token::Symbol(Content::new(buffer.to_string(), *ch, *line))),
                }
        }
    }
}

// #[derive(Debug)]
// pub struct TokenError {
//     value: String,
//     ch: usize,
//     line: usize,
// }

// impl Display for TokenError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.value.as_str() {
//             "(" =>
//                 write!(
//                     f,
//                     "{line}:{ch} Unmatched closing parenthesis",
//                     line = self.line,
//                     ch = self.ch
//                 ),
//             ")" =>
//                 write!(
//                     f,
//                     "{line}:{ch} Unmatched opening parenthesis",
//                     line = self.line,
//                     ch = self.ch
//                 ),
//             _ =>
//                 write!(
//                     f,
//                     "{line}:{ch} Unexpected character: {value}",
//                     value = self.value,
//                     line = self.line,
//                     ch = self.ch
//                 ),
//         }
//     }
// }
