# Contributing to BOM Calculation Engine

Thank you for your interest in contributing! We welcome contributions from the community.

## 🌍 Languages

This project supports multiple languages:
- English (primary development language)
- 繁體中文 (Traditional Chinese)
- 简体中文 (Simplified Chinese)
- Deutsch (German)

Documentation and discussions can be in any of these languages.

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourname/bom.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Create a Pull Request

## 📝 Development Guidelines

### Code Style

- Follow Rust's official style guide
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write documentation for public APIs
- Add tests for new features

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add SAP adapter for BOM repository
fix: correct cost calculation for phantom components
docs: update README with new examples
test: add benchmarks for deep BOM structures
```

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass: `cargo test --all`
- Run benchmarks to check performance: `cargo bench`

### Documentation

- Update README if adding new features
- Add doc comments for public APIs
- Update CHANGELOG.md
- Translate important documentation to supported languages

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. Your operating system and Rust version
2. Steps to reproduce the bug
3. Expected behavior
4. Actual behavior
5. Error messages or logs

## 💡 Suggesting Features

We love new ideas! When suggesting features:

1. Check if it's already been suggested
2. Explain the use case
3. Describe the proposed solution
4. Consider backward compatibility

## 🔍 Code Review Process

1. All submissions require review
2. We'll review your PR within a few days
3. Address any feedback
4. Once approved, we'll merge your PR

## 📜 License

By contributing, you agree that your contributions will be licensed under both:
- AGPL-3.0 (for open source use)
- Commercial License (as determined by the project maintainers)

## 🙏 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md
- Release notes
- Project documentation

## 💬 Communication

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Email**: xiaoivan1@proton.me for private inquiries

## 🎯 Priority Areas

We especially welcome contributions in:

1. **ERP/PLM Adapters**
   - SAP connector
   - Oracle connector
   - Other ERP systems

2. **Performance Improvements**
   - Algorithm optimizations
   - Cache enhancements
   - Parallel processing improvements

3. **Language Bindings**
   - Python bindings
   - Java bindings
   - .NET bindings

4. **Documentation**
   - Usage examples
   - Integration guides
   - Translations

5. **Testing**
   - More test cases
   - Performance benchmarks
   - Real-world scenarios

## 📊 Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-watch
cargo install cargo-edit
```

### Building

```bash
# Build all crates
cargo build --all

# Build with optimizations
cargo build --release

# Build FFI library
cargo build --release -p bom-ffi
```

### Testing

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p bom-core

# Run benchmarks
cargo bench
```

### Documentation

```bash
# Generate documentation
cargo doc --no-deps --open

# Check documentation
cargo doc --no-deps --all
```

## 🔧 Project Structure

```
bom/
├── crates/
│   ├── bom-core/       # Core data models
│   ├── bom-graph/      # Graph data structure
│   ├── bom-calc/       # Calculation engines
│   ├── bom-cache/      # Caching layer
│   ├── bom-ffi/        # C FFI bindings
│   ├── bom-adapters/   # ERP/PLM adapters
│   └── bom-benches/    # Benchmarks
├── examples/           # Usage examples
├── docs/              # Documentation
└── tests/             # Integration tests
```

## ❓ Questions?

If you have questions:

1. Check existing documentation
2. Search GitHub Issues
3. Ask in GitHub Discussions
4. Email xiaoivan1@proton.me

Thank you for contributing! 🎉
