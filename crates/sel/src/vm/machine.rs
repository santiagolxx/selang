use crate::{Memory, OpCode, Result, VMError, vm::operations};

pub struct VirtualMachine {
    memory: Memory,
    program: Vec<OpCode>,
    pc: usize,
    running: bool,
}

impl VirtualMachine {
    /// Crea una nueva máquina virtual con el tamaño de stack especificado
    pub fn new(stack_size: usize) -> Self {
        Self {
            memory: Memory::new(stack_size),
            program: Vec::new(),
            pc: 0,
            running: false,
        }
    }

    /// Carga un programa en la VM
    pub fn load_program(&mut self, program: Vec<OpCode>) {
        self.program = program;
        self.pc = 0;
        self.running = false;
    }

    /// Ejecuta el programa completo
    pub fn run(&mut self) -> Result<()> {
        self.running = true;
        self.memory.reset();

        while self.running && self.pc < self.program.len() {
            self.step()?;
        }

        Ok(())
    }

    /// Ejecuta una sola instrucción
    pub fn step(&mut self) -> Result<()> {
        if self.pc >= self.program.len() {
            self.running = false;
            return Ok(());
        }

        let instruction = self.program[self.pc].clone();
        self.execute_instruction(&instruction)
    }

    /// Ejecuta una instrucción específica
    fn execute_instruction(&mut self, instruction: &OpCode) -> Result<()> {
        match instruction {
            // Stack operations
            OpCode::Push(value) => operations::push(self, value.clone()),
            OpCode::Pop => operations::pop(self),
            OpCode::Dup => operations::dup(self),
            OpCode::Swap => operations::swap(self),

            // Arithmetic
            OpCode::Add => operations::add(self),
            OpCode::Sub => operations::sub(self),
            OpCode::Mul => operations::mul(self),
            OpCode::Div => operations::div(self),
            OpCode::Mod => operations::modulo(self),

            // Logic
            OpCode::And => operations::and(self),
            OpCode::Or => operations::or(self),
            OpCode::Not => operations::not(self),

            // Comparison
            OpCode::Eq => operations::eq(self),
            OpCode::Ne => operations::ne(self),
            OpCode::Lt => operations::lt(self),
            OpCode::Le => operations::le(self),
            OpCode::Gt => operations::gt(self),
            OpCode::Ge => operations::ge(self),

            // Control flow
            OpCode::Jump(addr) => operations::jump(self, *addr),
            OpCode::JumpIf(addr) => operations::jump_if(self, *addr),
            OpCode::JumpIfNot(addr) => operations::jump_if_not(self, *addr),
            OpCode::Call(addr) => operations::call(self, *addr),
            OpCode::Return => operations::ret(self),

            // Variables
            OpCode::Load(id) => operations::load(self, *id),
            OpCode::Store(id) => operations::store(self, *id),

            // Arrays
            OpCode::ArrayNew => operations::array_new(self),
            OpCode::ArrayGet => operations::array_get(self),
            OpCode::ArraySet => operations::array_set(self),
            OpCode::ArrayLen => operations::array_len(self),

            // Type conversion
            OpCode::ToInt => operations::to_int(self),
            OpCode::ToFloat => operations::to_float(self),
            OpCode::ToString => operations::to_string(self),
            OpCode::ToBool => operations::to_bool(self),

            // I/O
            OpCode::Print => operations::print(self),

            // Control
            OpCode::Halt => operations::halt(self),
            OpCode::Nop => operations::nop(self),
        }
    }

    /// Avanza el program counter en 1
    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    /// Salta a una dirección específica
    pub fn jump_to(&mut self, addr: usize) -> Result<()> {
        if addr >= self.program.len() {
            return Err(VMError::InvalidAddress(addr));
        }
        self.pc = addr;
        Ok(())
    }

    /// Obtiene una referencia mutable a la memoria
    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    /// Verifica si la VM está ejecutándose
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Detiene la ejecución de la VM
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Reinicia la VM al estado inicial
    pub fn reset(&mut self) {
        self.pc = 0;
        self.running = false;
        self.memory.reset();
    }

    /// Obtiene el program counter actual
    pub fn get_pc(&self) -> usize {
        self.pc
    }

    /// Obtiene el tamaño del programa cargado
    pub fn program_size(&self) -> usize {
        self.program.len()
    }

    /// Obtiene una referencia al programa actual (para debugging)
    pub fn get_program(&self) -> &[OpCode] {
        &self.program
    }

    /// Obtiene la instrucción actual (para debugging)
    pub fn current_instruction(&self) -> Option<&OpCode> {
        self.program.get(self.pc)
    }
}

impl std::fmt::Debug for VirtualMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualMachine")
            .field("pc", &self.pc)
            .field("running", &self.running)
            .field("program_size", &self.program.len())
            .field("stack_size", &self.memory.stack_size())
            .finish()
    }
}
