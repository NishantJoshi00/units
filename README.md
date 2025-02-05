# Finternet Runtime

<div align="center">

<a href="https://finternetlab.io/">
  <img src="https://finternetlab.io/images/headers/finternet_logo_for_website-transformed1.png" alt="Finternet" height="46">
</a>

[![Rust](https://img.shields.io/badge/rust-1.75+-93450a.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/yourusername/finternet-runtime.svg)](https://github.com/yourusername/finternet-runtime/issues)

A modular and extensible WebAssembly runtime system for financial transactions and asset management

[Website](https://finternetlab.io/) •
[Architecture](#architecture) •
[Quick Start](#quick-start) •
[Documentation](#documentation) •
[Contributing](#contributing)

</div>

## Overview

Finternet Runtime is a three-tier architecture system that provides:

1. **Process Layer**: Direct user interaction layer for executing workflows (WebAssembly modules)
2. **Driver Layer**: Intermediary layer providing abstraction over platform-specific operations
3. **Platform Layer**: Low-level system access and services integration

Key Features:
- 📦 WebAssembly Component Model support
- 🔌 Dynamic driver loading/unloading
- 💾 Pluggable storage backends (Redis, in-memory)
- ⛓️ Solana blockchain integration
- 🌐 gRPC/gRPC-Web API interface
- 🔍 Built-in health checks and metrics
- ⚡ WASI support

## Overview

Finternet Runtime is a three-tier runtime system that provides:

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
git clone https://github.com/NishantJoshi00/units.git

# Enter the directory
cd units

# Build the project
cargo build --release

# Start the server
cargo run --release -- config/development.toml
```

## Documentation

Our documentation is organized into three main sections:

### [📐 Architecture](docs/ARCHITECTURE.md)
Detailed explanation of Finternet Runtime's design and components:
- Three-tier architecture
- System flow and interactions
- Security model
- Extension points

### [🚀 Try Locally](docs/TRY_LOCALLY.md)
Get started with running Finternet Runtime:
- Prerequisites and setup
- Configuration options
- Quick test guide
- Development tips

### [📚 Development Guidelines](docs/DEVELOPMENT.md)
Learn how to develop for Finternet Runtime:
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

- 📝 [Open an issue](https://github.com/NishantJoshi00/units/issues)
- 💬 [Start a discussion](https://github.com/NishantJoshi00/units/discussions)
- 🔍 [Browse documentation](docs/)

---

<div align="center">
Made with ❤️ by the UNITS Team
</div>
