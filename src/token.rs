#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Equal,        // =
    NotEqual,     // <>
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Exponent,     // ^
    Greater,      // >
    GreaterEqual, // >=
    Concatenate,  // &
    Colon,        // :
    Comma,        // , 求并集
    EmptyChar,    // ' ' 求交集
    Percent,      // %
    Less,         // <
    LessEqual,    // <=
    Identifier,   //
    String,       // 字符串
    Float,        // 浮点数
    Integer,      // 整数
    True,         // true
    False,        // false
    LeftBracket,  // (
    RightBracket, // )
    LeftBrace,    // {
    RightBrace,   // }
    Semicolon,    // ;
    Exclamation,  // !
    R1C1,         // R1C1 引用
    MixedCell,    // 混合引用
    AbsoluteCell, // 绝对引用
    Eof,          // 文件结束符
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }

    pub fn to_string(&self) -> &str {
        &self.value
    }
}
