use bincode::config;
use bincode::{Decode, Encode};
use sel::OpCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct BytecodeHeader {
    pub magic_number: [u8; 4], // RSEL
    pub mod_name: String,
    pub mod_author: String,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct BytecodeFile {
    pub header: BytecodeHeader,
    pub bytecode: Vec<SerializableOpCode>,
}

// Wrapper serializable para OpCode
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum SerializableOpCode {
    Push(SerializableValue),
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Jump(usize),
    JumpIf(usize),
    JumpIfNot(usize),
    Call(usize),
    Return,
    Load(usize),
    Store(usize),
    ArrayNew,
    ArrayGet,
    ArraySet,
    ArrayLen,
    ToInt,
    ToFloat,
    ToString,
    ToBool,
    Print,
    Halt,
    Nop,
}

// Wrapper serializable para sel::Value
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum SerializableValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<SerializableValue>),
    Object(HashMap<String, SerializableValue>),
}

impl From<sel::Value> for SerializableValue {
    fn from(value: sel::Value) -> Self {
        match value {
            sel::Value::Null => SerializableValue::Null,
            sel::Value::Boolean(b) => SerializableValue::Boolean(b),
            sel::Value::Integer(i) => SerializableValue::Integer(i),
            sel::Value::Float(f) => SerializableValue::Float(f),
            sel::Value::String(s) => SerializableValue::String(s),
            sel::Value::Array(arr) => {
                SerializableValue::Array(arr.into_iter().map(Into::into).collect())
            }
            sel::Value::Object(obj) => {
                SerializableValue::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect()
                )
            }
        }
    }
}

impl From<SerializableValue> for sel::Value {
    fn from(value: SerializableValue) -> Self {
        match value {
            SerializableValue::Null => sel::Value::Null,
            SerializableValue::Boolean(b) => sel::Value::Boolean(b),
            SerializableValue::Integer(i) => sel::Value::Integer(i),
            SerializableValue::Float(f) => sel::Value::Float(f),
            SerializableValue::String(s) => sel::Value::String(s),
            SerializableValue::Array(arr) => {
                sel::Value::Array(arr.into_iter().map(Into::into).collect())
            }
            SerializableValue::Object(obj) => {
                sel::Value::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect()
                )
            }
        }
    }
}

impl From<OpCode> for SerializableOpCode {
    fn from(opcode: OpCode) -> Self {
        match opcode {
            OpCode::Push(v) => SerializableOpCode::Push(v.into()),
            OpCode::Pop => SerializableOpCode::Pop,
            OpCode::Dup => SerializableOpCode::Dup,
            OpCode::Swap => SerializableOpCode::Swap,
            OpCode::Add => SerializableOpCode::Add,
            OpCode::Sub => SerializableOpCode::Sub,
            OpCode::Mul => SerializableOpCode::Mul,
            OpCode::Div => SerializableOpCode::Div,
            OpCode::Mod => SerializableOpCode::Mod,
            OpCode::And => SerializableOpCode::And,
            OpCode::Or => SerializableOpCode::Or,
            OpCode::Not => SerializableOpCode::Not,
            OpCode::Eq => SerializableOpCode::Eq,
            OpCode::Ne => SerializableOpCode::Ne,
            OpCode::Lt => SerializableOpCode::Lt,
            OpCode::Le => SerializableOpCode::Le,
            OpCode::Gt => SerializableOpCode::Gt,
            OpCode::Ge => SerializableOpCode::Ge,
            OpCode::Jump(addr) => SerializableOpCode::Jump(addr),
            OpCode::JumpIf(addr) => SerializableOpCode::JumpIf(addr),
            OpCode::JumpIfNot(addr) => SerializableOpCode::JumpIfNot(addr),
            OpCode::Call(addr) => SerializableOpCode::Call(addr),
            OpCode::Return => SerializableOpCode::Return,
            OpCode::Load(id) => SerializableOpCode::Load(id),
            OpCode::Store(id) => SerializableOpCode::Store(id),
            OpCode::ArrayNew => SerializableOpCode::ArrayNew,
            OpCode::ArrayGet => SerializableOpCode::ArrayGet,
            OpCode::ArraySet => SerializableOpCode::ArraySet,
            OpCode::ArrayLen => SerializableOpCode::ArrayLen,
            OpCode::ToInt => SerializableOpCode::ToInt,
            OpCode::ToFloat => SerializableOpCode::ToFloat,
            OpCode::ToString => SerializableOpCode::ToString,
            OpCode::ToBool => SerializableOpCode::ToBool,
            OpCode::Print => SerializableOpCode::Print,
            OpCode::Halt => SerializableOpCode::Halt,
            OpCode::Nop => SerializableOpCode::Nop,
        }
    }
}

impl From<SerializableOpCode> for OpCode {
    fn from(serializable: SerializableOpCode) -> Self {
        match serializable {
            SerializableOpCode::Push(v) => OpCode::Push(v.into()),
            SerializableOpCode::Pop => OpCode::Pop,
            SerializableOpCode::Dup => OpCode::Dup,
            SerializableOpCode::Swap => OpCode::Swap,
            SerializableOpCode::Add => OpCode::Add,
            SerializableOpCode::Sub => OpCode::Sub,
            SerializableOpCode::Mul => OpCode::Mul,
            SerializableOpCode::Div => OpCode::Div,
            SerializableOpCode::Mod => OpCode::Mod,
            SerializableOpCode::And => OpCode::And,
            SerializableOpCode::Or => OpCode::Or,
            SerializableOpCode::Not => OpCode::Not,
            SerializableOpCode::Eq => OpCode::Eq,
            SerializableOpCode::Ne => OpCode::Ne,
            SerializableOpCode::Lt => OpCode::Lt,
            SerializableOpCode::Le => OpCode::Le,
            SerializableOpCode::Gt => OpCode::Gt,
            SerializableOpCode::Ge => OpCode::Ge,
            SerializableOpCode::Jump(addr) => OpCode::Jump(addr),
            SerializableOpCode::JumpIf(addr) => OpCode::JumpIf(addr),
            SerializableOpCode::JumpIfNot(addr) => OpCode::JumpIfNot(addr),
            SerializableOpCode::Call(addr) => OpCode::Call(addr),
            SerializableOpCode::Return => OpCode::Return,
            SerializableOpCode::Load(id) => OpCode::Load(id),
            SerializableOpCode::Store(id) => OpCode::Store(id),
            SerializableOpCode::ArrayNew => OpCode::ArrayNew,
            SerializableOpCode::ArrayGet => OpCode::ArrayGet,
            SerializableOpCode::ArraySet => OpCode::ArraySet,
            SerializableOpCode::ArrayLen => OpCode::ArrayLen,
            SerializableOpCode::ToInt => OpCode::ToInt,
            SerializableOpCode::ToFloat => OpCode::ToFloat,
            SerializableOpCode::ToString => OpCode::ToString,
            SerializableOpCode::ToBool => OpCode::ToBool,
            SerializableOpCode::Print => OpCode::Print,
            SerializableOpCode::Halt => OpCode::Halt,
            SerializableOpCode::Nop => OpCode::Nop,
        }
    }
}

pub fn pack_bytecode(opcodes: Vec<OpCode>, mod_name: String, mod_author: String) -> Vec<u8> {
    let serializable_opcodes: Vec<SerializableOpCode> =
        opcodes.into_iter().map(Into::into).collect();

    let file = BytecodeFile {
        header: BytecodeHeader {
            magic_number: *b"RSEL",
            mod_name,
            mod_author,
        },
        bytecode: serializable_opcodes,
    };

    // Usar bincode v2 para serialización binaria eficiente
    let config = config::standard();
    bincode::encode_to_vec(&file, config).expect("Failed to serialize bytecode with bincode")
}

pub fn read_bytecode(data: Vec<u8>) -> Result<BytecodeFile, String> {
    let config = config::standard();
    let (file, _): (BytecodeFile, usize) = bincode::decode_from_slice(&data, config)
        .map_err(|e| format!("Invalid bytecode format: {}", e))?;
    Ok(file)
}

// Función de conveniencia para extraer OpCodes del archivo de bytecode
pub fn extract_opcodes(bytecode_file: BytecodeFile) -> Vec<OpCode> {
    bytecode_file.bytecode.into_iter().map(Into::into).collect()
}
