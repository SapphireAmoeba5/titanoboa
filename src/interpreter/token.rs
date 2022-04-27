#[derive(Debug, Clone)]
pub enum Token {
    Fn,
    Arrow,
    
    IntegerLiteral(i64),
    UIntegerLiteral(u64),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),
    BooleanLiteral(bool),

    Comma,
    Int,
    Uint,
    Float,
    Bool,
    Char,
    String,
    None,

    OpeningBracket,
    ClosingBracket,

    Identifier(String),

}