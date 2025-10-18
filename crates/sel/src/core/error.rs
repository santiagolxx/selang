use std::fmt;

#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    StackOverflow,
    InvalidAddress(usize),
    DivisionByZero,
    TypeMismatch(String),
    UndefinedVariable(usize),
    IndexOutOfBounds,
    RuntimeError(String),
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::StackOverflow => write!(f, "Stack overflow"),
            VMError::InvalidAddress(addr) => write!(f, "Invalid address: {}", addr),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            VMError::UndefinedVariable(id) => write!(f, "Undefined variable: {}", id),
            VMError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            VMError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for VMError {}

pub type Result<T> = std::result::Result<T, VMError>;
