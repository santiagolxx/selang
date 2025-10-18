use crate::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    // Stack operations
    Push(Value),
    Pop,
    Dup,
    Swap,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Logic
    And,
    Or,
    Not,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Control flow
    Jump(usize),
    JumpIf(usize),
    JumpIfNot(usize),
    Call(usize),
    Return,

    // Variables
    Load(usize),
    Store(usize),

    // Array operations
    ArrayNew,
    ArrayGet,
    ArraySet,
    ArrayLen,

    // Type conversion
    ToInt,
    ToFloat,
    ToString,
    ToBool,

    // I/O
    Print,

    // Control
    Halt,
    Nop,
}
