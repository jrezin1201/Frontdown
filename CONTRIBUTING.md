# Contributing to RavensOne

Thank you for your interest in contributing! 🎉

## Getting Started

1. **Fork the repository**
2. **Clone your fork:**
   ```bash
   git clone https://github.com/yourusername/ravensone.git
   cd ravensone
```

1. **Create a branch:**
   
   ```bash
   git checkout -b feature/my-feature
   ```

## Development Setup

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run specific test
cargo test test_lexer_basic

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

See [Repository Structure](docs/repository-structure.md) for details.

## Making Changes

### 1. Choose an Issue

- Check [open issues](https://github.com/yourusername/ravensone/issues)
- Comment on the issue you want to work on
- Wait for approval before starting

### 2. Write Code

- Follow [Rust style guidelines](https://doc.rust-lang.org/1.0.0/style/)
- Write tests for new features
- Update documentation
- Keep commits atomic and well-described

### 3. Test Thoroughly

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test '*'

# Run specific example
cd examples/02-todo-app
raven build
```

### 4. Submit Pull Request

- Push your branch
- Open a PR with clear description
- Link related issues
- Wait for CI to pass
- Address review feedback

## Code Style

### Rust Code

```rust
// Good
fn parse_expression(&mut self) -> Result<Expression, String> {
    // Implementation
}

// Bad
fn parseExpression(&mut self)->Result<Expression,String>{
    // Implementation
}
```

### RavensOne Code

```raven
// Good
component Button(text: string, onClick: fn() -> void) {
  return <button on:click={onClick}>{text}</button>
}

// Bad
component button(text:string,onClick:fn()->void){return <button on:click={onClick}>{text}</button>}
```

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add JSX fragment support
fix: resolve type inference bug in match expressions
docs: update getting started guide
test: add parser integration tests
refactor: simplify code generation logic
```

## Testing Guidelines

### Unit Tests

```rust
#[test]
fn test_feature_name() {
    // Arrange
    let input = "test input";
    
    // Act
    let result = my_function(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

### Integration Tests

```rust
#[test]
fn test_full_compilation() {
    let source = r#"
        component App() {
            return <div>Hello</div>
        }
    "#;
    
    // Test full pipeline
    compile_and_verify(source);
}
```

## Documentation

- Update `docs/` for user-facing changes
- Add inline documentation for public APIs
- Include examples in documentation

```rust
/// Parses a RavensOne expression.
///
/// # Examples
///
/// ```
/// let expr = parser.parse_expression()?;
/// ```
pub fn parse_expression(&mut self) -> Result<Expression, String> {
    // Implementation
}
```

## Areas Needing Help

- 🟢 **Easy:** Documentation, examples, tests
- 🟡 **Medium:** Bug fixes, small features
- 🔴 **Hard:** Compiler features, optimizations

See [good first issue](https://github.com/yourusername/ravensone/labels/good%20first%20issue) label.

## Questions?

- Open a [discussion](https://github.com/yourusername/ravensone/discussions)
- Join our [Discord](https://discord.gg/ravensone)
- Ask on [Reddit](https://reddit.com/r/ravensone)

## Code of Conduct

Be respectful, inclusive, and constructive. See <CODE_OF_CONDUCT.md>.

## License

By contributing, you agree that your contributions will be licensed under MIT License.

