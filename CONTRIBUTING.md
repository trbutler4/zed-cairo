## Contributing to Zed Cairo

Thank you for your interest in contributing to Zed Cairo! This document provides guidelines and instructions for contributing to this project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Resources](#resources)

## Code of Conduct

Please be respectful and considerate of others when contributing to this project.

## Getting Started

1. Ensure you have the required development tools installed:
   - Rust toolchain
    > Rust must be installed via rustup. If you have Rust installed via homebrew or otherwise, installing dev extensions will not work.
   - Zed editor
   - Scarb

2. Fork the repository on GitHub.

3. Clone your fork locally

4. Add the original repository as an upstream remote:
```bash
git remote add upstream https://github.com/0xpantera/zed-cairo.git
```

## Development Workflow

1. **Create an Issue**: Before starting work, check existing [issues](https://github.com/0xpantera/zed-cairo/issues) to see if your intended work is already being addressed.

   - If you find a relevant issue, comment on it to express your interest in working on it.
   - If no relevant issue exists, create a new one describing the feature, enhancement, or bug fix you intend to work on.

2. **Create a Branch**: Create a branch in your fork for your contribution:
```bash
git checkout -b feature/your-feature-name
```

3. **Make Changes**: Implement your changes, following the project's coding standards.

4. **Commit Your Changes**: Make commits with clear, descriptive messages:

5. **Keep Your Branch Updated**:
```bash
git fetch upstream
git rebase upstream/main
```

## Pull Request Process

1. Push your changes to your fork:
```bash
git push origin feature/your-feature-name
```

Submit a pull request from your fork to the main repository.

3. In your pull request description:
   - Reference the issue it addresses
   - Provide a clear description of the changes
   - Include any relevant documentation updates

4. Wait for a review from a maintainer.

5. Address any requested changes or feedback.

6. Once approved, a maintainer will merge your pull request.

## Coding Standards

- Follow Rust's standard coding conventions
- Write clear, descriptive comments
- Use meaningful variable and function names
- Ensure your code is well-formatted (use `cargo fmt`)
- Run `cargo clippy` to check for common mistakes and improvements

### Resources

- [Zed Editor docs](https://zed.dev/docs/)
- [Cairo Book](https://book.cairo-lang.org/)
- [Cairo tree-sitter implementation](https://github.com/starkware-libs/tree-sitter-cairo)
- [Cairo Languuage Server](https://github.com/software-mansion/cairols)
