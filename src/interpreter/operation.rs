use super::Type;

/// The operations that the source code "compiles" into akin to how C code compiles to Assembly.
#[derive(Clone, Debug)]
pub enum Operation {
    // Name refers to a variable.
    // Store refers to loading the value on top of the stack into a variable.
    StoreName(String),

    PushName(String),
    // The word "Const" refers to a constant value
    PushConst(Type),

    AddConst(Type),
    SubConst(Type),
    MulConst(Type),
    DivConst(Type),
    ModConst(Type),
}
