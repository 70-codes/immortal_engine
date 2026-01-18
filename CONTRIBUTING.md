# Contributing to Immortal Engine

Thank you for your interest in contributing to Immortal Engine! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. Please:

- Be respectful and considerate in all interactions
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Accept criticism gracefully

## Getting Started

### Prerequisites

- **Rust 1.70 or later** - [Install Rust](https://rustup.rs/)
- **Git** - For version control
- **Linux dependencies** (Ubuntu/Debian):
  ```bash
  sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
  ```

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/70-codes/immortal_engine.git
   cd immortal_engine
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/70-codes/immortal_engine.git
   ```

## Development Setup

### Building the Project

```bash
# Build all crates
cargo build --workspace

# Build in release mode
cargo build --workspace --release
```

### Running the Editor

```bash
cargo run --bin imortal-editor
```

### Running the CLI

```bash
cargo run -p imortal_cli -- --help
```

### Project Structure

```
imortal_engine/
├── src/main.rs              # Visual editor entry point
├── crates/
│   ├── core/                # Shared types and traits
│   ├── ir/                  # Intermediate representation
│   ├── components/          # Component definitions
│   ├── codegen/             # Code generation
│   ├── ui/                  # Visual editor UI
│   └── cli/                 # Command-line interface
├── docs/                    # Documentation
└── tests/                   # Integration tests
```

## Making Changes

### Branch Naming

Use descriptive branch names:

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Test additions/changes

### Commit Messages

Write clear, concise commit messages:

```
type: short description

Longer description if needed. Explain what and why,
not how (the code shows how).

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat: add zoom with mouse wheel`
- `fix: correct edge drawing position`
- `docs: update getting started guide`

## Code Style

### Formatting

Always format your code before committing:

```bash
cargo fmt --all
```

### Linting

Run clippy and fix any warnings:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### Rust Style Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Document public APIs with doc comments
- Keep functions focused and small
- Prefer `impl Trait` over `Box<dyn Trait>` when possible

### Documentation

- Add doc comments (`///`) to all public items
- Include examples in doc comments where helpful
- Update relevant documentation files in `docs/`

Example:
```rust
/// Creates a new node with the given component type and name.
///
/// # Arguments
///
/// * `component_type` - The type identifier (e.g., "data.entity")
/// * `name` - Display name for the node
///
/// # Example
///
/// ```
/// let node = Node::new("data.entity", "User");
/// ```
pub fn new(component_type: impl Into<String>, name: impl Into<String>) -> Self {
    // ...
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p imortal_ir

# Run tests with output
cargo test --workspace -- --nocapture

# Run a specific test
cargo test test_name
```

### Writing Tests

- Place unit tests in the same file as the code being tested
- Use the `#[cfg(test)]` module pattern
- Write descriptive test names that explain what's being tested
- Test both success and failure cases

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation_with_valid_type() {
        let node = Node::new("data.entity", "User");
        assert_eq!(node.name, "User");
        assert_eq!(node.component_type, "data.entity");
    }

    #[test]
    fn test_node_has_default_fields() {
        let node = Node::new_entity("User");
        assert!(!node.fields.is_empty());
        assert!(node.fields.iter().any(|f| f.name == "id"));
    }
}
```

### Test Coverage

Aim for good test coverage, especially for:

- Core data structures (IR, nodes, edges)
- Validation logic
- Serialization/deserialization
- Component instantiation

## Submitting Changes

### Before Submitting

1. **Sync with upstream:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run all checks:**
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

3. **Update documentation** if needed

4. **Update CHANGELOG.md** for user-facing changes

### Pull Request Process

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature
   ```

2. Open a Pull Request on GitHub

3. Fill out the PR template:
   - Describe what the PR does
   - Link related issues
   - Include screenshots for UI changes
   - List any breaking changes

4. Wait for review and address feedback

5. Once approved, your PR will be merged

### PR Checklist

- [ ] Code follows the style guidelines
- [ ] Tests pass locally
- [ ] New code has tests
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if applicable)
- [ ] No new warnings from clippy

## Reporting Issues

### Bug Reports

When reporting bugs, include:

1. **Description** - Clear description of the bug
2. **Steps to Reproduce** - Minimal steps to trigger the bug
3. **Expected Behavior** - What should happen
4. **Actual Behavior** - What actually happens
5. **Environment** - OS, Rust version, etc.
6. **Screenshots** - If applicable

### Feature Requests

For feature requests, include:

1. **Problem** - What problem does this solve?
2. **Solution** - Describe your proposed solution
3. **Alternatives** - Other solutions you considered
4. **Context** - Any additional context

### Issue Labels

- `bug` - Something isn't working
- `enhancement` - New feature request
- `documentation` - Documentation improvements
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention needed

## Getting Help

- **Documentation**: Check the [docs](docs/) folder
- **Issues**: Search existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions

## Recognition

Contributors will be recognized in:

- The project README
- Release notes for significant contributions
- The CONTRIBUTORS file

Thank you for contributing to Immortal Engine! 🎉