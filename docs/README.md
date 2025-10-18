# Glyph Project Documentation

## Overview

Glyph is a complete compiler toolchain for the Sel programming language, consisting of three main components:

- **Glyph**: The main CLI application that orchestrates compilation and execution
- **Chisel**: The compiler frontend that parses Sel assembly and generates bytecode
- **Sel**: The virtual machine runtime that executes Sel bytecode

## Project Structure

```
glyph/
├── src/                    # Main CLI application
├── crates/
│   ├── chisel/            # Compiler frontend
│   └── sel/               # Virtual machine runtime
└── docs/                  # Documentation
    ├── glyph/            # CLI documentation
    ├── chisel/           # Compiler documentation
    └── sel/              # VM documentation
```

## Quick Start

1. **Build the project**:
   ```bash
   cargo build --release
   ```

2. **Compile a Sel program**:
   ```bash
   ./glyph -c program.sel -o program.selb
   ```

3. **Run the compiled program**:
   ```bash
   ./glyph -r program.selb
   ```

## Documentation Structure

- [Glyph CLI Documentation](./glyph/README.md) - Main application interface
- [Chisel Compiler Documentation](./chisel/README.md) - Compiler frontend
- [Sel VM Documentation](./sel/README.md) - Virtual machine runtime

## Contributing

Please read the individual component documentation for detailed contribution guidelines:

- [Glyph Contributing Guide](./glyph/CONTRIBUTING.md)
- [Chisel Contributing Guide](./chisel/CONTRIBUTING.md)
- [Sel Contributing Guide](./sel/CONTRIBUTING.md)

## Architecture Overview

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    Glyph    │───▶│   Chisel    │───▶│     Sel     │
│  CLI Tool   │    │  Compiler   │    │  VM Runtime │
└─────────────┘    └─────────────┘    └─────────────┘
      │                    │                  │
      │                    │                  │
   Commands            .sel files         .selb files
   & Options           Assembly           Bytecode
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
