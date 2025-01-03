# Finternet (UNITS)

A WebAssembly-powered financial transaction processing system with a layered architecture designed for secure, extensible asset management.

## Architecture

Finternet implements a three-tier "burger" architecture:

1. **Process Layer**: User interaction layer for executing workflows (WebAssembly modules). Users interact with assets through the Virtual Asset Layer (VAL), which provides access to driver layer functions.

2. **Driver Layer**: Intermediary between platform and process layers. Gives data meaning and handles user context with permissioned access. Drivers are dynamically loaded/unloaded at runtime.

3. **Platform Layer**: Base layer providing low-level component abstractions exposed to the driver runtime. Handles storage, networking, and runtime services.

## Features

- Dynamic WebAssembly module loading for drivers and user processes
- GRPC API with web interface support
- Redis-backed storage system
- Comprehensive permission and context management
- Support for custom asset types through driver system

## User Interface

For detailed UI documentation and setup instructions, see [UI Documentation](ui/units-ui/README.md).

The Finternet system includes a modern web interface built with Next.js and Tailwind CSS, providing:

### Core Features
- WebAssembly Module Management: Interface for loading and managing token handlers
- Account Management: User onboarding and account binding workflows
- Transaction Execution: Visual interface for executing WebAssembly programs
- Real-time Status Updates: Live monitoring of transaction and system status

### Technical Implementation
- Built on Next.js App Router architecture
- Responsive design using Tailwind CSS and shadcn/ui components
- Type-safe gRPC-web communication with the backend
- Dark/light theme support with consistent design system
- Component hierarchy:
  - UI primitives (buttons, inputs, dialogs)
  - Feature components (module loader, transaction forms)
  - Layout components (navigation, dashboard)

### Integration Points
- Connects to core services via gRPC:
  - Driver Service: Token handler management
  - Bind Service: User onboarding
  - Execution Service: WebAssembly program execution
- Real-time updates using server-sent events
- Secure authentication and session management

## Prerequisites

- Rust toolchain
- Redis server
- Protocol Buffers compiler
- WebAssembly target support (`wasm32-unknown-unknown`)

## Installation

1. Add WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### Starting the Server

```bash
cargo run --bin server -- config/development.toml
```

### Loading a Driver

```bash
cd modules/driver
make build
./load-driver.sh
```

### Binding Assets

```bash
./bind1.sh  # Bind first account
./bind2.sh  # Bind second account
```

### Executing Workflows

```bash
cd modules/user-module
make build
./exec.sh
```

## Project Structure

- `src/`: Core server implementation
  - `runtime/`: Main runtime implementation
  - `service/`: GRPC service definitions
  - `types/`: Core type definitions
  
- `modules/`: WebAssembly modules
  - `driver/`: Example driver implementation
  - `user-module/`: Example user workflow

- `proto/`: Protocol buffer definitions
  - `service.proto`: Main service definitions
  - `health_check.proto`: Health check service

- `config/`: Configuration files
  - `development.toml`: Development configuration

## Configuration

Configuration is handled through TOML files with the following sections:

- `server`: HTTP server settings
- `runtime`: Runtime name and version
- `driver`: Driver limits and timeouts
- `process`: Process-related settings
- `platform`: Platform-specific settings

## Development

### Building Modules

Drivers and user modules are built with:

```bash
RUSTFLAGS="-C target-feature=+multivalue" cargo build --target wasm32-unknown-unknown --release
```

### Adding New Drivers

1. Implement the required driver interface functions:
   - `intend`
   - `done`
   - `transfer`
   - `view`

2. Compile to WebAssembly

3. Load using the Driver service API

## Testing

Run the test suite with:

```bash
cargo test
```
