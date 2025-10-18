// crates/sel/src/vm/debugger.rs
use crate::{OpCode, VMError, Value, VirtualMachine};

#[derive(Debug)]
pub struct CrashReport<'a> {
    pub error: &'a VMError,
    pub pc: usize,
    pub stack: Vec<Value>,
    pub program: Vec<OpCode>,
    pub current_instruction: Option<OpCode>,
}

impl<'a> CrashReport<'a> {
    pub fn from_vm(vm: &mut VirtualMachine, error: &'a VMError) -> Self {
        let pc = vm.get_pc();
        let stack = vm.memory().get_stack_snapshot();
        let program = vm.get_program().to_vec();
        let current_instruction = vm.current_instruction().cloned();

        Self {
            error,
            pc,
            stack,
            program,
            current_instruction,
        }
    }

    pub fn display(&self) {
        println!("\n{}", "═".repeat(70));
        println!("{}", "✗ SEL VIRTUAL MACHINE CRASH REPORT ✗");
        println!("{}", "═".repeat(70));

        println!("\n{}", "ERROR:");
        println!("  └─ {}\n", self.error.to_string());

        println!("{}", "PROGRAM COUNTER (PC):");
        println!("  └─ {}\n", self.pc);

        println!("{}", "CURRENT INSTRUCTION:");
        if let Some(instr) = &self.current_instruction {
            println!("  └─ [{}] {}", self.pc, self.format_instruction(instr));
        } else {
            println!("  └─ [{}] <END OF PROGRAM>", self.pc);
        }
        println!();

        println!("{}", "NEARBY INSTRUCTIONS:");
        let start = self.pc.saturating_sub(3);
        let end = (self.pc + 4).min(self.program.len());
        for i in start..end {
            let marker = if i == self.pc { ">>> " } else { "    " };
            println!(
                "  {}[{:3}] {}",
                marker,
                i,
                self.format_instruction(&self.program[i])
            );
        }
        println!();

        println!("{}", "STACK STATE:");
        if self.stack.is_empty() {
            println!("  └─ <empty stack>");
        } else {
            for (i, value) in self.stack.iter().enumerate() {
                let pos = self.stack.len() - 1 - i;
                let marker = if i == 0 { ">>> (top)" } else { "    " };
                println!(
                    "  {} [{:2}] {} ({})",
                    marker,
                    pos,
                    self.format_value(value),
                    value.type_name()
                );
            }
        }
        println!();

        println!("{}", "STATISTICS:");
        println!("  └─ Stack depth: {}", self.stack.len());
        println!("  └─ Program size: {} instructions", self.program.len());
        println!(
            "  └─ Execution progress: {:.1}%",
            (self.pc as f64 / self.program.len() as f64) * 100.0
        );
        println!();

        println!("{}", "SUGGESTIONS:");
        self.print_suggestions();

        println!("{}", "═".repeat(70));
        println!();
    }

    fn format_instruction(&self, instr: &OpCode) -> String {
        match instr {
            OpCode::Push(v) => format!("PUSH {}", self.format_value(v)),
            OpCode::Pop => "POP".to_string(),
            OpCode::Dup => "DUP".to_string(),
            OpCode::Swap => "SWAP".to_string(),
            OpCode::Add => "ADD".to_string(),
            OpCode::Sub => "SUB".to_string(),
            OpCode::Mul => "MUL".to_string(),
            OpCode::Div => "DIV".to_string(),
            OpCode::Mod => "MOD".to_string(),
            OpCode::And => "AND".to_string(),
            OpCode::Or => "OR".to_string(),
            OpCode::Not => "NOT".to_string(),
            OpCode::Eq => "EQ".to_string(),
            OpCode::Ne => "NE".to_string(),
            OpCode::Lt => "LT".to_string(),
            OpCode::Le => "LE".to_string(),
            OpCode::Gt => "GT".to_string(),
            OpCode::Ge => "GE".to_string(),
            OpCode::Jump(addr) => format!("JUMP {}", addr),
            OpCode::JumpIf(addr) => format!("JUMPIF {}", addr),
            OpCode::JumpIfNot(addr) => format!("JUMPIFNOT {}", addr),
            OpCode::Call(addr) => format!("CALL {}", addr),
            OpCode::Return => "RETURN".to_string(),
            OpCode::Load(id) => format!("LOAD {}", id),
            OpCode::Store(id) => format!("STORE {}", id),
            OpCode::ArrayNew => "ARRAYNEW".to_string(),
            OpCode::ArrayGet => "ARRAYGET".to_string(),
            OpCode::ArraySet => "ARRAYSET".to_string(),
            OpCode::ArrayLen => "ARRAYLEN".to_string(),
            OpCode::ToInt => "TOINT".to_string(),
            OpCode::ToFloat => "TOFLOAT".to_string(),
            OpCode::ToString => "TOSTRING".to_string(),
            OpCode::ToBool => "TOBOOL".to_string(),
            OpCode::Print => "PRINT".to_string(),
            OpCode::Halt => "HALT".to_string(),
            OpCode::Nop => "NOP".to_string(),
        }
    }

    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::Array(arr) => format!("[array: {} items]", arr.len()),
            Value::Object(obj) => format!("{{object: {} keys}}", obj.len()),
            Value::Null => "null".to_string(),
        }
    }

    fn print_suggestions(&self) {
        match &self.error {
            VMError::StackUnderflow => {
                println!("  ├─ La instrucción intenta sacar más elementos del que hay");
                println!("  ├─ Stack actual: {} elementos", self.stack.len());
                println!("  └─ Revisa: POP, DUP, SWAP, operaciones binarias (ADD, SUB, etc)");
            }
            VMError::StackOverflow => {
                println!("  ├─ Se superó el límite de stack");
                println!("  ├─ Aumenta el tamaño con: glyph -r archivo.sel -s 8KiB");
                println!("  └─ O reduce PUSH operations en tu programa");
            }
            VMError::DivisionByZero => {
                println!("  ├─ Intentaste dividir por cero");
                println!("  ├─ Usa condicionales para verificar antes de DIV o MOD");
                println!("  └─ Ejemplo: PUSH 0 PUSH valor NE JUMPIF safe_div");
            }
            VMError::InvalidAddress(addr) => {
                println!("  ├─ JUMP/CALL a dirección inválida: {}", addr);
                println!("  ├─ Rango válido: 0 - {}", self.program.len() - 1);
                println!("  └─ Revisa tus labels y direcciones de salto");
            }
            VMError::IndexOutOfBounds => {
                println!("  ├─ Array index fuera de rango");
                println!("  ├─ Verifica que el índice sea >= 0 y < longitud del array");
                println!("  └─ Usa ARRAYLEN para obtener el tamaño del array");
            }
            VMError::TypeMismatch(msg) => {
                println!("  ├─ Operación con tipos incompatibles");
                println!("  ├─ {}", msg);
                println!("  └─ Usa operaciones de conversión: TOINT, TOFLOAT, TOSTRING, TOBOOL");
            }
            VMError::UndefinedVariable(id) => {
                println!("  ├─ Variable {} no fue inicializada", id);
                println!("  ├─ Asegúrate de STORE antes de LOAD");
                println!("  └─ Usa: PUSH valor STORE {}", id);
            }
            VMError::RuntimeError(msg) => {
                println!("  └─ {}", msg);
            }
        }
    }
}
