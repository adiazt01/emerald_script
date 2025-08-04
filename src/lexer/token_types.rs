#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenTypes {
    // SINGLE-CHAR TOKENS
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // ONE OR TWO CHAR TOKENS
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    // LITERALS
    Identifier, String, Number,

    // KEYWORDS
    And, Class, Else, False,
    For, Fun, If, Nil,
    Or, Print, Return, Super,
    This, True, Var, While,

    // END OF FILE
    Eof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_types() {
        assert_eq!(TokenTypes::LeftParen as u8, 0);
        assert_eq!(TokenTypes::RightParen as u8, 1);
        assert_eq!(TokenTypes::Identifier as u8, 20);
        assert_eq!(TokenTypes::Eof as u8, 27);
    }

    #[test]
    fn test_token_types_equality() {
        assert_eq!(TokenTypes::LeftParen, TokenTypes::LeftParen);
        assert_ne!(TokenTypes::LeftParen, TokenTypes::RightParen);
    }
}