use std::fmt::{ Display, Debug };

#[derive(Clone)]
pub struct Content<T> {
    pub content: T,
    pub ch: usize,
    pub line: usize,
}

impl<T> Display for Content<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{line}:{ch} {content}",
            line = self.line,
            ch = self.ch.clone(),
            content = format!("{:?}", self.content)
        )
    }
}

impl<T> Debug for Content<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{line}:{ch} {content}",
            line = self.line,
            ch = self.ch.clone(),
            content = format!("{:?}", self.content)
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

#[derive(PartialEq, Clone)]
pub enum Token {
    Integer(Content<i64>),
    Symbol(Content<String>),
    LParen(Content<()>),
    RParen(Content<()>),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(c) =>
                write!(f, "{line}:{ch} Int({value})", line = c.line, ch = c.ch, value = c.content),
            Token::Symbol(c) =>
                write!(
                    f,
                    "{line}:{ch} Symbol({value})",
                    line = c.line,
                    ch = c.ch,
                    value = c.content
                ),
            Token::LParen(c) => write!(f, "{line}:{ch} LParen", line = c.line, ch = c.ch),
            Token::RParen(c) => write!(f, "{line}:{ch} RParen", line = c.line, ch = c.ch),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(c) =>
                write!(f, "{line}:{ch} Int({value})", line = c.line, ch = c.ch, value = c.content),
            Token::Symbol(c) =>
                write!(
                    f,
                    "{line}:{ch} Symbol({value})",
                    line = c.line,
                    ch = c.ch,
                    value = c.content
                ),
            Token::LParen(c) => write!(f, "{line}:{ch} LParen", line = c.line, ch = c.ch),
            Token::RParen(c) => write!(f, "{line}:{ch} RParen", line = c.line, ch = c.ch),
        }
    }
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
        let buffer_size = buffer.len();
        let current_pos = ch.clone() - buffer_size + 1;
        match buffer.is_empty() {
            true => (),
            false =>
                match buffer.parse::<i64>() {
                    Ok(value) =>
                        tokens.push(Token::Integer(Content::new(value, current_pos, line.clone()))),
                    Err(_) =>
                        tokens.push(
                            Token::Symbol(
                                Content::new(String::from(buffer), current_pos, line.clone())
                            )
                        ),
                }
        }
    }
}
