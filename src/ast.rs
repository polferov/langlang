use crate::tokenizer::{Token, TokenValue};

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

#[derive(Debug)]
pub struct FunctionCall {
    name: Identifier,
    args: Vec<Expr>,
}

#[derive(Debug)]
pub enum Statement {
    Call(FunctionCall),
}

#[derive(Debug)]
pub enum Expr {
    IntegerLiteral(i32),
    StringLiteral(String),
    Identifier(Identifier),
    Call(FunctionCall),
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}


pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let tokens = tokens.into_iter().filter(|t| !t.value.is_cosmetic()).collect();
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_n(&self, n: usize) -> Vec<&Token> {
        self.tokens.iter().skip(self.pos).take(n).collect()
    }

    fn take(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.tokens.get(self.pos - 1)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();
        while self.peek().is_some() {
            program.push(self.parse_statement());
        }
        Program { statements: program }
    }

    fn parse_statement(&mut self) -> Statement {
        let tokens = self.peek_n(2).iter().map(|t| &t.value).collect::<Vec<_>>();
        let stmt = match tokens.as_slice() {
            [TokenValue::Identifier(_), TokenValue::RoundOpen] => Statement::Call(self.parse_call()),
            _ => panic!("Unexpected tokens {:?}", tokens ),
        };

        match self.take().expect("expected semicolon").value.clone() {
            TokenValue::Semicolon => (),
            v => panic!("Expected semicolon. Got {:?}", v),
        }

        stmt
    }


    fn parse_call(&mut self) -> FunctionCall {
        let identifier = match self.take().expect("Expected identifier").value.clone() {
            TokenValue::Identifier(name) => Identifier { name },
            _ => panic!("Expected identifier"),
        };

        match self.take().expect("Expected open").value.clone() {
            TokenValue::RoundOpen => (),
            _ => panic!("Expected open"),
        }

        let mut args = Vec::new();
        while self.peek().expect("close expected").value != TokenValue::RoundClose {
            args.push(self.parse_expr());
        }
        self.take();

        FunctionCall { name: identifier, args }
    }

    fn parse_expr(&mut self) -> Expr {
        let tokens = self.peek_n(2).iter().map(|t| &t.value).collect::<Vec<_>>();
        match tokens.as_slice() {
            [TokenValue::Identifier(_), TokenValue::RoundOpen] => Expr::Call(self.parse_call()),
            [TokenValue::Integer(_), _] => Expr::IntegerLiteral(self.parse_integer_literal()),
            [TokenValue::StringLiteral(_), _] => Expr::StringLiteral(self.parse_string_literal()),
            [TokenValue::Identifier(_), _] => Expr::Identifier(self.parse_identifier()),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_integer_literal(&mut self) -> i32 {
        match self.take().expect("Expected integer").value.clone() {
            TokenValue::Integer(i) => i,
            _ => panic!("Expected integer"),
        }
    }

    fn parse_string_literal(&mut self) -> String {
        match self.take().expect("Expected string").value.clone() {
            TokenValue::StringLiteral(s) => s,
            _ => panic!("Expected string"),
        }
    }

    fn parse_identifier(&mut self) -> Identifier {
        match self.take().expect("Expected identifier").value.clone() {
            TokenValue::Identifier(name) => Identifier { name },
            _ => panic!("Expected identifier"),
        }
    }
}
