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

    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,
    BinaryMod,

    ComparisonEq,
    ComparisonGreater,
    ComparisonGreaterEqual,
    ComparisonLess,
    ComparisonLessEqual,

    JumpTrue(usize),
    JumpFalse(usize),

    // This is implemented straight in the interpreter. It is planned that this will only be used for debug purposes and will be replaced by a standard library print function
    IntrinsicPrint,
}
