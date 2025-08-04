use crate::{errors::error::{self, error}, lexer::{token::Token, token_types}};

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

        self.tokens.push(Token {
            token_type: super::token_types::TokenTypes::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

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
            '!' => {
                let token_type = if self.match_token('=') {
                    super::token_types::TokenTypes::BangEqual
                } else {
                    super::token_types::TokenTypes::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_token('=') {
                    super::token_types::TokenTypes::EqualEqual
                } else {
                    super::token_types::TokenTypes::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_token('=') {
                    super::token_types::TokenTypes::LessEqual
                } else {
                    super::token_types::TokenTypes::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_token('=') {
                    super::token_types::TokenTypes::GreaterEqual
                } else {
                    super::token_types::TokenTypes::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_token('/') {
                    // A comment goes until the end of the line.
                    while !self.is_at_end() && self.advance() != '\n' {}
                } else {
                    self.add_token(super::token_types::TokenTypes::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace and newlines.
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                // Handle string literals.
                while !self.is_at_end() && self.advance() != '"' {
                    if self.advance() == '\n' {
                        self.line += 1;
                    }
                }
                if self.is_at_end() {
                    // Unterminated string literal.
                    println!("Error: Unterminated string at line {}", self.line);

                    return;
                } else {
                    // Add the string token.
                    let _lexeme = self.source[self.start + 1..self.current - 1].to_string();
                    self.add_token(super::token_types::TokenTypes::String);
                }
            }
            _ => {
                if (self.is_digit(character)) {
                    // Handle number literals.
                    while !self.is_at_end()
                        && self.is_digit(self.source.chars().nth(self.current).unwrap())
                    {
                        self.advance();
                    }
                    let lexeme = self.source[self.start..self.current].to_string();
                    self.add_token(super::token_types::TokenTypes::Number);
                } else if character.is_alphabetic() || character == '_' {
                    // Handle identifiers and keywords.
                    while !self.is_at_end()
                        && (self
                            .source
                            .chars()
                            .nth(self.current)
                            .unwrap()
                            .is_alphanumeric()
                            || self.source.chars().nth(self.current).unwrap() == '_')
                    {
                        self.advance();
                    }
                    let lexeme = self.source[self.start..self.current].to_string();
                    // Here you would typically check if the lexeme is a keyword.
                    self.add_token(super::token_types::TokenTypes::Identifier);
                }

                println!("Unexpected character '{}' at line {}", character, self.line);
                return;
            }
        }
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

   fn add_token(&mut self, token_type: super::token_types::TokenTypes) {
        self.add_token_with_literal(token_type, None);
    }


    fn add_token_with_literal(&mut self, token_type: super::token_types::TokenTypes, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }


    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) == Some(expected) {
            self.current += 1;
            return true;
        }
        false
    }

    // Handles string literals.
    fn string(&mut self) {
        // 
        while self.peek() != '"' && !self.is_at_end() {
            if (self.peek() == '\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
           error(&self.line, "Unterminated string literal");
           return; 
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        
        self.add_token_with_literal(token_types::TokenTypes::String, Some(value));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        
        self.source.chars().nth(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, character: char) -> bool {
        character >= '0' && character <= '9'
    }
}
