# Contributing to wacloudapi

Thank you for your interest in contributing to wacloudapi! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all skill levels.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/fdciabdul/whatsapp-cloud-api-rs/issues)
2. If not, create a new issue with:
   - A clear, descriptive title
   - Steps to reproduce the bug
   - Expected behavior
   - Actual behavior
   - Rust version and OS information

### Suggesting Features

1. Check existing issues for similar suggestions
2. Create a new issue with:
   - A clear description of the feature
   - Use cases and benefits
   - Any implementation ideas (optional)

### Pull Requests

1. Fork the repository
2. Create a new branch for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes
4. Ensure all tests pass:
   ```bash
   cargo test
   ```
5. Format your code:
   ```bash
   cargo fmt
   ```
6. Run clippy:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
7. Commit your changes with a descriptive message
8. Push to your fork and create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Generating Documentation

```bash
cargo doc --no-deps --open
```

## Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write documentation for public APIs
- Add tests for new functionality

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Reference issue numbers when applicable (e.g., "Fix #123")

## Release Process

Releases are automated via GitHub Actions:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Commit and push to main branch
4. GitHub Actions will automatically:
   - Create a git tag
   - Create a GitHub release
   - Publish to crates.io

## Questions?

Feel free to open an issue for any questions or concerns.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
