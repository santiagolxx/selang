use chisel::{extract_opcodes, read_bytecode};
use sel::vm::VirtualMachine;
use std::fs;
use std::path::Path;

pub fn run(input: &Path, memory_size: &str) {
    println!(
        r##"
        ____            ___
       /\  _`\         /\_ \
       \ \,\L\_\     __\//\ \
        \/_\__ \   /'__`\\ \ \
          /\ \L\ \/\  __/ \_\ \_
          \ `\____\ \____\/\____\
           \/_____/\/____/\/____/
        "##
    );
    println!("The Sel Virtual Machine");
    let memory_bytes =
        parse_size(memory_size).expect("Formato de memoria invalido como argumento.");
    println!("Starting VM Instance with {} Stack", memory_size);

    let mut vm_instance = VirtualMachine::new(memory_bytes);
    let file = read_bytecode(fs::read(input).expect("cannot read file"));
    match file {
        Ok(bytecode_file) => {
            let opcodes = extract_opcodes(bytecode_file);
            vm_instance.load_program(opcodes);
        }
        Err(error) => {
            eprintln!("Error reading bytecode: {}", error);
            std::process::exit(1);
        }
    }
    vm_instance.run().expect("Error while running the program.");
}

fn parse_size(size_str: &str) -> Result<usize, String> {
    let size_str = size_str.trim().to_lowercase();
    let mut number_part = String::new();
    let mut unit_part = String::new();

    // Separar la parte numérica de la unidad
    for c in size_str.chars() {
        if c.is_digit(10) || c == '.' {
            if !unit_part.is_empty() {
                return Err("Formato inválido: número después de unidad".to_string());
            }
            number_part.push(c);
        } else if !c.is_whitespace() {
            unit_part.push(c);
        }
    }

    let number: f64 = number_part
        .parse()
        .map_err(|_| "Número inválido".to_string())?;

    let multiplier: usize = match unit_part.as_str() {
        "b" | "" => 1,
        "kb" => 1_000,
        "mb" => 1_000_000,
        "gb" => 1_000_000_000,
        "kib" => 1024,
        "mib" => 1024 * 1024,
        "gib" => 1024 * 1024 * 1024,
        _ => return Err(format!("Unidad desconocida '{}'", unit_part)),
    };

    let bytes_f64 = number * multiplier as f64;

    // Chequeo de integralidad y rango
    if bytes_f64.fract() != 0.0 {
        return Err("La cantidad de bytes no es un entero".to_string());
    }

    if bytes_f64 < 0.0 {
        return Err("El tamaño no puede ser negativo".to_string());
    }

    // Comprobar que cabe en usize
    if bytes_f64 > usize::MAX as f64 {
        return Err("El tamaño excede el límite de usize".to_string());
    }

    Ok(bytes_f64 as usize)
}
