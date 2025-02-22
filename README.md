# UNITS Runtime

UNITS is a modular WebAssembly runtime system designed for financial transactions and asset management. At its core, it provides a secure and flexible environment for executing WebAssembly modules while managing financial assets through a driver-based architecture.

## Getting Started

Starting with UNITS requires Rust 1.75 or later and a few core dependencies. First, ensure you have Rust installed with component support and the Protocol Buffers compiler. You'll also need SQLite for data persistence.

Begin by cloning the repository:

```bash
git clone https://github.com/yourusername/units.git
cd units
```

Before building the project, set up your environment variables for authentication:

```bash
export UNITS_USERNAME=your_username
export UNITS_PASSWORD=your_password
export secret=your_jwt_secret
```

Now initialize your database:

```bash
sqlite3 units.db < migrations/20250206_000000_create_table.sql
```

You can now build and start the server:

```bash
cargo build --release
cargo run --release -- config/development.toml
```

For development with debug logging enabled:

```bash
RUST_LOG=debug cargo run -- config/development.toml
```

## Working with the API 

UNITS exposes its functionality through a gRPC API. Here's how to interact with the key services:

Create a new user and get authentication token:
```bash
# Sign up
grpcurl -plaintext -d @- localhost:8080 units.UserSignUp/SignUp << EOM
{
  "user_name": "testuser",
  "name": "Test User",
  "email": "test@example.com",
  "password": "testpass123"
}
EOM

# Log in and get JWT token
grpcurl -plaintext -d @- localhost:8080 units.UserLogin/Login << EOM
{
  "user_name": "testuser",
  "password": "testpass123"
}
EOM
```

Load a driver into the system:
```bash
grpcurl -plaintext -d @- localhost:8080 units.Driver/LoadDriver << EOM
{
  "driver_name": "example-driver",
  "driver_version": "1.0.0",
  "driver_binary": "$(base64 -w0 path/to/driver.wasm)"
}
EOM
```

Submit and execute a program:
```bash
# Submit program
grpcurl -plaintext -d @- localhost:8080 units.Execution/Submit << EOM
{
  "name": "example-program",
  "version": "1.0.0",
  "binary": "$(base64 -w0 path/to/program.wasm)"
}
EOM

# Execute program (include your JWT token)
grpcurl -plaintext \
  -H "Authorization: your.jwt.token" \
  -d @- localhost:8080 units.Execution/Execute << EOM
{
  "input": "{\"action\":\"transfer\",\"amount\":100}",
  "program_id": "your-program-id"
}
EOM
```

List available drivers:
```bash
grpcurl -plaintext -d @- localhost:8080 units.DriverDetails/SendDetails << EOM
{}
EOM
```

## Understanding the Architecture

UNITS employs a three-tier "burger architecture" that cleanly separates concerns across different layers of the system. The Process Layer sits at the top, handling user workflows and WebAssembly module execution. Below it, the Driver Layer manages asset abstractions and permissions. The Platform Layer forms the foundation, handling system operations and storage.

This architecture enables UNITS to maintain strong isolation between different components while providing flexible integration points. Each layer communicates through well-defined interfaces, making the system both modular and extensible.

## Development Workflow

When developing with UNITS, you'll typically work with two main components: drivers and programs. Drivers provide the interface to underlying assets, while programs implement the business logic that operates on these assets.

To create a new driver:

```bash
./scripts/create-driver.sh my-driver
cd modules/drivers/my-driver
```

For a new program:

```bash
./scripts/create-program.sh my-program
cd modules/programs/my-program
```

The project includes a comprehensive test script that builds the runtime, starts the server, loads test drivers, and verifies everything works correctly:

```bash
./test.sh
```

## User Interfaces

UNITS provides two interface options for different use cases. The Terminal UI offers a lightweight, command-line interface perfect for development and scripting:

```bash
cd ui/units-tui
cargo run
```

The Web UI provides a more visual, user-friendly interface:

```bash
cd ui/units-ui
npm install
npm start
```

## Contributing

We welcome contributions to UNITS. Start by forking the repository and creating a feature branch for your work. After making your changes, submit a pull request. See our development guidelines in [DEVELOPMENT.md](docs/DEVELOPMENT.md) for more details.

## Support and Resources

If you need help or want to contribute, you can open an issue, start a discussion, or browse our documentation. Join our community and help make UNITS better for everyone.

## License

UNITS is available under the MIT License, allowing for both personal and commercial use with proper attribution.
