# Contributing to Glyph CLI

## Overview

This guide covers contributing to the Glyph command-line interface, the main entry point for the Sel programming language toolchain.

## Development Setup

### Prerequisites
- Rust 1.70+ with 2024 edition support
- Git for version control
- Basic familiarity with CLI development

### Building from Source
```bash
git clone <repository-url>
cd glyph
cargo build
```

### Running Tests
```bash
cargo test
cargo test --package glyph
```

## Code Organization

### File Structure
```
src/
├── main.rs         # CLI setup and argument parsing
├── compiler.rs     # Compilation workflow
└── runner.rs       # Execution workflow
```

### Dependency Management
- **clap**: Use derive macros for argument parsing
- **dialoguer**: Interactive prompts for user input
- **chisel**: Internal compiler crate
- **sel**: Internal VM runtime crate

## Coding Standards

### Style Guidelines
- Follow `rustfmt` formatting
- Use `clippy` for linting
- Maintain consistent error handling patterns
- Document all public functions

### Error Handling
- Use `expect()` for unrecoverable errors
- Display user-friendly error messages
- Exit with appropriate status codes
- Provide helpful suggestions when possible

### User Experience
- Keep CLI interface simple and intuitive
- Provide clear progress indicators
- Use consistent terminology
- Support standard Unix conventions

## Adding New Features

### Command-Line Options
1. Add new field to `Args` struct in `main.rs`
2. Use clap derive attributes for configuration
3. Update help text and documentation
4. Add validation logic if needed

Example:
```rust
#[derive(Parser)]
struct Args {
    /// New feature flag
    #[arg(long, help = "Enable new feature")]
    new_feature: bool,
}
```

### Workflow Extensions
1. Create new function in appropriate module
2. Follow existing error handling patterns
3. Add comprehensive error messages
4. Update main dispatch logic

### Interactive Features
1. Use `dialoguer` for user input
2. Provide sensible defaults
3. Handle user cancellation gracefully
4. Validate input before processing

## Testing Guidelines

### Unit Tests
- Test argument parsing logic
- Validate error handling paths
- Mock file system operations when possible
- Test edge cases and error conditions

### Integration Tests
- Test complete compilation workflows
- Verify file I/O operations
- Test error propagation from dependencies
- Validate output formats

### Manual Testing
- Test with various file types and sizes
- Verify error messages are helpful
- Check cross-platform compatibility
- Test interactive prompts

## Documentation

### Code Documentation
- Document all public functions with `///`
- Include examples for complex operations
- Explain error conditions and return values
- Update module-level documentation

### User Documentation
- Update CLI help text for new options
- Add examples to README files
- Document breaking changes
- Provide migration guides when needed

## Performance Considerations

### File Operations
- Use buffered I/O for large files
- Avoid unnecessary file copies
- Handle file system errors gracefully
- Support standard input/output streams

### Memory Usage
- Minimize memory allocations in hot paths
- Use streaming for large inputs when possible
- Profile memory usage for large programs
- Avoid keeping entire files in memory

### Startup Time
- Minimize dependency initialization
- Use lazy loading where appropriate
- Avoid expensive operations in argument parsing
- Profile startup performance regularly

## Error Handling Patterns

### File System Errors
```rust
let content = fs::read_to_string(&path)
    .expect(&format!("Failed to read file: {}", path.display()));
```

### Compilation Errors
```rust
match chisel::compile_source(source, name, author) {
    Ok(bytecode) => {
        // Handle success
    }
    Err(error) => {
        error.report_with_ariadne(&source, &filename);
        std::process::exit(1);
    }
}
```

### User Input Errors
```rust
let input = dialoguer::Input::new()
    .with_prompt("Enter value")
    .default("default".to_string())
    .interact()
    .expect("Failed to read user input");
```

## Release Process

### Version Management
- Follow semantic versioning
- Update version in `Cargo.toml`
- Tag releases in git
- Update changelog

### Binary Distribution
- Build release binaries for target platforms
- Test binaries on clean systems
- Package with documentation
- Verify all features work in release mode

### Documentation Updates
- Update README with new features
- Refresh examples and tutorials
- Verify all links and references
- Update API documentation

## Common Pitfalls

### Argument Parsing
- Don't forget to handle conflicting options
- Provide clear error messages for invalid combinations
- Support both short and long option forms
- Validate file paths before processing

### File Handling
- Always check file permissions
- Handle missing files gracefully
- Support relative and absolute paths
- Clean up temporary files

### Cross-Platform Issues
- Test path handling on different systems
- Use `std::path::Path` for file operations
- Handle different line endings
- Test on Windows, macOS, and Linux

## Debugging Tips

### Common Issues
- Argument parsing failures
- File permission problems
- Path resolution issues
- Dependency version conflicts

### Debugging Tools
- Use `cargo run -- --help` to test CLI
- Enable debug logging with environment variables
- Use `strace`/`dtruss` to debug file operations
- Profile with `cargo flamegraph`

### Testing Strategies
- Create test fixtures for various file types
- Use temporary directories for test outputs
- Mock external dependencies when possible
- Test error conditions explicitly

## Contributing Workflow

### Pull Request Process
1. Fork the repository
2. Create feature branch
3. Make changes with tests
4. Update documentation
5. Submit pull request
6. Address review feedback

### Code Review Checklist
- [ ] Code follows style guidelines
- [ ] Tests cover new functionality
- [ ] Documentation is updated
- [ ] Error handling is appropriate
- [ ] Performance impact is considered
- [ ] Breaking changes are documented

### Commit Messages
- Use conventional commit format
- Include scope (cli, args, etc.)
- Provide clear description
- Reference issues when applicable

Example:
```
feat(cli): add support for custom output formats

- Add --format option for output control
- Support JSON and YAML formats
- Update help text and documentation

Closes #123
```
