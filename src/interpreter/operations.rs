use super::Type;

/// The operations that the source code "compiles" into akin to how C code compiles to Assembly.
pub enum Operations {
    // The "Name" in PushName, AddName, etc refers to the name of a variable that should be defined
    PushName(String),
    AddName(String),
    // This operation will call a function somewhere in memory
    CallName(String),

    // The word "Const" refers to a constant value
    PushConst(Type),

    AddConst(Type),
    SubConst(Type),
    MulConst(Type),
    DivConst(Type),
    ModConst(Type),
}
