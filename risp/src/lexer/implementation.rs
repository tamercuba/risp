use super::token::{Content, Span, Token};

pub struct Lexer {
    tokens: Vec<Token>,
    buffer: String,
    buffer_lo: u32,
    in_comment: bool,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            tokens: vec![],
            buffer: String::new(),
            buffer_lo: 0,
            in_comment: false,
        }
    }
}

impl Lexer {
    pub fn tokenize(program: &str) -> Vec<Token> {
        let mut lexer = Lexer::default();

        for (ch_offset, ch) in program.char_indices() {
            if lexer.in_comment {
                if ch == '\n' || ch == '\r' {
                    lexer.in_comment = false;
                }
                continue;
            }

            match ch {
                '(' => {
                    lexer.flush_buffer(ch_offset);
                    lexer.push_token(Token::LParen(Content::new((), Span::at(ch_offset))));
                }
                ')' => {
                    lexer.flush_buffer(ch_offset);
                    lexer.push_token(Token::RParen(Content::new((), Span::at(ch_offset))));
                }
                ' ' | '\t' | '\n' | '\r' => {
                    lexer.flush_buffer(ch_offset);
                }
                ';' => {
                    lexer.flush_buffer(ch_offset);
                    lexer.in_comment = true;
                }
                _ => {
                    lexer.push_to_buffer(ch, ch_offset);
                }
            }
        }

        lexer.flush_buffer(program.len());
        lexer.tokens
    }

    fn flush_buffer(&mut self, hi: usize) {
        if self.buffer.is_empty() {
            return;
        }
        let span = Span {
            lo: self.buffer_lo,
            hi: hi as u32,
        };
        let token = match self.buffer.parse::<i64>() {
            Ok(value) => Token::Integer(Content::new(value, span)),
            Err(_) => Token::Symbol(Content::new(self.buffer.clone(), span)),
        };
        self.push_token(token);
        self.buffer.clear();
    }

    fn push_to_buffer(&mut self, ch: char, offset: usize) {
        if self.buffer.is_empty() {
            self.buffer_lo = offset as u32;
        }
        self.buffer.push(ch);
    }

    fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
