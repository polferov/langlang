#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Semicolon,
    Integer(i32),
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


    fn next_token(&mut self) -> Option<Token> {
        let c = self.take()?;
        // handle single character tokens
        match c {
            ';' => return Some(Token { value: TokenValue::Semicolon }),
            _ => (),
        };

        let mut value = String::new();
        value.push(c);
        while let Some(c) = self.peek() {
            match c {
                ';' => break,
                _ => value.push(self.take().unwrap()),
            }
        }
        // handle integer tokens
        if let Ok(i) = value.parse::<i32>() {
            return Some(Token { value: TokenValue::Integer(i) });
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