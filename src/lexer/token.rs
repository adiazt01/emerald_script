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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(
            TokenTypes::Identifier,
            "myVar".to_string(),
            Some("42".to_string()),
            1,
        );
        assert_eq!(token.token_type, TokenTypes::Identifier);
        assert_eq!(token.lexeme, "myVar");
        assert_eq!(token.literal, Some("42".to_string()));
        assert_eq!(token.line, 1);
    }

    #[test]
    fn test_token_to_string() {
        let token = Token::new(
            TokenTypes::String,
            "\"Hello\"".to_string(),
            None,
            2,
        );
        assert_eq!(
            token.to_string(),
            "Token { type: String, lexeme: \"Hello\", literal: None, line: 2 }"
        );
    }
}