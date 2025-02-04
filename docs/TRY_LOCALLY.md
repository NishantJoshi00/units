# Try Finternet Runtime Locally

This guide will help you set up and run Finternet Runtime in your local environment.

## Prerequisites

Required:
- Rust 1.75 or later
- Cargo
- Protocol Buffers compiler
- Git

Optional:
- Redis (enabled by default, can be disabled)
- grpcurl (for API testing)

## Quick Start

### 1. Setup

Clone and enter the repository:
```bash
git clone https://github.com/yourusername/finternet-runtime.git
cd finternet-runtime
```

### 2. Configuration

Create your local configuration:
```bash
# Copy default configuration
cp config/development.toml config/local.toml
```

The default configuration provides:
- Local server on 127.0.0.1:8080
- In-memory or Redis storage
- Default driver limits

### 3. Build and Run

Build the project:
```bash
# Build with Redis support (default)
cargo build --release

# Build without Redis
cargo build --release --no-default-features
```

Start the server:
```bash
# Basic start
cargo run --release -- config/local.toml

# With debug logging
RUST_LOG=debug cargo run -- config/local.toml
```

### 4. Verify Installation

Check if the server is running:
```bash
# Using grpcurl
grpcurl -plaintext localhost:8080 grpc.health.v1.Health/Check

# List available services
grpcurl -plaintext localhost:8080 list
```

## Quick Test

### 1. Load a Sample Driver

```bash
# Build the example driver
cd modules/drivers/component-driver
cargo component build --release

# Load the driver
grpcurl -plaintext -d @- localhost:8080 finternet.Driver/LoadDriver << EOM
{
  "driver_name": "example-driver",
  "driver_version": "1.0.0",
  "driver_binary": "$(base64 -w0 target/wasm32-wasi/release/component-driver.wasm)"
}
EOM
```

### 2. Run a Sample Program

```bash
# Build the example program
cd modules/programs/component-transfer-module
cargo component build --release

# Submit the program
grpcurl -plaintext -d @- localhost:8080 finternet.Execution/Submit << EOM
{
  "name": "transfer-module",
  "version": "1.0.0",
  "binary": "$(base64 -w0 target/wasm32-wasi/release/component-transfer-module.wasm)"
}
EOM

# Execute the program
grpcurl -plaintext -d @- localhost:8080 finternet.Execution/Execute << EOM
{
  "input": "{\"action\":\"transfer\",\"amount\":100}",
  "program_id": "received-program-id"
}
EOM
```

## Development Tips

### Logging
- Use `RUST_LOG=debug` for detailed logs
- Check `finternet.log` for runtime logs
- Use `tracing` macros in your code for consistent logging

### Common Issues

1. **Server Won't Start**
- Check port availability
- Verify Redis connection (if enabled)
- Check configuration file syntax

2. **Driver Loading Fails**
- Verify WASM binary format
- Check driver interface implementation
- Confirm version compatibility

3. **Program Execution Fails**
- Check input format
- Verify driver dependencies
- Review error messages in logs

### Using the Test Script

The repository includes a test script for quick verification:
```bash
./test.sh
```

This script:
1. Builds the runtime
2. Starts the server
3. Loads a test driver
4. Runs a test program
5. Verifies the results

## Next Steps

After getting the runtime working locally, you can:
1. Create your own drivers
2. Develop custom modules
3. Integrate with your systems
4. Contribute to the project

For detailed development guidelines, refer to our [Development Guidelines](./development-guidelines.md).
