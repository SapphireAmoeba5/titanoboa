#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Type {
    Int(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    Char(u8),
    String(String),
    None,
}