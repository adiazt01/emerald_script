use crate::lexer::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current: usize = 0;
        let mut start: usize = 0;
        let mut line: usize = 1;

        Scanner {
            source,
            tokens,
            current,
            start,
            line,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let start = self.current;
            let line = self.line;
        }

        self.tokens.push(Token { token_type: super::token_types::TokenTypes::Eof, lexeme: "".to_string(), literal: None, line: self.line });

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let character = self.advance();

        match character {
            '(' => self.add_token(super::token_types::TokenTypes::LeftParen),
            ')' => self.add_token(super::token_types::TokenTypes::RightParen),
            '{' => self.add_token(super::token_types::TokenTypes::LeftBrace),
            '}' => self.add_token(super::token_types::TokenTypes::RightBrace),
            ',' => self.add_token(super::token_types::TokenTypes::Comma),
            '.' => self.add_token(super::token_types::TokenTypes::Dot),
            '-' => self.add_token(super::token_types::TokenTypes::Minus),
            '+' => self.add_token(super::token_types::TokenTypes::Plus),
            ';' => self.add_token(super::token_types::TokenTypes::Semicolon),
            '*' => self.add_token(super::token_types::TokenTypes::Star),
            _ => {
                // Handle other characters, keywords, identifiers, etc.
                // This is a placeholder for now.
                println!("Unexpected character: {}", character);
            }
        }
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    fn add_token(&mut self, token_type: super::token_types::TokenTypes) {
        let lexeme = self.source[self.start..self.current].to_string();

        let token = Token::new(token_type, lexeme, None, self.line);
        
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}