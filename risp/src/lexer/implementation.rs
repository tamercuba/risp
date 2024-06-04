use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Content<T> {
    pub content: T,
    pub ch: usize,
    pub line: usize,
}

impl<T> Display for Content<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{line}:{ch} {content}",
            line = self.line,
            ch = self.ch.clone(),
            content = self.content
        )
    }
}

impl<T> PartialEq for Content<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl<T> Eq for Content<T> where T: Eq {}

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

        let lines = program.lines();
        for file_line in lines {
            let mut new_tokens = Self::parse_line(&file_line, line);
            tokens.append(&mut new_tokens);
            line += 1;
        }

        return tokens;
    }

    fn parse_line(program: &str, line: usize) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut ch: usize = 0;
        let mut chars = program.chars().peekable();
        let mut current_buffer = String::new();

        while let Some(&current) = chars.peek() {
            match current {
                '(' => {
                    if !current_buffer.is_empty() {
                        Self::parse_buffer(&current_buffer.clone(), &mut tokens, &mut ch, line);
                        current_buffer.clear();
                    }
                    ch += 1;
                    tokens.push(Token::LParen(Content::new((), ch.clone(), line.clone())));
                    chars.next();
                }
                ')' => {
                    if !current_buffer.is_empty() {
                        Self::parse_buffer(&current_buffer.clone(), &mut tokens, &mut ch, line);
                        current_buffer.clear();
                    }
                    ch += 1;
                    tokens.push(Token::RParen(Content::new((), ch.clone(), line.clone())));
                    chars.next();
                }
                ' ' => {
                    if !current_buffer.is_empty() {
                        let value = current_buffer.clone();
                        Self::parse_buffer(&value, &mut tokens, &mut ch, line);
                        current_buffer.clear();
                    }
                    ch += 1;
                    chars.next();
                }
                _ => {
                    current_buffer.push(current);
                    ch += 1;
                    chars.next();
                }
            }
        }
        if !current_buffer.is_empty() {
            let value = current_buffer.clone();
            Self::parse_buffer(&value, &mut tokens, &mut ch, line);
        }
        tokens
    }

    fn parse_buffer(buffer: &str, tokens: &mut Vec<Token>, ch: &mut usize, line: usize) {
        match buffer.is_empty() {
            true => (),
            false =>
                match buffer.parse::<i64>() {
                    Ok(value) =>
                        tokens.push(Token::Integer(Content::new(value, ch.clone(), line.clone()))),
                    Err(_) =>
                        tokens.push(
                            Token::Symbol(
                                Content::new(String::from(buffer), ch.clone(), line.clone())
                            )
                        ),
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
