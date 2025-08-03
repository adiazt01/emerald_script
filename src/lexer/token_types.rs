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