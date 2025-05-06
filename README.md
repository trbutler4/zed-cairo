# zed-cairo

Cairo language support for Zed

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

## Overview

Cairo language support for the [Zed](https://zed.dev/) editor. This extension provides syntax highlighting, language server integration, and other essential features for developing Cairo applications in Zed.

> **This extension is under active development**

## Features

- Syntax highlighting for Cairo files
- Language server integration via `scarb`
- Code diagnostics and error reporting
- Common development tasks integration (build, test, run)

## Requirements

- [Zed editor](https://zed.dev/)
- [Scarb](https://docs.swmansion.com/scarb/) - Cairo package manager

## Usage

After installation, Zed will automatically detect `.cairo` files and provide language support features.

Available tasks:
- `scarb build`
- `scarb run`
- `scarb test`
- `scarb fmt`
- `scarb clean`

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Starkware's tree-sitter-cairo](https://github.com/starkware-libs/tree-sitter-cairo) for grammar definitions
- [Software Mansion's CairoLS](https://github.com/software-mansion/cairols) for the cairo language server
