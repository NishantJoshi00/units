# Running UNITS Locally

This guide walks you through setting up and running UNITS in your local development environment. We'll cover the initial setup, configuration, and basic usage to get you started quickly.

## Setting Up Your Environment

UNITS requires a few key components to run. You'll need Rust 1.75 or later with component support, the Protocol Buffers compiler, and SQLite. Redis is optional but recommended for production deployments.

Start by cloning the repository and setting up your environment variables:

```bash
git clone https://github.com/yourusername/units.git
cd units

export UNITS_USERNAME=your_username
export UNITS_PASSWORD=your_password
export secret=your_jwt_secret
```

Next, create and initialize your SQLite database:

```bash
sqlite3 units.db < migrations/20250206_000000_create_table.sql
```

This creates the core tables: Resolver for path mapping, Program for WebAssembly programs, Driver for WebAssembly drivers, and User for authentication.

## Building and Running

With the environment ready, build the project:

```bash
cargo build --release
```

If you don't need Redis support, you can build without it:

```bash
cargo build --release --no-default-features
```

Start the server using the development configuration:

```bash
cargo run --release -- config/development.toml
```

During development, you might want more detailed logging:

```bash
RUST_LOG=debug cargo run -- config/development.toml
```

## First Steps with UNITS

Let's walk through a basic workflow to ensure everything is working. First, create a test user and get authentication set up:

```bash
# Create a new user
grpcurl -plaintext -d @- localhost:8080 units.UserSignUp/SignUp << EOM
{
  "user_name": "testuser",
  "name": "Test User",
  "email": "test@example.com",
  "password": "testpass123"
}
EOM

# Log in to get your JWT token
grpcurl -plaintext -d @- localhost:8080 units.UserLogin/Login << EOM
{
  "user_name": "testuser",
  "password": "testpass123"
}
EOM
```

Now let's load a sample driver. Navigate to the example driver directory and build it:

```bash
cd modules/drivers/component-driver
cargo component build --release
```

Load the driver into UNITS:

```bash
grpcurl -plaintext -d @- localhost:8080 units.Driver/LoadDriver << EOM
{
  "driver_name": "example-driver",
  "driver_version": "1.0.0",
  "driver_binary": "$(base64 -w0 target/wasm32-wasi/release/component-driver.wasm)"
}
EOM
```

Finally, let's run a sample program:

```bash
cd modules/programs/component-transfer-module
cargo component build --release

# Submit the program
grpcurl -plaintext -d @- localhost:8080 units.Execution/Submit << EOM
{
  "name": "transfer-module",
  "version": "1.0.0",
  "binary": "$(base64 -w0 target/wasm32-wasi/release/component-transfer-module.wasm)"
}
EOM
```

## Development Tools

UNITS provides two interfaces for development and testing. The Terminal UI offers a command-line interface that's perfect for development:

```bash
cd ui/units-tui
cargo run
```

The Web Dashboard provides a more visual interface:

```bash
cd ui/units-ui
npm install
npm start
```

## Troubleshooting Common Issues

If the server won't start, check that port 8080 is available and verify Redis is running if you're using it. For authentication issues, ensure your environment variables are set correctly and your JWT token hasn't expired.

When loading drivers fails, verify the WebAssembly binary format and check that the component interface is implemented correctly. The logs will often provide helpful information about what's going wrong.

## Moving Forward

Once you have the basic setup working, you can start creating your own drivers using the driver template and developing custom programs. The architecture documentation provides deeper insight into how these components work together.

Remember to check the development guidelines in DEVELOPMENT.md when you're ready to start contributing to the project. We encourage you to join our community and help make UNITS even better.
