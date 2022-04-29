#[derive(Debug, Clone)]
pub enum Token {
    Fn {
        line: usize,
        start: usize,
        end: usize,
    },
    Arrow {
        line: usize,
        start: usize,
        end: usize,
    },
    Comma {
        line: usize,
        start: usize,
        end: usize,
    },
    Period {
        line: usize,
        start: usize,
        end: usize,
    },

    Colon {
        line: usize,
        start: usize,
        end: usize,
    },

    Assignment {
        line: usize,
        start: usize,
        end: usize,
    },

    Equality {
        line: usize,
        start: usize,
        end: usize,
    },

    EqualityNot {
        line: usize,
        start: usize,
        end: usize,
    },

    GreaterThan {
        line: usize,
        start: usize,
        end: usize,
    },

    GreaterThanEqual {
        line: usize,
        start: usize,
        end: usize,
    },

    LessThan {
        line: usize,
        start: usize,
        end: usize,
    },

    LessThanEqual {
        line: usize,
        start: usize,
        end: usize,
    },

    Return {
        line: usize,
        start: usize,
        end: usize,
    },

    IntegerLiteral {
        line: usize,
        start: usize,
        end: usize,
        value: i64,
    },
    UIntegerLiteral {
        line: usize,
        start: usize,
        end: usize,
        value: u64,
    },
    FloatLiteral {
        line: usize,
        start: usize,
        end: usize,
        value: f64,
    },
    CharLiteral {
        line: usize,
        start: usize,
        end: usize,
        value: u8,
    },
    StringLiteral {
        line: usize,
        start: usize,
        end: usize,
        value: String,
    },
    BooleanLiteral {
        line: usize,
        start: usize,
        end: usize,
        value: bool,
    },

    Int {
        line: usize,
        start: usize,
        end: usize,
    },
    Uint {
        line: usize,
        start: usize,
        end: usize,
    },
    Float {
        line: usize,
        start: usize,
        end: usize,
    },
    Bool {
        line: usize,
        start: usize,
        end: usize,
    },
    Char {
        line: usize,
        start: usize,
        end: usize,
    },
    String {
        line: usize,
        start: usize,
        end: usize,
    },
    // This token is just the 'None' keyword
    None {
        line: usize,
        start: usize,
        end: usize,
    },

    True {
        line: usize,
        start: usize,
        end: usize,
    },

    False {
        line: usize,
        start: usize,
        end: usize,
    },

    If {
        line: usize,
        start: usize,
        end: usize,
    },

    Elif {
        line: usize,
        start: usize,
        end: usize,
    },

    Else {
        line: usize,
        start: usize,
        end: usize,
    },

    Begin {
        line: usize,
        start: usize,
        end: usize,
    },

    Then {
        line: usize,
        start: usize,
        end: usize,
    },

    Do {
        line: usize,
        start: usize,
        end: usize,
    },

    End {
        line: usize,
        start: usize,
        end: usize,
    },

    OpeningBracket {
        line: usize,
        start: usize,
        end: usize,
    },
    ClosingBracket {
        line: usize,
        start: usize,
        end: usize,
    },

    Identifier {
        line: usize,
        start: usize,
        end: usize,
    },

    Newline {
        line: usize,
        start: usize,
        end: usize,
    },

    Unknown {
        line: usize,
        start: usize,
        end: usize,
        token: String,
    },
}
