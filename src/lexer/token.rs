use crate::lexer::token_types::TokenTypes;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenTypes,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenTypes, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Token {{ type: {:?}, lexeme: {}, literal: {:?}, line: {} }}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}
