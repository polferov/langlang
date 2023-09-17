use std::arch::x86_64::__cpuid;

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Semicolon,
    Integer(i32),
    Newline,
    StringLiteral(String),
    Space,
    Identifier(String),
    RoundOpen,
    RoundClose,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: TokenValue,
    pub span: Span,
}

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct CharAt {
    c: char,
    position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn take(&mut self) -> Option<CharAt> {
        self.position += 1;
        let c = self.input.chars().nth(self.position - 1)?;
        let char_at = CharAt {
            c,
            position: Position {
                position: self.position,
                line: self.line,
                column: self.column,
            },
        };

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(char_at)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn read_string_literal(&mut self) -> Token {
        let open_quote = self.take().unwrap();
        if open_quote.c != '"'{
            panic!("Expected an opening quote");
        }

        let mut value = String::new();
        let mut close_quote = None;
        while let Some(c) = self.take() {
            if c.c == '"' {
                close_quote = Some(c);
                break;
            }
            value.push(c.c);
        }

        if close_quote.is_none() {
            panic!("Expected a closing quote");
        }

        let span = Span {
            start: open_quote.position,
            end: close_quote.unwrap().position,
        };
        Token { value: TokenValue::StringLiteral(value), span }
    }

    fn single_char_token_value(c: char) -> Option<TokenValue> {
        match c {
            ';' => Some(TokenValue::Semicolon),
            '\n' => Some(TokenValue::Newline),
            ' ' => Some(TokenValue::Space),
            '(' => Some(TokenValue::RoundOpen),
            ')' => Some(TokenValue::RoundClose),
            _ => None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let c = self.peek()?;

        // handle single character tokens
        if let Some(token_value) = Self::single_char_token_value(c) {
            let pos = self.take().unwrap().position;
            return Some(Token { value: token_value, span: Span { start: pos.clone(), end: pos } });
        }

        // handle string literals
        if c == '"' {
            return Some(self.read_string_literal());
        }

        // accumulate characters into a string
        let mut value = String::new();
        let mut current = self.take().unwrap();
        let start_pos = current.position.clone();
        value.push(current.c);
        while let Some(c) = self.peek() {
            if Self::single_char_token_value(c).is_some() || c == '"' {
                break;
            }
            current = self.take().unwrap();
            value.push(current.c);
        }
        let end_pos = current.position.clone();

        let span = Span {
            start: start_pos,
            end: end_pos,
        };


        // handle integer tokens
        if let Ok(i) = value.parse::<i32>() {
            return Some(Token { value: TokenValue::Integer(i), span });
        }

        // handle identifiers
        if value.chars().all(|c| c.is_alphanumeric()) {
            return Some(Token { value: TokenValue::Identifier(value), span });
        }

        panic!("Unexpected token: {}", value);
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}