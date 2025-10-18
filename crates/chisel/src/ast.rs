use sel::Value;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Label(String),
    Instruction(Instruction),
}

#[derive(Debug, Clone)]
pub enum Instruction {
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
    Jump(JumpTarget),
    JumpIf(JumpTarget),
    JumpIfNot(JumpTarget),
    Call(JumpTarget),
    Return,

    // Variables
    Load(usize),
    Store(usize),

    // Arrays
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

#[derive(Debug, Clone)]
pub enum JumpTarget {
    Address(usize),
    Label(String),
}
