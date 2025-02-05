# UNITS

<div align="center">

<img src="https://finternetlab.io/images/headers/finternet_logo_for_website-transformed1.png" alt="Finternet" height="46">

[![Rust](https://img.shields.io/badge/rust-1.75+-93450a.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/yourusername/finternet-runtime.svg)](https://github.com/yourusername/finternet-runtime/issues)

A modular and extensible WebAssembly runtime system for financial transactions and asset management

[Architecture](#architecture) •
[Quick Start](#quick-start) •
[Documentation](#documentation) •
[Contributing](#contributing)

</div>

## Overview

UNITS is a three-tier runtime system that provides:

- **Process Layer**: Direct user interaction for executing workflows
- **Driver Layer**: Asset abstraction and platform operations
- **Platform Layer**: System services and storage integration

Key Features:
- 📦 WebAssembly Component Model support
- 🔌 Dynamic driver loading/unloading
- 💾 Pluggable storage backends
- ⛓️ Solana blockchain integration
- 🌐 gRPC/gRPC-Web API interface
- 🔍 Built-in health checks

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/finternet-runtime.git

# Enter the directory
cd finternet-runtime

# Build the project
cargo build --release

# Start the server
cargo run --release -- config/development.toml
```

## Documentation

Our documentation is organized into three main sections:

### [📐 Architecture](docs/ARCHITECTURE.md)
Detailed explanation of UNITS's design and components:
- Three-tier architecture
- System flow and interactions
- Security model
- Extension points

### [🚀 Try Locally](docs/TRY_LOCALLY.md)
Get started with running UNITS:
- Prerequisites and setup
- Configuration options
- Quick test guide
- Development tips

### [📚 Development Guidelines](docs/DEVELOPMENT.md)
Learn how to develop for UNITS:
- Driver development
- Module development
- Best practices
- Contributing guidelines

## Contributing

We welcome contributions! See our [Development Guidelines](docs/DEVELOPMENT.md) for details on:
- Creating drivers and modules
- Requesting new capabilities
- Development best practices
- Pull request process

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- 📝 [Open an issue](https://github.com/yourusername/finternet-runtime/issues)
- 💬 [Start a discussion](https://github.com/yourusername/finternet-runtime/discussions)
- 🔍 [Browse documentation](docs/)

---

<div align="center">
Made with ❤️ by the UNITS Team
</div>
