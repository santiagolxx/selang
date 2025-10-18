use clap::Parser;
use std::path::PathBuf;

mod compiler;
mod info;
mod runner;

/// CLI tool for building and running projects independently
#[derive(Parser, Debug)]
#[command(
    name = "glyph",
    version,
    about = "A minimal CLI for the Sel Virtual Machine"
)]
struct Args {
    #[arg(short = 'c', long = "compile")]
    compile_file: Option<PathBuf>,

    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    #[arg(short = 'r', long = "run")]
    run_file: Option<PathBuf>,

    #[arg(short = 'i', long = "info", default_value_t = false)]
    check_info: bool,

    #[arg(short = 's', long = "stack", default_value = "4KiB")]
    stack: String,
}

pub fn cli_main() {
    let args = Args::parse();

    if args.check_info {
        info::print_info();
    }

    if let Some(ref source) = args.compile_file {
        let output = args.output.clone().unwrap_or_else(|| "a.sel".into());
        compiler::compile(&source, &output);
    }

    if let Some(ref run_path) = args.run_file {
        runner::run(&run_path, &args.stack);
    }

    // Si no se pasó ningún argumento relevante:
    if !args.check_info && args.compile_file.is_none() && args.run_file.is_none() {
        eprintln!("No command given. Use -c <file> to compile, -r <file> to run, or -i for info.");
    }
}
