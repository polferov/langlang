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
}

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            position: 0,
        }
    }

    fn take(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.position);
        self.position += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn read_string_literal(&mut self) -> Token {
        let mut value = String::new();
        while let Some(c) = self.take() {
            if c == '"' {
                break;
            }
            value.push(c);
        }
        Token { value: TokenValue::StringLiteral(value) }
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
        let c = self.take()?;

        // handle single character tokens
        if let Some(token_value) = Self::single_char_token_value(c) {
            return Some(Token { value: token_value });
        }

        // handle string literals
        if c == '"' {
            return Some(self.read_string_literal());
        }

        // accumulate characters into a string
        let mut value = String::new();
        value.push(c);
        while let Some(c) = self.peek() {
            if Self::single_char_token_value(c).is_some() {
                break;
            }
            value.push(self.take()?);
        }


        // handle integer tokens
        if let Ok(i) = value.parse::<i32>() {
            return Some(Token { value: TokenValue::Integer(i) });
        }

        // handle identifiers
        if value.chars().all(|c| c.is_alphanumeric()) {
            return Some(Token { value: TokenValue::Identifier(value) });
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