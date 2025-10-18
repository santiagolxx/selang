# Sel Virtual Machine Documentation

## Overview

Sel is a stack-based virtual machine runtime designed for executing Sel bytecode. It provides a complete execution environment with memory management, instruction processing, and error handling.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Sel Virtual Machine                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Memory    │  │   Program   │  │  Execution  │         │
│  │  Manager    │  │   Counter   │  │   Engine    │         │
│  │             │  │             │  │             │         │
│  │ • Stack     │  │ • PC        │  │ • Dispatch  │         │
│  │ • Variables │  │ • Jump      │  │ • OpCode    │         │
│  │ • Call      │  │   Targets   │  │   Execution │         │
│  │   Stack     │  │             │  │ • Error     │         │
│  │             │  │             │  │   Handling  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Dependencies

### External Crates
- **std**: Standard library collections and I/O

### Internal Dependencies
- Self-contained with no external runtime dependencies

## Source Structure

```
src/
├── lib.rs              # Public API and re-exports
├── core/
│   ├── mod.rs          # Core module exports
│   ├── value.rs        # Value type system
│   ├── opcode.rs       # Instruction definitions
│   └── error.rs        # Error types and handling
├── memory/
│   ├── mod.rs          # Memory module exports
│   └── stack.rs        # Stack and variable management
└── vm/
    ├── mod.rs          # VM module exports
    ├── machine.rs      # Virtual machine implementation
    └── operations.rs   # Instruction implementations
```

## Module Documentation

### core/value.rs

**Purpose**: Defines the type system for Sel values.

#### `Value`
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}
```

**Type System**:
- **Integer**: 64-bit signed integers (`i64`)
- **Float**: 64-bit floating-point numbers (`f64`)
- **Boolean**: True/false values
- **String**: UTF-8 encoded strings
- **Array**: Dynamic arrays of values
- **Object**: Hash maps with string keys
- **Null**: Represents absence of value

**Key Methods**:

#### `is_truthy(&self) -> bool`
**Purpose**: Determines truthiness for conditional operations.

**Truthiness Rules**:
- `Boolean(false)` → false
- `Integer(0)` → false
- `Float(0.0)` → false
- `String("")` → false (empty string)
- `Array([])` → false (empty array)
- `Object({})` → false (empty object)
- `Null` → false
- All other values → true

#### `type_name(&self) -> &'static str`
**Purpose**: Returns human-readable type name for error messages.

**Return Values**:
- "integer", "float", "boolean", "string", "array", "object", "null"

#### `to_number(&self) -> Result<f64, String>`
**Purpose**: Converts value to numeric representation.

**Conversion Rules**:
- `Integer(i)` → `i as f64`
- `Float(f)` → `f`
- `Boolean(true)` → `1.0`
- `Boolean(false)` → `0.0`
- `String(s)` → Parse as number or error
- Other types → Error

### core/opcode.rs

**Purpose**: Defines the instruction set for the Sel virtual machine.

#### `OpCode`
```rust
#[derive(Debug, Clone)]
pub enum OpCode {
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
    Jump(usize),
    JumpIf(usize),
    JumpIfNot(usize),
    Call(usize),
    Return,

    // Variables
    Load(usize),
    Store(usize),

    // Array operations
    ArrayNew, ArrayGet, ArraySet, ArrayLen,

    // Type conversion
    ToInt, ToFloat, ToString, ToBool,

    // I/O
    Print,

    // Control
    Halt, Nop,
}
```

**Instruction Categories**:

**Stack Operations**:
- `Push(Value)` - Push value onto stack
- `Pop` - Remove and discard top stack element
- `Dup` - Duplicate top stack element
- `Swap` - Exchange top two stack elements

**Arithmetic Operations**:
- `Add` - Add top two stack elements
- `Sub` - Subtract (second - top)
- `Mul` - Multiply top two stack elements
- `Div` - Divide (second / top)
- `Mod` - Modulo (second % top)

**Logical Operations**:
- `And` - Logical AND of top two elements
- `Or` - Logical OR of top two elements
- `Not` - Logical NOT of top element

**Comparison Operations**:
- `Eq` - Test equality (==)
- `Ne` - Test inequality (!=)
- `Lt` - Test less than (<)
- `Le` - Test less than or equal (<=)
- `Gt` - Test greater than (>)
- `Ge` - Test greater than or equal (>=)

**Control Flow**:
- `Jump(usize)` - Unconditional jump to address
- `JumpIf(usize)` - Jump if top stack element is truthy
- `JumpIfNot(usize)` - Jump if top stack element is falsy
- `Call(usize)` - Function call with return address
- `Return` - Return from function call

**Variable Operations**:
- `Load(usize)` - Load variable by index onto stack
- `Store(usize)` - Store top stack element to variable

**Array Operations**:
- `ArrayNew` - Create new array (size from stack)
- `ArrayGet` - Get array element (array, index from stack)
- `ArraySet` - Set array element (array, index, value from stack)
- `ArrayLen` - Get array length

**Type Conversion**:
- `ToInt` - Convert top stack element to integer
- `ToFloat` - Convert top stack element to float
- `ToString` - Convert top stack element to string
- `ToBool` - Convert top stack element to boolean

**I/O Operations**:
- `Print` - Print top stack element to stdout

**Control Operations**:
- `Halt` - Stop program execution
- `Nop` - No operation (do nothing)

### core/error.rs

**Purpose**: Defines error types for VM operations.

#### `VMError`
```rust
#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    StackOverflow,
    InvalidInstruction,
    DivisionByZero,
    TypeMismatch(String),
    IndexOutOfBounds,
    InvalidAddress(usize),
    CallStackUnderflow,
    CallStackOverflow,
}
```

**Error Types**:
- `StackUnderflow` - Attempted to pop from empty stack
- `StackOverflow` - Stack exceeded maximum size
- `InvalidInstruction` - Unknown or malformed instruction
- `DivisionByZero` - Division or modulo by zero
- `TypeMismatch(String)` - Operation on incompatible types
- `IndexOutOfBounds` - Array access outside bounds
- `InvalidAddress(usize)` - Jump to invalid program address
- `CallStackUnderflow` - Return without matching call
- `CallStackOverflow` - Too many nested function calls

#### `Result<T>`
```rust
pub type Result<T> = std::result::Result<T, VMError>;
```

**Purpose**: Standard result type for VM operations.

### memory/stack.rs

**Purpose**: Implements memory management for the virtual machine.

#### `Memory`
```rust
pub struct Memory {
    stack: Vec<Value>,
    variables: HashMap<usize, Value>,
    call_stack: Vec<usize>,
    max_stack_size: usize,
    max_call_stack_size: usize,
}
```

**Fields**:
- `stack: Vec<Value>` - Main execution stack
- `variables: HashMap<usize, Value>` - Variable storage by index
- `call_stack: Vec<usize>` - Function call return addresses
- `max_stack_size: usize` - Maximum stack depth (default: 1024)
- `max_call_stack_size: usize` - Maximum call depth (default: 256)

**Key Methods**:

#### `new(max_stack_size: usize) -> Self`
**Purpose**: Creates new memory manager with specified stack size.

**Parameters**:
- `max_stack_size: usize` - Maximum number of stack elements

#### Stack Operations

#### `push(&mut self, value: Value) -> Result<()>`
**Purpose**: Pushes value onto execution stack.

**Error Conditions**:
- `StackOverflow` if stack exceeds maximum size

#### `pop(&mut self) -> Result<Value>`
**Purpose**: Pops and returns top stack element.

**Error Conditions**:
- `StackUnderflow` if stack is empty

#### `peek(&self) -> Result<&Value>`
**Purpose**: Returns reference to top stack element without removing.

**Error Conditions**:
- `StackUnderflow` if stack is empty

#### `dup(&mut self) -> Result<()>`
**Purpose**: Duplicates top stack element.

**Implementation**: Equivalent to `push(peek().clone())`

#### `swap(&mut self) -> Result<()>`
**Purpose**: Exchanges top two stack elements.

**Error Conditions**:
- `StackUnderflow` if stack has fewer than 2 elements

#### Variable Operations

#### `load_var(&self, id: usize) -> Result<Value>`
**Purpose**: Loads variable value by index.

**Parameters**:
- `id: usize` - Variable index

**Returns**: Cloned variable value or `Null` if undefined

#### `store_var(&mut self, id: usize, value: Value)`
**Purpose**: Stores value to variable by index.

**Parameters**:
- `id: usize` - Variable index
- `value: Value` - Value to store

#### Call Stack Operations

#### `push_call(&mut self, return_addr: usize) -> Result<()>`
**Purpose**: Pushes return address for function call.

**Parameters**:
- `return_addr: usize` - Address to return to

**Error Conditions**:
- `CallStackOverflow` if call stack exceeds maximum depth

#### `pop_call(&mut self) -> Result<usize>`
**Purpose**: Pops and returns return address.

**Error Conditions**:
- `CallStackUnderflow` if call stack is empty

#### Utility Methods

#### `stack_size(&self) -> usize`
**Purpose**: Returns current stack depth.

#### `is_empty(&self) -> bool`
**Purpose**: Checks if stack is empty.

#### `clear(&mut self)`
**Purpose**: Clears all stacks and variables.

### vm/machine.rs

**Purpose**: Core virtual machine implementation.

#### `VirtualMachine`
```rust
pub struct VirtualMachine {
    memory: Memory,
    program: Vec<OpCode>,
    pc: usize,
    running: bool,
}
```

**Fields**:
- `memory: Memory` - Memory manager instance
- `program: Vec<OpCode>` - Loaded program instructions
- `pc: usize` - Program counter (current instruction index)
- `running: bool` - Execution state flag

**Key Methods**:

#### `new(stack_size: usize) -> Self`
**Purpose**: Creates new VM instance.

**Parameters**:
- `stack_size: usize` - Maximum stack size in elements

#### `load_program(&mut self, program: Vec<OpCode>)`
**Purpose**: Loads program into VM memory.

**Parameters**:
- `program: Vec<OpCode>` - Instruction sequence to execute

**Side Effects**:
- Resets program counter to 0
- Sets running flag to false
- Clears existing program

#### `run(&mut self) -> Result<()>`
**Purpose**: Executes loaded program until completion.

**Execution Loop**:
1. Set running flag to true
2. While running and PC is valid:
   - Fetch instruction at PC
   - Execute instruction
   - Handle errors
3. Return result

**Error Handling**:
- Propagates instruction execution errors
- Validates program counter bounds
- Handles halt conditions

#### `step(&mut self) -> Result<()>`
**Purpose**: Executes single instruction.

**Returns**: Result of instruction execution

#### `execute_instruction(&mut self, instruction: &OpCode) -> Result<()>`
**Purpose**: Dispatches instruction to appropriate handler.

**Implementation**: Large match statement delegating to operations module

#### Program Counter Management

#### `get_pc(&self) -> usize`
**Purpose**: Returns current program counter value.

#### `advance_pc(&mut self)`
**Purpose**: Increments program counter by 1.

#### `jump_to(&mut self, addr: usize) -> Result<()>`
**Purpose**: Sets program counter to specified address.

**Parameters**:
- `addr: usize` - Target address

**Error Conditions**:
- `InvalidAddress` if address is outside program bounds

#### State Management

#### `is_running(&self) -> bool`
**Purpose**: Returns current execution state.

#### `stop(&mut self)`
**Purpose**: Sets running flag to false.

#### `memory(&mut self) -> &mut Memory`
**Purpose**: Returns mutable reference to memory manager.

#### Debugging Support

#### `current_instruction(&self) -> Option<&OpCode>`
**Purpose**: Returns current instruction for debugging.

#### `get_program(&self) -> &[OpCode]`
**Purpose**: Returns program slice for inspection.

### vm/operations.rs

**Purpose**: Implements individual instruction operations.

**Module Structure**: Each instruction has a dedicated function that takes `&mut VirtualMachine` and performs the operation.

#### Stack Operations

#### `push(vm: &mut VirtualMachine, value: Value) -> Result<()>`
**Purpose**: Pushes value onto stack.

**Implementation**:
1. Push value to memory stack
2. Advance program counter
3. Return result

#### `pop(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Removes top stack element.

**Implementation**:
1. Pop value from memory stack (discard result)
2. Advance program counter
3. Return result

#### `dup(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Duplicates top stack element.

**Implementation**:
1. Call memory.dup()
2. Advance program counter
3. Return result

#### `swap(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Swaps top two stack elements.

**Implementation**:
1. Call memory.swap()
2. Advance program counter
3. Return result

#### Arithmetic Operations

#### `add(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Adds top two stack elements.

**Implementation**:
1. Pop second operand (b)
2. Pop first operand (a)
3. Compute result based on types:
   - `Integer + Integer` → `Integer`
   - `Float + Float` → `Float`
   - `Integer + Float` → `Float` (promote integer)
   - `Float + Integer` → `Float` (promote integer)
   - `String + String` → `String` (concatenation)
   - Other combinations → `TypeMismatch` error
4. Push result onto stack
5. Advance program counter

#### `sub(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Subtracts top stack element from second.

**Type Promotion**: Same as addition, excluding string operations

#### `mul(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Multiplies top two stack elements.

**Type Promotion**: Same as addition, excluding string operations

#### `div(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Divides second stack element by top.

**Error Conditions**:
- `DivisionByZero` if divisor is zero
- `TypeMismatch` for incompatible types

#### `mod_op(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Computes modulo of second by top stack element.

**Note**: Function named `mod_op` to avoid Rust keyword conflict

#### Logical Operations

#### `and(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Logical AND of top two stack elements.

**Implementation**:
1. Pop two values
2. Evaluate truthiness of both
3. Push `Boolean(a.is_truthy() && b.is_truthy())`
4. Advance program counter

#### `or(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Logical OR of top two stack elements.

#### `not(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Logical NOT of top stack element.

#### Comparison Operations

#### `eq(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Tests equality of top two stack elements.

**Implementation**:
1. Pop two values
2. Compare using `Value::PartialEq`
3. Push `Boolean(a == b)`
4. Advance program counter

#### `ne(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Tests inequality of top two stack elements.

#### `lt(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Tests if second < top.

**Type Support**:
- Numeric types with promotion
- String lexicographic comparison
- Other types → `TypeMismatch`

#### `le(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Tests if second <= top.

#### `gt(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Tests if second > top.

#### `ge(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Tests if second >= top.

#### Control Flow Operations

#### `jump(vm: &mut VirtualMachine, addr: usize) -> Result<()>`
**Purpose**: Unconditional jump to address.

**Implementation**:
1. Call `vm.jump_to(addr)`
2. Return result (no PC advance)

#### `jump_if(vm: &mut VirtualMachine, addr: usize) -> Result<()>`
**Purpose**: Conditional jump if top stack element is truthy.

**Implementation**:
1. Pop condition value
2. If `condition.is_truthy()`:
   - Jump to address
3. Else:
   - Advance program counter
4. Return result

#### `jump_if_not(vm: &mut VirtualMachine, addr: usize) -> Result<()>`
**Purpose**: Conditional jump if top stack element is falsy.

#### `call(vm: &mut VirtualMachine, addr: usize) -> Result<()>`
**Purpose**: Function call with return address.

**Implementation**:
1. Calculate return address (PC + 1)
2. Push return address to call stack
3. Jump to target address
4. Return result

#### `ret(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Return from function call.

**Implementation**:
1. Pop return address from call stack
2. Jump to return address
3. Return result

#### Variable Operations

#### `load(vm: &mut VirtualMachine, id: usize) -> Result<()>`
**Purpose**: Load variable onto stack.

**Implementation**:
1. Load variable value by ID
2. Push value onto stack
3. Advance program counter
4. Return result

#### `store(vm: &mut VirtualMachine, id: usize) -> Result<()>`
**Purpose**: Store top stack element to variable.

**Implementation**:
1. Pop value from stack
2. Store value to variable ID
3. Advance program counter
4. Return result

#### Array Operations

#### `array_new(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Creates new array with size from stack.

**Implementation**:
1. Pop size value
2. Validate size is non-negative integer
3. Create array filled with `Null` values
4. Push array onto stack
5. Advance program counter

#### `array_get(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Gets array element by index.

**Implementation**:
1. Pop index value
2. Pop array value
3. Validate types and bounds
4. Push element value onto stack
5. Advance program counter

**Error Conditions**:
- `TypeMismatch` if not array/integer
- `IndexOutOfBounds` if index invalid

#### `array_set(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Sets array element by index.

**Implementation**:
1. Pop value to set
2. Pop index
3. Pop array
4. Validate types and bounds
5. Update array element
6. Push modified array back
7. Advance program counter

#### `array_len(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Gets array length.

#### Type Conversion Operations

#### `to_int(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Converts top stack element to integer.

**Conversion Rules**:
- `Integer(i)` → `Integer(i)` (no change)
- `Float(f)` → `Integer(f as i64)` (truncate)
- `Boolean(true)` → `Integer(1)`
- `Boolean(false)` → `Integer(0)`
- `String(s)` → Parse as integer or error
- Other types → `TypeMismatch`

#### `to_float(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Converts top stack element to float.

#### `to_string(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Converts top stack element to string.

**Uses**: `Value::Display` implementation for conversion

#### `to_bool(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Converts top stack element to boolean.

**Uses**: `Value::is_truthy()` for conversion

#### I/O Operations

#### `print(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Prints top stack element to stdout.

**Implementation**:
1. Pop value from stack
2. Print using `println!("{}", value)`
3. Advance program counter
4. Return result

#### Control Operations

#### `halt(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: Stops program execution.

**Implementation**:
1. Call `vm.stop()`
2. Return `Ok(())` (no PC advance)

#### `nop(vm: &mut VirtualMachine) -> Result<()>`
**Purpose**: No operation.

**Implementation**:
1. Advance program counter
2. Return `Ok(())`

## Memory Model

### Stack Layout
```
┌─────────────────┐ ← Top of Stack (TOS)
│     Value N     │
├─────────────────┤
│     Value N-1   │
├─────────────────┤
│       ...       │
├─────────────────┤
│     Value 1     │
├─────────────────┤
│     Value 0     │ ← Bottom of Stack
└─────────────────┘
```

### Variable Storage
- Variables stored in hash map by numeric index
- No scope or lifetime management
- Global namespace for all variables
- Uninitialized variables return `Null`

### Call Stack
```
┌─────────────────┐ ← Current Call
│  Return Addr N  │
├─────────────────┤
│ Return Addr N-1 │
├─────────────────┤
│       ...       │
├─────────────────┤
│  Return Addr 1  │
├─────────────────┤
│  Return Addr 0  │ ← Initial Call
└─────────────────┘
```

### Address Space
- Program counter addresses instructions sequentially
- Jump targets must be valid instruction indices
- No memory protection or segmentation

## Execution Model

### Instruction Cycle
1. **Fetch**: Read instruction at program counter
2. **Decode**: Match instruction opcode
3. **Execute**: Call appropriate operation function
4. **Update**: Advance program counter (unless jump)
5. **Repeat**: Continue until halt or error

### Error Propagation
- Errors bubble up through call stack
- No exception handling or recovery
- Program terminates on first error

### Performance Characteristics
- **Time Complexity**: O(1) per instruction
- **Space Complexity**: O(n) for stack and variables
- **Memory Usage**: Minimal overhead per value

## Type System

### Dynamic Typing
- Values carry type information at runtime
- Type checking performed during operations
- Automatic type promotion for numeric operations

### Type Coercion Rules
1. **Arithmetic**: Integers promote to floats
2. **Comparison**: Same-type comparison preferred
3. **Logical**: All values have truthiness
4. **String**: Concatenation for addition only

### Memory Management
- Reference counting for complex types
- Automatic cleanup when values go out of scope
- No garbage collection needed

## Debugging and Introspection

### VM State Access
- Program counter inspection
- Stack contents examination
- Variable value lookup
- Call stack traversal

### Error Context
- Instruction-level error reporting
- Stack trace information
- Type mismatch details
- Memory state at error

### Performance Monitoring
- Instruction count tracking
- Stack depth monitoring
- Memory usage statistics
- Execution time measurement
