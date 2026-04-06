#[cfg(test)]
mod test_lexer;
#[cfg(test)]
mod test_token;
mod token;
pub use token::{Content, Span, Token};

#[derive(Default)]
pub struct Lexer {
    tokens: Vec<Token>,
    buffer: String,
    buffer_lo: u32,
    in_comment: bool,
    in_string: bool,
}

type DelimiterVariant = fn(Content<()>) -> Token;

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
            if lexer.in_string && ch != '"' {
                lexer.push_to_buffer(ch, ch_offset);
                continue;
            }

            match ch {
                '(' => {
                    lexer.push_delimiter(Token::LParen, ch_offset);
                }
                ')' => {
                    lexer.push_delimiter(Token::RParen, ch_offset);
                }
                '{' => {
                    lexer.push_delimiter(Token::LBrace, ch_offset);
                }
                '}' => {
                    lexer.push_delimiter(Token::RBrace, ch_offset);
                }
                '[' => {
                    lexer.push_delimiter(Token::LBracket, ch_offset);
                }
                ']' => {
                    lexer.push_delimiter(Token::RBracket, ch_offset);
                }
                '#' => {
                    lexer.push_delimiter(Token::Hash, ch_offset);
                }
                '\'' => {
                    lexer.push_delimiter(Token::Quote, ch_offset);
                }
                ' ' | '\t' | '\n' | '\r' => {
                    lexer.flush_buffer(ch_offset);
                }
                ';' => {
                    lexer.flush_buffer(ch_offset);
                    lexer.in_comment = true;
                }
                '"' => {
                    if lexer.in_string {
                        lexer.flush_buffer(ch_offset + 1);
                        lexer.in_string = false;
                    } else {
                        lexer.push_to_buffer(ch, ch_offset);
                        lexer.in_string = true;
                    }
                }
                _ => {
                    lexer.push_to_buffer(ch, ch_offset);
                }
            }
        }
        lexer.flush_buffer(program.len());
        lexer.tokens
    }

    fn push_delimiter(&mut self, variant: DelimiterVariant, ch_offset: usize) {
        self.flush_buffer(ch_offset);
        self.push_token(variant(Content::new((), Span::at(ch_offset))));
    }

    fn flush_buffer(&mut self, hi: usize) {
        if self.buffer.is_empty() {
            return;
        }
        let span = Span {
            lo: self.buffer_lo,
            hi: hi as u32,
        };
        let token = self.classify_buffer(span);

        self.push_token(token);
        self.buffer.clear();
    }

    fn classify_buffer(&self, span: Span) -> Token {
        None.or_else(|| {
            self.in_string
                .then(|| Token::String(Content::new(self.buffer[1..].to_string(), span)))
        })
        .or_else(|| {
            self.buffer
                .starts_with(':')
                .then(|| Token::Keyword(Content::new(self.buffer[1..].to_string(), span)))
        })
        .or_else(|| {
            self.buffer
                .parse::<i64>()
                .ok()
                .map(|v| Token::Long(Content::new(v, span)))
        })
        .or_else(|| {
            self.buffer
                .parse::<f64>()
                .ok()
                .map(|v| Token::Double(Content::new(v, span)))
        })
        .unwrap_or_else(|| Token::Symbol(Content::new(self.buffer.clone(), span)))
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
