use colored::*;
use dialoguer::Input;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn compile(source: &Path, output: &Path) {
    let start = Instant::now();

    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  📦 Chisel Compiler".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    // Pedir metadata
    let author_name: String = Input::new()
        .with_prompt("👤 Program's author name")
        .interact_text()
        .unwrap();

    let program_name: String = Input::new()
        .with_prompt("📝 Program's name")
        .interact_text()
        .unwrap();

    println!();
    println!(
        "{} {:?}",
        "⏳ Compiling".bright_yellow().bold(),
        source.file_name().unwrap_or_default()
    );

    // Leer el archivo fuente
    let bytes = match std::fs::read(source) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "{} {}",
                "✗ Error:".red().bold(),
                format!("No se pudo leer el archivo: {}", e).red()
            );
            std::process::exit(1);
        }
    };

    let source_str = match std::str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "{} {}",
                "✗ Error:".red().bold(),
                format!("El archivo no contiene UTF-8 válido: {}", e).red()
            );
            std::process::exit(1);
        }
    };

    // Compilar
    match chisel::compile_source(source_str, program_name.clone(), author_name.clone()) {
        Ok(bytecode) => {
            let bytecode_len = bytecode.len();

            // Escribir bytecode
            if let Err(e) = fs::write(output, &bytecode) {
                println!(
                    "{} {}",
                    "✗ Error:".red().bold(),
                    format!("No se pudo escribir el archivo: {}", e).red()
                );
                std::process::exit(1);
            }

            let duration = start.elapsed();

            println!();
            println!(
                "{}",
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green()
            );
            println!(
                "{} {}",
                "✓ Success!".green().bold(),
                "Compilación exitosa".green()
            );
            println!();
            println!("{} {}", "📦 Program:".cyan(), program_name.bright_white());
            println!("{} {}", "👤 Author:".cyan(), author_name.bright_white());
            println!(
                "{} {} -> {}",
                "📄 Output:".cyan(),
                source.display().to_string().bright_white(),
                output.display().to_string().bright_yellow()
            );
            println!(
                "{} {} bytes",
                "💾 Size:".cyan(),
                bytecode_len.to_string().bright_yellow()
            );
            println!(
                "{} {} ms",
                "⚡ Time:".cyan(),
                duration.as_millis().to_string().bright_yellow()
            );
            println!(
                "{}",
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_green()
            );
            println!();
        }
        Err(error) => {
            println!();
            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_red());
            println!(
                "{} {}",
                "✗ Compilation failed".red().bold(),
                "Con errores".red()
            );
            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_red());
            println!();

            error.report_with_ariadne(source_str, source.to_str().unwrap_or("<unknown>"));

            std::process::exit(1);
        }
    }
}
