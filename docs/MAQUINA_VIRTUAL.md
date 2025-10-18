# La Máquina Virtual Sel - Guía Completa

## ¿Qué es una Máquina Virtual?

Una máquina virtual (VM) es un programa que simula una computadora completa. En el caso de Sel, la VM:

- **Ejecuta bytecode** compilado por Chisel
- **Gestiona memoria** (stack, variables, programa)
- **Procesa instrucciones** una por una
- **Maneja errores** durante la ejecución

## Arquitectura de la VM Sel

```
┌─────────────────────────────────────────────────────────────┐
│                  Máquina Virtual Sel                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   MEMORIA   │  │  PROGRAMA   │  │  EJECUCIÓN  │         │
│  │             │  │             │  │             │         │
│  │ Stack       │  │ Contador    │  │ Motor de    │         │
│  │ Variables   │  │ Programa    │  │ Instruc.    │         │
│  │ Call Stack  │  │ (PC)        │  │             │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Componentes Principales

### 1. Stack (Pila de Ejecución)

La pila es donde se almacenan temporalmente los valores durante las operaciones:

```
┌─────────────────┐ ← Tope (TOS - Top of Stack)
│     Valor N     │   Último valor agregado
├─────────────────┤
│     Valor 2     │
├─────────────────┤
│     Valor 1     │
├─────────────────┤
│     Valor 0     │ ← Fondo (primer valor)
└─────────────────┘
```

**Operaciones básicas:**
- `PUSH`: Agregar valor al tope
- `POP`: Quitar valor del tope
- `DUP`: Duplicar valor del tope
- `SWAP`: Intercambiar dos valores del tope

**Ejemplo práctico:**
```sel
PUSH 10;           // Stack: [10]
PUSH 5;            // Stack: [10, 5]
ADD;               // Stack: [15] (10 + 5)
DUP;               // Stack: [15, 15]
SWAP;              // Stack: [15, 15] (no cambia porque son iguales)
```

### 2. Variables (Almacenamiento Persistente)

Las variables se almacenan por índice numérico y persisten durante toda la ejecución:

```
Variables:
┌─────┬─────────────┐
│  0  │    100      │ ← Variable 0 contiene 100
├─────┼─────────────┤
│  1  │   "hola"    │ ← Variable 1 contiene "hola"
├─────┼─────────────┤
│  2  │    true     │ ← Variable 2 contiene true
├─────┼─────────────┤
│ ... │    ...      │
└─────┴─────────────┘
```

**Operaciones:**
- `STORE <índice>`: Guardar valor del stack en variable
- `LOAD <índice>`: Cargar variable al stack

**Ejemplo:**
```sel
PUSH 42;           // Stack: [42]
STORE 0;           // Stack: [], Variable[0] = 42

PUSH 100;          // Stack: [100]
LOAD 0;            // Stack: [100, 42]
ADD;               // Stack: [142]
```

### 3. Contador de Programa (PC)

El PC indica qué instrucción se ejecutará a continuación:

```
Programa: [PUSH 10] [PUSH 5] [ADD] [PRINT] [HALT]
Índices:      0        1       2      3       4
PC: ────────▲
```

**Flujo normal:**
1. Ejecutar instrucción en PC
2. PC = PC + 1
3. Repetir hasta HALT

**Saltos:**
```sel
0: PUSH 10;
1: JUMPIF 4;       // Si verdadero, PC = 4
2: PUSH 20;        // Se salta si el salto ocurre
3: PRINT;
4: HALT;           // PC salta aquí
```

### 4. Call Stack (Pila de Llamadas)

Para funciones futuras, almacena direcciones de retorno:

```
Call Stack:
┌─────────────────┐ ← Llamada actual
│  Retorno a 15   │
├─────────────────┤
│  Retorno a 8    │
├─────────────────┤
│  Retorno a 0    │ ← Llamada inicial
└─────────────────┘
```

## Sistema de Tipos

### Tipos Soportados

```rust
pub enum Value {
    Integer(i64),                    // Enteros: 42, -100
    Float(f64),                      // Flotantes: 3.14, -2.5
    Boolean(bool),                   // Booleanos: true, false
    String(String),                  // Cadenas: "hola mundo"
    Array(Vec<Value>),               // Arrays: [1, 2, 3]
    Object(HashMap<String, Value>),  // Objetos: {"key": "value"}
    Null,                           // Nulo: null
}
```

### Conversiones Automáticas

**En operaciones aritméticas:**
```sel
PUSH 10;           // Integer(10)
PUSH 3.5;          // Float(3.5)
ADD;               // Float(13.5) - Integer se convierte a Float
```

**En comparaciones:**
```sel
PUSH 10;           // Integer(10)
PUSH 10.0;         // Float(10.0)
EQ;                // Boolean(true) - Se comparan como números
```

### Truthiness (Veracidad)

Reglas para determinar si un valor es "verdadero":

```rust
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(b) => *b,           // true → true, false → false
        Value::Integer(i) => *i != 0,      // 0 → false, otros → true
        Value::Float(f) => *f != 0.0,      // 0.0 → false, otros → true
        Value::String(s) => !s.is_empty(), // "" → false, otros → true
        Value::Array(a) => !a.is_empty(),  // [] → false, otros → true
        Value::Object(o) => !o.is_empty(), // {} → false, otros → true
        Value::Null => false,              // null → false
    }
}
```

## Conjunto de Instrucciones Completo

### Stack Operations (Operaciones de Pila)

#### `PUSH <valor>`
Agrega un valor al tope de la pila.

```sel
PUSH 42;           // Stack: [42]
PUSH "hola";       // Stack: [42, "hola"]
PUSH true;         // Stack: [42, "hola", true]
```

#### `POP`
Quita y descarta el valor del tope.

```sel
// Stack inicial: [42, "hola", true]
POP;               // Stack: [42, "hola"]
```

#### `DUP`
Duplica el valor del tope.

```sel
// Stack inicial: [42, "hola"]
DUP;               // Stack: [42, "hola", "hola"]
```

#### `SWAP`
Intercambia los dos valores del tope.

```sel
// Stack inicial: [42, "hola"]
SWAP;              // Stack: ["hola", 42]
```

### Arithmetic Operations (Operaciones Aritméticas)

#### `ADD`
Suma los dos valores del tope.

```sel
PUSH 10;
PUSH 5;
ADD;               // Stack: [15]

PUSH "Hola ";
PUSH "mundo";
ADD;               // Stack: ["Hola mundo"] (concatenación)
```

#### `SUB`
Resta el tope del segundo valor.

```sel
PUSH 10;           // Stack: [10]
PUSH 3;            // Stack: [10, 3]
SUB;               // Stack: [7] (10 - 3)
```

#### `MUL`
Multiplica los dos valores del tope.

```sel
PUSH 6;
PUSH 7;
MUL;               // Stack: [42]
```

#### `DIV`
Divide el segundo valor por el tope.

```sel
PUSH 15;
PUSH 3;
DIV;               // Stack: [5] (15 / 3)

PUSH 10;
PUSH 0;
DIV;               // ERROR: DivisionByZero
```

### Comparison Operations (Operaciones de Comparación)

#### `EQ` (Igual)
```sel
PUSH 5;
PUSH 5;
EQ;                // Stack: [true]

PUSH "a";
PUSH "b";
EQ;                // Stack: [false]
```

#### `LT` (Menor que)
```sel
PUSH 3;
PUSH 5;
LT;                // Stack: [true] (3 < 5)
```

#### `GT` (Mayor que)
```sel
PUSH 10;
PUSH 5;
GT;                // Stack: [true] (10 > 5)
```

#### `LE` (Menor o igual)
```sel
PUSH 5;
PUSH 5;
LE;                // Stack: [true] (5 <= 5)
```

#### `GE` (Mayor o igual)
```sel
PUSH 7;
PUSH 5;
GE;                // Stack: [true] (7 >= 5)
```

### Logical Operations (Operaciones Lógicas)

#### `AND`
```sel
PUSH true;
PUSH false;
AND;               // Stack: [false]

PUSH 5;            // truthy
PUSH 0;            // falsy
AND;               // Stack: [false]
```

#### `OR`
```sel
PUSH false;
PUSH true;
OR;                // Stack: [true]
```

#### `NOT`
```sel
PUSH true;
NOT;               // Stack: [false]

PUSH 0;
NOT;               // Stack: [true] (0 es falsy)
```

### Control Flow (Control de Flujo)

#### `JUMP <dirección>`
Salto incondicional.

```sel
0: PUSH 1;
1: JUMP 4;         // Salta a la instrucción 4
2: PUSH 2;         // Esta línea se salta
3: PUSH 3;         // Esta línea se salta
4: PRINT;          // Imprime: 1
```

#### `JUMPIF <dirección>`
Salta si el tope de la pila es verdadero.

```sel
PUSH true;
JUMPIF 10;         // Salta a la instrucción 10

PUSH false;
JUMPIF 20;         // NO salta, continúa normalmente
```

#### `JUMPIFNOT <dirección>`
Salta si el tope de la pila es falso.

```sel
PUSH false;
JUMPIFNOT 10;      // Salta a la instrucción 10

PUSH true;
JUMPIFNOT 20;      // NO salta, continúa normalmente
```

### Variable Operations (Operaciones de Variables)

#### `STORE <índice>`
Guarda el tope de la pila en una variable.

```sel
PUSH 100;
STORE 0;           // Variable[0] = 100, Stack: []

PUSH "datos";
STORE 5;           // Variable[5] = "datos"
```

#### `LOAD <índice>`
Carga una variable a la pila.

```sel
LOAD 0;            // Stack: [Variable[0]]
LOAD 5;            // Stack: [Variable[0], Variable[5]]
```

### I/O Operations (Entrada/Salida)

#### `PRINT`
Imprime el tope de la pila y lo quita.

```sel
PUSH "¡Hola mundo!";
PRINT;             // Imprime: ¡Hola mundo!
                   // Stack: []
```

### Control Operations (Operaciones de Control)

#### `HALT`
Detiene la ejecución del programa.

```sel
PUSH 42;
PRINT;
HALT;              // El programa termina aquí
PUSH 100;          // Esta línea nunca se ejecuta
```

#### `NOP`
No hace nada (No Operation).

```sel
PUSH 1;
NOP;               // No hace nada
PRINT;             // Imprime: 1
```

## Ciclo de Ejecución

### 1. Inicialización
```rust
let mut vm = VirtualMachine::new(4096); // Stack de 4KB
vm.load_program(opcodes);               // Cargar programa
```

### 2. Bucle Principal
```rust
while vm.is_running() && pc < program.len() {
    let instruction = program[pc];      // Fetch (obtener)
    execute_instruction(instruction);   // Execute (ejecutar)
    // PC se actualiza automáticamente
}
```

### 3. Ejecución de Instrucción
```rust
fn execute_instruction(instruction: OpCode) -> Result<()> {
    match instruction {
        OpCode::Push(value) => {
            memory.push(value)?;        // Puede fallar por overflow
            advance_pc();               // PC = PC + 1
        }
        OpCode::Add => {
            let b = memory.pop()?;      // Puede fallar por underflow
            let a = memory.pop()?;
            let result = add_values(a, b)?; // Puede fallar por tipos
            memory.push(result)?;
            advance_pc();
        }
        OpCode::Jump(addr) => {
            jump_to(addr)?;             // PC = addr (sin +1)
        }
        // ... otras instrucciones
    }
}
```

## Manejo de Errores

### Tipos de Errores

```rust
pub enum VMError {
    StackUnderflow,        // Intentar POP en stack vacío
    StackOverflow,         // Stack lleno
    InvalidInstruction,    // Instrucción desconocida
    DivisionByZero,        // División por cero
    TypeMismatch(String),  // Tipos incompatibles
    IndexOutOfBounds,      // Índice de array inválido
    InvalidAddress(usize), // Dirección de salto inválida
    CallStackUnderflow,    // RETURN sin CALL
    CallStackOverflow,     // Demasiadas llamadas anidadas
}
```

### Ejemplos de Errores

#### Stack Underflow
```sel
POP;               // ERROR: Stack vacío
```

#### Type Mismatch
```sel
PUSH "texto";
PUSH 5;
ADD;               // ERROR: No se puede sumar string + number
```

#### Division by Zero
```sel
PUSH 10;
PUSH 0;
DIV;               // ERROR: División por cero
```

#### Invalid Address
```sel
JUMP 1000;         // ERROR: Dirección fuera del programa
```

## Optimizaciones y Rendimiento

### Gestión de Memoria

**Stack Size**: Configurable al crear la VM
```rust
VirtualMachine::new(1024);    // Stack pequeño (1KB)
VirtualMachine::new(65536);   // Stack grande (64KB)
```

**Variable Storage**: HashMap para acceso O(1)
```rust
variables: HashMap<usize, Value>  // Acceso rápido por índice
```

### Optimizaciones de Instrucciones

**Arithmetic Operations**: Promoción de tipos eficiente
```rust
match (a, b) {
    (Integer(a), Integer(b)) => Integer(a + b),     // Rápido
    (Integer(a), Float(b)) => Float(a as f64 + b),  // Conversión
    (Float(a), Integer(b)) => Float(a + b as f64),  // Conversión
    (Float(a), Float(b)) => Float(a + b),           // Rápido
}
```

**Jump Operations**: Validación de direcciones
```rust
fn jump_to(&mut self, addr: usize) -> Result<()> {
    if addr >= self.program.len() {
        return Err(VMError::InvalidAddress(addr));
    }
    self.pc = addr;
    Ok(())
}
```

## Debugging y Introspección

### Estado de la VM

```rust
// Inspeccionar stack
println!("Stack: {:?}", vm.memory().stack);

// Ver variables
println!("Variables: {:?}", vm.memory().variables);

// Posición actual
println!("PC: {}", vm.get_pc());

// Instrucción actual
println!("Current: {:?}", vm.current_instruction());
```

### Tracing de Ejecución

```rust
fn execute_with_trace(&mut self) -> Result<()> {
    while self.is_running() {
        println!("PC: {} | Instruction: {:?}", self.pc, self.current_instruction());
        println!("Stack before: {:?}", self.memory.stack);
        
        self.step()?;
        
        println!("Stack after: {:?}", self.memory.stack);
        println!("---");
    }
    Ok(())
}
```

## Ejemplos Prácticos

### Calculadora Simple
```sel
// Leer dos números y sumarlos
PUSH 15;
PUSH 25;
ADD;
PRINT;             // Imprime: 40
HALT;
```

### Contador con Bucle
```sel
// Contar de 0 a 4
PUSH 0;
STORE 0;           // contador = 0

// Inicio del bucle (PC = 2)
LOAD 0;            // Cargar contador
DUP;               // Duplicar para comparación
PRINT;             // Imprimir valor actual

PUSH 1;
ADD;               // contador + 1
STORE 0;           // Guardar nuevo valor

LOAD 0;            // Cargar para comparación
PUSH 5;
LT;                // ¿contador < 5?
JUMPIF 2;          // Si sí, volver al bucle

HALT;              // Terminar
```

### Factorial Iterativo
```sel
// Calcular 5!
PUSH 5;
STORE 0;           // n = 5

PUSH 1;
STORE 1;           // resultado = 1

// Bucle (PC = 4)
LOAD 0;            // Cargar n
PUSH 0;
LE;                // ¿n <= 0?
JUMPIF 16;         // Si sí, terminar

LOAD 1;            // Cargar resultado
LOAD 0;            // Cargar n
MUL;               // resultado * n
STORE 1;           // Guardar nuevo resultado

LOAD 0;            // Cargar n
PUSH 1;
SUB;               // n - 1
STORE 0;           // Guardar nuevo n

JUMP 4;            // Volver al bucle

LOAD 1;            // Cargar resultado final
PRINT;             // Imprimir 120
HALT;
```

## Futuras Extensiones

### Arrays Dinámicos
```sel
PUSH 5;
ARRAYNEW;          // Crear array de 5 elementos
STORE 0;           // Guardar array en variable 0

LOAD 0;            // Cargar array
PUSH 0;            // Índice
PUSH 42;           // Valor
ARRAYSET;          // array[0] = 42

LOAD 0;            // Cargar array
PUSH 0;            // Índice
ARRAYGET;          // Obtener array[0]
PRINT;             // Imprime: 42
```

### Funciones y Llamadas
```sel
CALL función;      // Llamar función
// ... código principal ...
HALT;

// función: (PC = 10)
PUSH 100;
PRINT;
RETURN;            // Volver al punto de llamada
```

### Manejo de Excepciones
```sel
TRY;               // Iniciar bloque try
PUSH 10;
PUSH 0;
DIV;               // Puede lanzar excepción
CATCH;             // Manejar excepción
PUSH "Error de división";
PRINT;
ENDTRY;
```

---

Esta guía cubre todos los aspectos fundamentales de la Máquina Virtual Sel. ¡Ahora estás listo para entender, usar y contribuir al proyecto!
