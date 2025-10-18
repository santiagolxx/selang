# Chisel Compiler Documentation

## Overview

Chisel is the compiler frontend for the Sel programming language. It transforms Sel assembly code into executable bytecode through a multi-stage compilation pipeline including preprocessing, parsing, and bytecode generation.

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Preprocessor   │───▶│     Parser      │───▶│    Assembler    │
│                 │    │                 │    │                 │
│ • Macro         │    │ • Tokenization  │    │ • Code          │
│   Expansion     │    │ • AST Building  │    │   Generation    │
│ • Include       │    │ • Syntax        │    │ • Label         │
│   Processing    │    │   Validation    │    │   Resolution    │
│ • Comment       │    │ • Error         │    │ • Bytecode      │
│   Removal       │    │   Reporting     │    │   Packaging     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                       │                       │
        ▼                       ▼                       ▼
   .sel source            AST (Program)           .selb bytecode
```

## Dependencies

### External Crates
- **chumsky**: Parser combinator library (v0.10.1)
- **ariadne**: Error reporting and diagnostics (v0.5.1)
- **serde**: Serialization framework (v1.0.219)
- **bincode**: Binary serialization (v2.0.1)

### Internal Crates
- **sel**: Core types and VM runtime

## Source Structure

```
src/
├── lib.rs           # Public API and error handling
├── ast.rs           # Abstract Syntax Tree definitions
├── parser.rs        # Parser implementation
├── preprocessor.rs  # Macro and preprocessing
└── bytecode.rs      # Bytecode generation and serialization
```

## Module Documentation

### lib.rs

**Purpose**: Public API, error handling, and compilation orchestration.

**Key Types**:

#### `CompilerError`
```rust
pub enum CompilerError {
    Preprocessor(PreprocessorError),
    Parser(Vec<Rich<'static, char>>),
    Assembler(String),
}
```

**Fields**:
- `Preprocessor`: Errors from macro expansion and preprocessing
- `Parser`: Collection of parsing errors with source spans
- `Assembler`: Errors from bytecode generation and label resolution

**Methods**:
- `report_with_ariadne(&self, source_code: &str, filename: &str)`: Generates visual error reports using Ariadne

#### `compile_source(source: &str, mod_name: String, mod_author: String) -> Result<Vec<u8>, CompilerError>`

**Purpose**: Main compilation entry point.

**Parameters**:
- `source: &str` - Raw Sel assembly source code
- `mod_name: String` - Program name for bytecode metadata
- `mod_author: String` - Author name for bytecode metadata

**Returns**: Compiled bytecode as `Vec<u8>` or compilation error

**Workflow**:
1. Preprocess source (macro expansion, includes)
2. Parse preprocessed source into AST
3. Generate opcodes from AST
4. Resolve labels and references
5. Package into bytecode format

### ast.rs

**Purpose**: Abstract Syntax Tree node definitions.

**Key Types**:

#### `Program`
```rust
pub struct Program {
    pub statements: Vec<Statement>,
}
```

**Fields**:
- `statements: Vec<Statement>` - Top-level program statements

#### `Statement`
```rust
pub enum Statement {
    Label(String),
    Instruction(Instruction),
}
```

**Variants**:
- `Label(String)` - Jump target labels
- `Instruction(Instruction)` - Executable instructions

#### `Instruction`
```rust
pub enum Instruction {
    // Stack operations
    Push(Value),
    Pop,
    Dup,
    Swap,

    // Arithmetic
    Add, Sub, Mul, Div, Mod,

    // Logic
    And, Or, Not,

    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,

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
    ArrayNew, ArrayGet, ArraySet, ArrayLen,

    // Type conversion
    ToInt, ToFloat, ToString, ToBool,

    // I/O
    Print,

    // Control
    Halt, Nop,
}
```

#### `JumpTarget`
```rust
pub enum JumpTarget {
    Address(usize),
    Label(String),
}
```

**Variants**:
- `Address(usize)` - Direct memory address
- `Label(String)` - Named label reference (resolved during assembly)

### parser.rs

**Purpose**: Transforms preprocessed source code into Abstract Syntax Tree.

**Key Functions**:

#### `parse_program(source: String) -> Result<Program, Vec<Rich<'static, char>>>`

**Purpose**: Main parsing entry point.

**Parameters**:
- `source: String` - Preprocessed source code

**Returns**: Parsed AST or collection of parsing errors

**Implementation Strategy**:
- Line-by-line parsing approach for simplicity and better error reporting
- Manual instruction parsing rather than complex combinators
- Comprehensive error messages with source spans

**Parsing Process**:
1. Split source into lines
2. Skip empty lines and comments
3. Validate semicolon requirements
4. Parse each line as instruction
5. Build AST from parsed instructions

#### `parse_instruction(line: &str) -> Result<Instruction, String>`

**Purpose**: Parses individual instruction lines.

**Parameters**:
- `line: &str` - Single instruction line (without semicolon)

**Returns**: Parsed instruction or error message

**Supported Instructions**:

**Stack Operations**:
- `PUSH <value>` - Push value onto stack
- `POP` - Remove top stack element
- `DUP` - Duplicate top stack element
- `SWAP` - Swap top two stack elements

**Arithmetic**:
- `ADD` - Add top two stack elements
- `SUB` - Subtract top two stack elements
- `MUL` - Multiply top two stack elements
- `DIV` - Divide top two stack elements

**Comparison**:
- `EQ` - Test equality
- `LT` - Test less than
- `LE` - Test less than or equal
- `GT` - Test greater than
- `GE` - Test greater than or equal

**Control Flow**:
- `JUMP <target>` - Unconditional jump
- `JUMPIF <target>` - Jump if true
- `JUMPIFNOT <target>` - Jump if false

**Memory**:
- `LOAD <index>` - Load variable to stack
- `STORE <index>` - Store stack top to variable

**I/O**:
- `PRINT` - Print stack top
- `HALT` - Stop execution

**Value Parsing**:
- Integers: `42`, `-123`
- Floats: `3.14`, `-2.5`
- Strings: `"hello world"`
- Booleans: `true`, `false`
- Null: `null`

### preprocessor.rs

**Purpose**: Handles macro expansion and source preprocessing.

**Key Types**:

#### `PreprocessorError`
```rust
pub enum PreprocessorError {
    InvalidDefine(String),
    CircularReference(String),
    IoError(String),
}
```

**Key Functions**:

#### `preprocess_source(source: &str) -> Result<String, PreprocessorError>`

**Purpose**: Preprocesses source code with macro expansion.

**Features**:
- `#define` macro definitions
- Macro expansion in source code
- Comment removal
- Circular reference detection

**Preprocessing Steps**:
1. Parse `#define` directives
2. Build macro definition table
3. Expand macros in source lines
4. Remove comments and empty lines
5. Return processed source

**Macro System**:
- Simple text replacement macros
- No parameter support (future enhancement)
- Circular reference protection
- Case-sensitive macro names

### bytecode.rs

**Purpose**: Bytecode generation, serialization, and packaging.

**Key Types**:

#### `BytecodeFile`
```rust
pub struct BytecodeFile {
    pub header: BytecodeHeader,
    pub bytecode: Vec<SerializableOpCode>,
}
```

**Fields**:
- `header: BytecodeHeader` - Metadata and magic number
- `bytecode: Vec<SerializableOpCode>` - Serializable instruction sequence

#### `BytecodeHeader`
```rust
pub struct BytecodeHeader {
    pub magic_number: [u8; 4], // "RSEL"
    pub mod_name: String,
    pub mod_author: String,
}
```

**Fields**:
- `magic_number: [u8; 4]` - File format identifier ("RSEL")
- `mod_name: String` - Program name
- `mod_author: String` - Author name

#### `SerializableOpCode`
```rust
pub enum SerializableOpCode {
    Push(SerializableValue),
    Pop, Dup, Swap,
    Add, Sub, Mul, Div, Mod,
    And, Or, Not,
    Eq, Ne, Lt, Le, Gt, Ge,
    Jump(usize), JumpIf(usize), JumpIfNot(usize),
    Call(usize), Return,
    Load(usize), Store(usize),
    ArrayNew, ArrayGet, ArraySet, ArrayLen,
    ToInt, ToFloat, ToString, ToBool,
    Print, Halt, Nop,
}
```

**Purpose**: Serializable version of `sel::OpCode` for bytecode storage.

#### `SerializableValue`
```rust
pub enum SerializableValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<SerializableValue>),
    Object(HashMap<String, SerializableValue>),
}
```

**Purpose**: Serializable version of `sel::Value` for bytecode storage.

**Key Functions**:

#### `pack_bytecode(opcodes: Vec<OpCode>, mod_name: String, mod_author: String) -> Vec<u8>`

**Purpose**: Packages opcodes into binary bytecode format.

**Parameters**:
- `opcodes: Vec<OpCode>` - Compiled instruction sequence
- `mod_name: String` - Program name
- `mod_author: String` - Author name

**Returns**: Binary bytecode ready for file output

**Process**:
1. Convert `OpCode` to `SerializableOpCode`
2. Create `BytecodeFile` with header
3. Serialize using bincode v2
4. Return binary data

#### `read_bytecode(data: Vec<u8>) -> Result<BytecodeFile, String>`

**Purpose**: Deserializes binary bytecode into structured format.

**Parameters**:
- `data: Vec<u8>` - Binary bytecode data

**Returns**: Deserialized bytecode file or error message

#### `extract_opcodes(bytecode_file: BytecodeFile) -> Vec<OpCode>`

**Purpose**: Extracts VM-compatible opcodes from bytecode file.

**Parameters**:
- `bytecode_file: BytecodeFile` - Deserialized bytecode

**Returns**: Vector of opcodes ready for VM execution

## Compilation Pipeline

### Stage 1: Preprocessing
```
Raw Source Code
      │
      ▼
┌─────────────┐
│ Preprocessor│
├─────────────┤
│ • #define   │
│ • Comments  │
│ • Includes  │
└─────────────┘
      │
      ▼
Clean Source Code
```

### Stage 2: Parsing
```
Clean Source Code
      │
      ▼
┌─────────────┐
│   Parser    │
├─────────────┤
│ • Lexing    │
│ • Syntax    │
│ • AST Build │
└─────────────┘
      │
      ▼
Abstract Syntax Tree
```

### Stage 3: Code Generation
```
Abstract Syntax Tree
      │
      ▼
┌─────────────┐
│  Assembler  │
├─────────────┤
│ • OpCode    │
│   Generation│
│ • Label     │
│   Resolution│
└─────────────┘
      │
      ▼
OpCode Sequence
```

### Stage 4: Packaging
```
OpCode Sequence
      │
      ▼
┌─────────────┐
│  Packager   │
├─────────────┤
│ • Serialize │
│ • Header    │
│ • Binary    │
│   Format    │
└─────────────┘
      │
      ▼
Bytecode File (.selb)
```

## Error Handling

### Error Categories

1. **Preprocessor Errors**:
   - Invalid `#define` syntax
   - Circular macro references
   - File I/O errors

2. **Parser Errors**:
   - Invalid instruction syntax
   - Missing semicolons
   - Unknown instructions
   - Invalid value formats

3. **Assembler Errors**:
   - Undefined label references
   - Invalid jump targets
   - Type conversion errors

### Error Reporting

- **Ariadne Integration**: Visual error reports with source highlighting
- **Span Information**: Precise error locations in source code
- **Contextual Messages**: Detailed explanations and suggestions
- **Multiple Errors**: Collects and reports all errors in single pass

## Memory Model

### Variable Storage
- Variables are identified by numeric indices (0, 1, 2, ...)
- No variable name resolution (assembly-level programming)
- Storage managed by VM runtime

### Label Resolution
- Labels are resolved to absolute addresses during compilation
- Forward references supported
- Circular references detected and reported

### Stack Operations
- All arithmetic and logic operations use stack-based evaluation
- PUSH/POP for explicit stack manipulation
- DUP/SWAP for stack reorganization

## Performance Characteristics

### Compilation Speed
- Linear time complexity for most operations
- Single-pass parsing and code generation
- Efficient label resolution with hash maps

### Memory Usage
- AST nodes allocated on heap
- Minimal memory copying during compilation
- Streaming serialization for large programs

### Error Recovery
- Continues parsing after errors when possible
- Collects multiple errors for batch reporting
- Graceful degradation on malformed input
