use crate::{Result, VMError, Value, VirtualMachine};

// ==================== Stack Operations ====================

/// Empuja un valor al stack
pub fn push(vm: &mut VirtualMachine, value: Value) -> Result<()> {
    vm.memory().push(value)?;
    vm.advance_pc();
    Ok(())
}

/// Saca un valor del stack
pub fn pop(vm: &mut VirtualMachine) -> Result<()> {
    vm.memory().pop()?;
    vm.advance_pc();
    Ok(())
}

/// Duplica el valor en la cima del stack
pub fn dup(vm: &mut VirtualMachine) -> Result<()> {
    vm.memory().dup()?;
    vm.advance_pc();
    Ok(())
}

/// Intercambia los dos valores en la cima del stack
pub fn swap(vm: &mut VirtualMachine) -> Result<()> {
    vm.memory().swap()?;
    vm.advance_pc();
    Ok(())
}

// ==================== Arithmetic Operations ====================

/// Suma dos valores del stack
pub fn add(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
        (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 + b),
        (Value::Float(a), Value::Integer(b)) => Value::Float(a + b as f64),
        (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot add {} and {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Resta dos valores del stack
pub fn sub(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
        (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 - b),
        (Value::Float(a), Value::Integer(b)) => Value::Float(a - b as f64),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot subtract {} from {}",
                b.type_name(),
                a.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Multiplica dos valores del stack
pub fn mul(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
        (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 * b),
        (Value::Float(a), Value::Integer(b)) => Value::Float(a * b as f64),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot multiply {} and {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Divide dos valores del stack
pub fn div(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => {
            if b == 0 {
                return Err(VMError::DivisionByZero);
            }
            Value::Integer(a / b)
        }
        (Value::Float(a), Value::Float(b)) => {
            if b == 0.0 {
                return Err(VMError::DivisionByZero);
            }
            Value::Float(a / b)
        }
        (Value::Integer(a), Value::Float(b)) => {
            if b == 0.0 {
                return Err(VMError::DivisionByZero);
            }
            Value::Float(a as f64 / b)
        }
        (Value::Float(a), Value::Integer(b)) => {
            if b == 0 {
                return Err(VMError::DivisionByZero);
            }
            Value::Float(a / b as f64)
        }
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot divide {} by {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Calcula el módulo de dos valores del stack
pub fn modulo(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => {
            if b == 0 {
                return Err(VMError::DivisionByZero);
            }
            Value::Integer(a % b)
        }
        (Value::Float(a), Value::Float(b)) => {
            if b == 0.0 {
                return Err(VMError::DivisionByZero);
            }
            Value::Float(a % b)
        }
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot modulo {} by {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

// ==================== Logic Operations ====================

/// AND lógico
pub fn and(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;
    let result = Value::Boolean(a.is_truthy() && b.is_truthy());
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// OR lógico
pub fn or(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;
    let result = Value::Boolean(a.is_truthy() || b.is_truthy());
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// NOT lógico
pub fn not(vm: &mut VirtualMachine) -> Result<()> {
    let a = vm.memory().pop()?;
    let result = Value::Boolean(!a.is_truthy());
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

// ==================== Comparison Operations ====================

/// Igualdad
pub fn eq(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;
    let result = Value::Boolean(a == b);
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Desigualdad
pub fn ne(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;
    let result = Value::Boolean(a != b);
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Menor que
pub fn lt(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a < b),
        (Value::Float(a), Value::Float(b)) => Value::Boolean(a < b),
        (Value::Integer(a), Value::Float(b)) => Value::Boolean((a as f64) < b),
        (Value::Float(a), Value::Integer(b)) => Value::Boolean(a < (b as f64)),
        (Value::String(a), Value::String(b)) => Value::Boolean(a < b),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot compare {} and {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Menor o igual que
pub fn le(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a <= b),
        (Value::Float(a), Value::Float(b)) => Value::Boolean(a <= b),
        (Value::Integer(a), Value::Float(b)) => Value::Boolean((a as f64) <= b),
        (Value::Float(a), Value::Integer(b)) => Value::Boolean(a <= (b as f64)),
        (Value::String(a), Value::String(b)) => Value::Boolean(a <= b),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot compare {} and {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Mayor que
pub fn gt(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a > b),
        (Value::Float(a), Value::Float(b)) => Value::Boolean(a > b),
        (Value::Integer(a), Value::Float(b)) => Value::Boolean((a as f64) > b),
        (Value::Float(a), Value::Integer(b)) => Value::Boolean(a > (b as f64)),
        (Value::String(a), Value::String(b)) => Value::Boolean(a > b),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot compare {} and {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Mayor o igual que
pub fn ge(vm: &mut VirtualMachine) -> Result<()> {
    let b = vm.memory().pop()?;
    let a = vm.memory().pop()?;

    let result = match (a, b) {
        (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a >= b),
        (Value::Float(a), Value::Float(b)) => Value::Boolean(a >= b),
        (Value::Integer(a), Value::Float(b)) => Value::Boolean((a as f64) >= b),
        (Value::Float(a), Value::Integer(b)) => Value::Boolean(a >= (b as f64)),
        (Value::String(a), Value::String(b)) => Value::Boolean(a >= b),
        (a, b) => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot compare {} and {}",
                a.type_name(),
                b.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

// ==================== Control Flow Operations ====================

/// Salto incondicional
pub fn jump(vm: &mut VirtualMachine, addr: usize) -> Result<()> {
    vm.jump_to(addr)
}

/// Salto condicional (si verdadero)
pub fn jump_if(vm: &mut VirtualMachine, addr: usize) -> Result<()> {
    let condition = vm.memory().pop()?;
    if condition.is_truthy() {
        vm.jump_to(addr)
    } else {
        vm.advance_pc();
        Ok(())
    }
}

/// Salto condicional (si falso)
pub fn jump_if_not(vm: &mut VirtualMachine, addr: usize) -> Result<()> {
    let condition = vm.memory().pop()?;
    if !condition.is_truthy() {
        vm.jump_to(addr)
    } else {
        vm.advance_pc();
        Ok(())
    }
}

pub fn call(vm: &mut VirtualMachine, addr: usize) -> Result<()> {
    let return_addr = vm.get_pc() + 1;
    vm.memory().push_call(return_addr);
    vm.jump_to(addr)
}

/// Retorno de función
pub fn ret(vm: &mut VirtualMachine) -> Result<()> {
    let return_addr = vm.memory().pop_call()?;
    vm.jump_to(return_addr)
}

// ==================== Variable Operations ====================

/// Carga una variable al stack
pub fn load(vm: &mut VirtualMachine, id: usize) -> Result<()> {
    let value = vm.memory().load_var(id)?;
    vm.memory().push(value)?;
    vm.advance_pc();
    Ok(())
}

/// Almacena un valor del stack en una variable
pub fn store(vm: &mut VirtualMachine, id: usize) -> Result<()> {
    let value = vm.memory().pop()?;
    vm.memory().store_var(id, value);
    vm.advance_pc();
    Ok(())
}

// ==================== Array Operations ====================

/// Crea un nuevo array
pub fn array_new(vm: &mut VirtualMachine) -> Result<()> {
    let size = vm.memory().pop()?;
    match size {
        Value::Integer(n) if n >= 0 => {
            let array = vec![Value::Null; n as usize];
            vm.memory().push(Value::Array(array))?;
            vm.advance_pc();
            Ok(())
        }
        _ => Err(VMError::TypeMismatch(
            "Array size must be non-negative integer".to_string(),
        )),
    }
}

/// Obtiene un elemento del array
pub fn array_get(vm: &mut VirtualMachine) -> Result<()> {
    let index = vm.memory().pop()?;
    let array = vm.memory().pop()?;

    match (array, index) {
        (Value::Array(arr), Value::Integer(i)) => {
            if i < 0 || i as usize >= arr.len() {
                return Err(VMError::IndexOutOfBounds);
            }
            vm.memory().push(arr[i as usize].clone())?;
            vm.advance_pc();
            Ok(())
        }
        (a, i) => Err(VMError::TypeMismatch(format!(
            "Cannot index {} with {}",
            a.type_name(),
            i.type_name()
        ))),
    }
}

/// Establece un elemento del array
pub fn array_set(vm: &mut VirtualMachine) -> Result<()> {
    let value = vm.memory().pop()?;
    let index = vm.memory().pop()?;
    let mut array = vm.memory().pop()?;

    match (&mut array, index) {
        (Value::Array(arr), Value::Integer(i)) => {
            if i < 0 || i as usize >= arr.len() {
                return Err(VMError::IndexOutOfBounds);
            }
            arr[i as usize] = value;
            vm.memory().push(array)?;
            vm.advance_pc();
            Ok(())
        }
        (a, i) => Err(VMError::TypeMismatch(format!(
            "Cannot index {} with {}",
            a.type_name(),
            i.type_name()
        ))),
    }
}

/// Obtiene la longitud de un array o string
pub fn array_len(vm: &mut VirtualMachine) -> Result<()> {
    let array = vm.memory().pop()?;
    match array {
        Value::Array(arr) => {
            vm.memory().push(Value::Integer(arr.len() as i64))?;
            vm.advance_pc();
            Ok(())
        }
        Value::String(s) => {
            vm.memory().push(Value::Integer(s.len() as i64))?;
            vm.advance_pc();
            Ok(())
        }
        a => Err(VMError::TypeMismatch(format!(
            "Cannot get length of {}",
            a.type_name()
        ))),
    }
}

// ==================== Type Conversion Operations ====================

/// Convierte a entero
pub fn to_int(vm: &mut VirtualMachine) -> Result<()> {
    let value = vm.memory().pop()?;
    let result = match value {
        Value::Integer(i) => Value::Integer(i),
        Value::Float(f) => Value::Integer(f as i64),
        Value::Boolean(b) => Value::Integer(if b { 1 } else { 0 }),
        Value::String(s) => match s.parse::<i64>() {
            Ok(i) => Value::Integer(i),
            Err(_) => {
                return Err(VMError::TypeMismatch(format!(
                    "Cannot convert '{}' to integer",
                    s
                )));
            }
        },
        v => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot convert {} to integer",
                v.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Convierte a flotante
pub fn to_float(vm: &mut VirtualMachine) -> Result<()> {
    let value = vm.memory().pop()?;
    let result = match value {
        Value::Integer(i) => Value::Float(i as f64),
        Value::Float(f) => Value::Float(f),
        Value::Boolean(b) => Value::Float(if b { 1.0 } else { 0.0 }),
        Value::String(s) => match s.parse::<f64>() {
            Ok(f) => Value::Float(f),
            Err(_) => {
                return Err(VMError::TypeMismatch(format!(
                    "Cannot convert '{}' to float",
                    s
                )));
            }
        },
        v => {
            return Err(VMError::TypeMismatch(format!(
                "Cannot convert {} to float",
                v.type_name()
            )));
        }
    };

    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Convierte a string
pub fn to_string(vm: &mut VirtualMachine) -> Result<()> {
    let value = vm.memory().pop()?;
    let result = Value::String(value.to_string());
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

/// Convierte a booleano
pub fn to_bool(vm: &mut VirtualMachine) -> Result<()> {
    let value = vm.memory().pop()?;
    let result = Value::Boolean(value.is_truthy());
    vm.memory().push(result)?;
    vm.advance_pc();
    Ok(())
}

// ==================== I/O Operations ====================

/// Imprime un valor
pub fn print(vm: &mut VirtualMachine) -> Result<()> {
    let value = vm.memory().pop()?;
    println!("{}", value);
    vm.advance_pc();
    Ok(())
}

// ==================== Control Operations ====================

/// Detiene la ejecución
pub fn halt(vm: &mut VirtualMachine) -> Result<()> {
    vm.stop();
    Ok(())
}

/// No operation (no hace nada)
pub fn nop(vm: &mut VirtualMachine) -> Result<()> {
    vm.advance_pc();
    Ok(())
}
