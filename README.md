# Finternet Runtime

A modular and extensible runtime system for executing WebAssembly components with a focus on financial transactions and asset management.

## Overview

Finternet Runtime is a three-tier architecture system that provides:

1. **Process Layer**: Direct user interaction layer for executing workflows (WebAssembly modules)
2. **Driver Layer**: Intermediary layer providing abstraction over platform-specific operations
3. **Platform Layer**: Low-level system access and services integration

## Features

- WebAssembly Component Model support
- Dynamic driver loading/unloading
- Pluggable storage backends (Redis, in-memory)
- Solana blockchain integration
- gRPC/gRPC-Web API interface
- Built-in health checks and metrics
- WASI support

## Architecture

The system follows a "burger architecture" with three main layers:

```
┌─────────────────────────┐
│     Process Layer       │ User workflows & WebAssembly modules
├─────────────────────────┤
│     Driver Layer        │ Asset abstraction & permissions
├─────────────────────────┤
│     Platform Layer      │ Low-level services & storage
└─────────────────────────┘
```

### Process Layer
- Executes user workflows as WebAssembly modules
- Provides Virtual Asset Layer (VAL) for accessing driver functions
- Manages workflow lifecycle and execution context

### Driver Layer
- Dynamically loaded modules providing asset abstractions
- Manages permissions and access control
- Implements VAL functions for workflow interaction
- Supports hot-loading of new drivers

### Platform Layer
- Provides core system services
- Manages storage backends (Redis/in-memory)
- Handles blockchain integration
- Implements low-level platform operations

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Cargo
- Protocol Buffers compiler
- Redis (optional)

### Building

1. Clone the repository:
```bash
git clone https://github.com/yourusername/finternet-runtime.git
cd finternet-runtime
```

2. Build the project:
```bash
cargo build --release
```

### Configuration

Create a configuration file (`development.toml`):

```toml
[server]
host = "127.0.0.1"
port = 8080

[runtime]
name = "finternet"
version = "0.1.0"

[driver]
driver_limit = 100
driver_timeout = 200

[process]
[platform]
```

### Running

Start the server:

```bash
cargo run --release -- config/development.toml
```

### Creating Components

The system supports two types of components: drivers and programs. We provide scripts to easily create new components with the correct structure and WIT interfaces.

#### Creating a Driver

To create a new driver, use the `create-driver.sh` script followed by the driver name (lowercase only):

```bash
./create-driver.sh my-driver
```

This will:
1. Create a new component driver in `modules/drivers/component-my-driver`
2. Copy the necessary WIT interface files
3. Set up the initial project structure

#### Creating a Program

To create a new program, use the `create-program.sh` script followed by the program name (lowercase only):

```bash
./create-program.sh my-program
```

This will:
1. Create a new component program in `modules/programs/component-my-program`
2. Copy the necessary WIT interface files
3. Set up the initial project structure

Note: Both driver and program names must be lowercase and cannot contain spaces.
