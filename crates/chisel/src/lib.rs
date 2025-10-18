mod ast;
mod bytecode;
mod parser;
mod preprocessor;

pub use ast::*;
pub use bytecode::{BytecodeFile, BytecodeHeader, extract_opcodes, pack_bytecode, read_bytecode};
pub use parser::parse_program;
pub use preprocessor::{PreprocessorError, preprocess_source};

// Re-export from sel
pub use sel::{OpCode, Value};

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use std::collections::HashMap;

#[derive(Debug)]
pub enum CompilerError {
    Preprocessor(PreprocessorError),
    Parser(Vec<Rich<'static, char>>),
    Assembler(String),
}

impl CompilerError {
    /// Genera un reporte visual usando Ariadne
    pub fn report_with_ariadne(&self, source_code: &str, filename: &str) {
        match self {
            CompilerError::Preprocessor(e) => {
                eprintln!("Error de preprocesamiento: {}", e);
            }
            CompilerError::Parser(errors) => {
                for error in errors {
                    let span = error.span();
                    Report::build(ReportKind::Error, (filename, span.start..span.end))
                        .with_message("Error de sintaxis")
                        .with_label(
                            Label::new((filename, span.start..span.end))
                                .with_message(error.reason().to_string())
                                .with_color(ariadne::Color::Red),
                        )
                        .finish()
                        .print((filename, Source::from(source_code)))
                        .unwrap();
                }
            }
            CompilerError::Assembler(e) => {
                eprintln!("Error del ensamblador: {}", e);
            }
        }
    }
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::Preprocessor(e) => write!(f, "Error de preprocesamiento: {}", e),
            CompilerError::Parser(errors) => {
                writeln!(f, "Errores de sintaxis:")?;
                for error in errors {
                    writeln!(f, "  • {}", error.reason())?;
                }
                Ok(())
            }
            CompilerError::Assembler(e) => write!(f, "Error del ensamblador: {}", e),
        }
    }
}

impl std::error::Error for CompilerError {}

/// Compila código fuente completo a bytecode
pub fn compile_source(
    source: &str,
    mod_name: String,
    mod_author: String,
) -> Result<Vec<u8>, CompilerError> {
    // 1. Preprocesamiento
    let preprocessed = preprocess_source(source).map_err(CompilerError::Preprocessor)?;

    // 2. Parsing
    let program = parse_program(preprocessed).map_err(CompilerError::Parser)?;

    // 3. Generación de bytecode
    let bytecode = generate_bytecode(program).map_err(CompilerError::Assembler)?;

    // 4. Empaquetado
    Ok(pack_bytecode(bytecode, mod_name, mod_author))
}

/// Genera bytecode a partir del AST
fn generate_bytecode(program: Program) -> Result<Vec<OpCode>, String> {
    let mut assembler = Assembler::new();

    // Primera pasada: registrar labels y contar instrucciones
    for statement in &program.statements {
        match statement {
            Statement::Label(name) => {
                assembler.define_label(name.clone())?;
            }
            Statement::Instruction(_) => {
                assembler.advance_position();
            }
        }
    }

    // Segunda pasada: generar opcodes
    for statement in program.statements {
        match statement {
            Statement::Label(_) => {
                // Las etiquetas ya fueron procesadas
                continue;
            }
            Statement::Instruction(instruction) => {
                assembler.emit_instruction(instruction)?;
            }
        }
    }

    // Tercera pasada: resolver referencias a labels
    assembler.resolve_labels()
}

/// Ensamblador que maneja la generación de bytecode y resolución de labels
struct Assembler {
    opcodes: Vec<OpCode>,
    label_positions: HashMap<String, usize>,
    label_references: Vec<LabelReference>,
    current_position: usize,
}

#[derive(Debug, Clone)]
struct LabelReference {
    opcode_index: usize,
    label_name: String,
    reference_type: ReferenceType,
}

#[derive(Debug, Clone)]
enum ReferenceType {
    Jump,
    JumpIf,
    JumpIfNot,
    Call,
}

impl Assembler {
    fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            label_positions: HashMap::new(),
            label_references: Vec::new(),
            current_position: 0,
        }
    }

    /// Define un label en la posición actual
    fn define_label(&mut self, name: String) -> Result<(), String> {
        if self.label_positions.contains_key(&name) {
            return Err(format!("Label '{}' ya está definido", name));
        }
        self.label_positions.insert(name, self.current_position);
        Ok(())
    }

    /// Avanza la posición actual (para contar instrucciones en primera pasada)
    fn advance_position(&mut self) {
        self.current_position += 1;
    }

    /// Emite una instrucción
    fn emit_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        let opcode = match instruction {
            Instruction::Push(value) => OpCode::Push(value),
            Instruction::Pop => OpCode::Pop,
            Instruction::Dup => OpCode::Dup,
            Instruction::Swap => OpCode::Swap,

            Instruction::Add => OpCode::Add,
            Instruction::Sub => OpCode::Sub,
            Instruction::Mul => OpCode::Mul,
            Instruction::Div => OpCode::Div,
            Instruction::Mod => OpCode::Mod,

            Instruction::And => OpCode::And,
            Instruction::Or => OpCode::Or,
            Instruction::Not => OpCode::Not,

            Instruction::Eq => OpCode::Eq,
            Instruction::Ne => OpCode::Ne,
            Instruction::Lt => OpCode::Lt,
            Instruction::Le => OpCode::Le,
            Instruction::Gt => OpCode::Gt,
            Instruction::Ge => OpCode::Ge,

            Instruction::Jump(target) => self.emit_jump(target, ReferenceType::Jump)?,
            Instruction::JumpIf(target) => self.emit_jump(target, ReferenceType::JumpIf)?,
            Instruction::JumpIfNot(target) => self.emit_jump(target, ReferenceType::JumpIfNot)?,
            Instruction::Call(target) => self.emit_jump(target, ReferenceType::Call)?,
            Instruction::Return => OpCode::Return,

            Instruction::Load(id) => OpCode::Load(id),
            Instruction::Store(id) => OpCode::Store(id),

            Instruction::ArrayNew => OpCode::ArrayNew,
            Instruction::ArrayGet => OpCode::ArrayGet,
            Instruction::ArraySet => OpCode::ArraySet,
            Instruction::ArrayLen => OpCode::ArrayLen,

            Instruction::ToInt => OpCode::ToInt,
            Instruction::ToFloat => OpCode::ToFloat,
            Instruction::ToString => OpCode::ToString,
            Instruction::ToBool => OpCode::ToBool,

            Instruction::Print => OpCode::Print,
            Instruction::Halt => OpCode::Halt,
            Instruction::Nop => OpCode::Nop,
        };

        self.opcodes.push(opcode);
        Ok(())
    }

    /// Emite una instrucción de salto (puede necesitar resolución posterior)
    fn emit_jump(&mut self, target: JumpTarget, ref_type: ReferenceType) -> Result<OpCode, String> {
        match target {
            JumpTarget::Address(addr) => {
                // Dirección directa, no necesita resolución
                Ok(match ref_type {
                    ReferenceType::Jump => OpCode::Jump(addr),
                    ReferenceType::JumpIf => OpCode::JumpIf(addr),
                    ReferenceType::JumpIfNot => OpCode::JumpIfNot(addr),
                    ReferenceType::Call => OpCode::Call(addr),
                })
            }
            JumpTarget::Label(label) => {
                // Guardar referencia para resolución posterior
                self.label_references.push(LabelReference {
                    opcode_index: self.opcodes.len(),
                    label_name: label,
                    reference_type: ref_type.clone(),
                });

                // Emitir placeholder con dirección 0
                Ok(match ref_type {
                    ReferenceType::Jump => OpCode::Jump(0),
                    ReferenceType::JumpIf => OpCode::JumpIf(0),
                    ReferenceType::JumpIfNot => OpCode::JumpIfNot(0),
                    ReferenceType::Call => OpCode::Call(0),
                })
            }
        }
    }

    /// Resuelve todas las referencias a labels
    fn resolve_labels(mut self) -> Result<Vec<OpCode>, String> {
        for reference in self.label_references {
            let label_position = self
                .label_positions
                .get(&reference.label_name)
                .ok_or_else(|| format!("Label '{}' no está definido", reference.label_name))?;

            // Reemplazar el placeholder con la dirección real
            match &mut self.opcodes[reference.opcode_index] {
                OpCode::Jump(addr) if matches!(reference.reference_type, ReferenceType::Jump) => {
                    *addr = *label_position;
                }
                OpCode::JumpIf(addr)
                    if matches!(reference.reference_type, ReferenceType::JumpIf) =>
                {
                    *addr = *label_position;
                }
                OpCode::JumpIfNot(addr)
                    if matches!(reference.reference_type, ReferenceType::JumpIfNot) =>
                {
                    *addr = *label_position;
                }
                OpCode::Call(addr) if matches!(reference.reference_type, ReferenceType::Call) => {
                    *addr = *label_position;
                }
                _ => {
                    return Err(format!(
                        "Error interno: tipo de referencia no coincide con opcode"
                    ));
                }
            }
        }

        Ok(self.opcodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_compilation() {
        let source = r#"
            PUSH 42;
            PRINT;
            HALT;
        "#;

        let result = compile_source(source, "test".to_string(), "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_labels() {
        let source = r#"
            start:
            PUSH 1;
            JUMP start;
        "#;

        let result = compile_source(source, "test".to_string(), "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_undefined_label() {
        let source = r#"
            PUSH 1;
            JUMP undefined_label;
        "#;

        let result = compile_source(source, "test".to_string(), "test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_label() {
        let source = r#"
            loop:
            PUSH 1;
            loop:
            PUSH 2;
        "#;

        let result = compile_source(source, "test".to_string(), "test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_fibonacci_with_labels() {
        let source = r#"
            PUSH 0;
            STORE 0;
            PUSH 10;
            STORE 1;
            PUSH 0;
            STORE 2;
            PUSH 1;
            STORE 3;

            loop:
            LOAD 0;
            LOAD 1;
            GE;
            JUMPIF end;

            LOAD 2;
            PRINT;

            LOAD 3;
            STORE 4;

            LOAD 2;
            LOAD 3;
            ADD;
            STORE 3;

            LOAD 4;
            STORE 2;

            LOAD 0;
            PUSH 1;
            ADD;
            STORE 0;

            JUMP loop;

            end:
            HALT;
        "#;

        let result = compile_source(source, "fibonacci".to_string(), "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_forward_reference() {
        let source = r#"
            JUMP forward;
            PUSH 1;
            forward:
            PUSH 2;
            HALT;
        "#;

        let result = compile_source(source, "test".to_string(), "test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_jumps_same_label() {
        let source = r#"
            start:
            PUSH 1;
            JUMP start;
            JUMP start;
            JUMPIF start;
            JUMPIFNOT start;
        "#;

        let result = compile_source(source, "test".to_string(), "test".to_string());
        assert!(result.is_ok());
    }
}
