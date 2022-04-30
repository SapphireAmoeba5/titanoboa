use crate::interpreter::parser::TokenStream;
use crate::interpreter::token::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mode {
    Ascii,
    Symbol,
    Default,
}

pub struct TokenBuilder {
    line_count: usize,
    column_count: usize,
    mode: Mode,
    buffer: String,
    tokens: TokenStream,
    in_string: bool,
    escaped: bool,
    quotation_char: char,
}

impl TokenBuilder {
    pub fn new() -> Self {
        Self {
            line_count: 1,
            column_count: 1,
            mode: Mode::Default,
            buffer: String::new(),
            tokens: TokenStream::new(),
            in_string: false,
            escaped: false,
            quotation_char: ' ',
        }
    }

    pub fn push_char(&mut self, ch: char) {
        if ch == '\'' || ch == '"' || self.in_string {
            if ch == '"' {
                if !self.in_string {
                    self.dump_buffer();
                    self.quotation_char = '"';
                    self.in_string = true;
                } else if self.quotation_char == '"' && !self.escaped {
                    self.in_string = false;
                    self.quotation_char = ' ';
                }
            } else if ch == '\'' {
                if !self.in_string {
                    self.dump_buffer();
                    self.quotation_char = '\'';
                    self.in_string = true;
                } else if self.quotation_char == '\'' && !self.escaped {
                    self.in_string = false;
                    self.quotation_char = ' ';
                }
            }

            if ch == '\\' {
                self.escaped = true;
            } else {
                self.escaped = false;
            }
            self.buffer.push(ch);
        } else if ch == ' ' {
            self.dump_buffer();
            self.mode = Mode::Default;
        } else if ch == '\n' {
            self.dump_buffer();
            self.tokens.push(Token::Newline {
                line: self.line_count,
                start: self.column_count,
                end: self.column_count,
            });
            self.line_count += 1;
            self.column_count = 0;
        } else if ch.is_alphanumeric() || ch == '_' {
            if self.mode == Mode::Symbol {
                self.dump_buffer();
                self.mode = Mode::Ascii;
            } else if self.mode == Mode::Default {
                self.mode = Mode::Ascii;
            }
            self.buffer.push(ch);
        } else {
            if self.mode == Mode::Ascii {
                self.dump_buffer();
                self.mode = Mode::Symbol;
            } else if self.mode == Mode::Default {
                self.mode = Mode::Symbol;
            }
            // Special symbols that are always by themself
            if ch == '(' || ch == '{' || ch == '[' || ch == ')' || ch == '}' || ch == ']' {
                self.dump_buffer();
                self.tokens.push_str(
                    &ch.to_string(),
                    self.column_count,
                    self.column_count,
                    self.line_count,
                );
                self.mode = Mode::Default;
            } else {
                self.buffer.push(ch);
            }
        }

        self.column_count += 1;
    }

    pub fn finalize(&mut self) -> TokenStream {
        self.dump_buffer();
        self.tokens.clone()
    }
}

impl TokenBuilder {
    fn token_column_start(&self) -> usize {
        self.column_count - self.buffer.len()
    }

    fn token_column_end(&self) -> usize {
        self.column_count
    }

    fn dump_buffer(&mut self) {
        let start = self.token_column_start();
        let end = self.token_column_end();
        self.tokens
            .push_str(&self.buffer, start, end, self.line_count);
        self.buffer.clear();
    }
}
