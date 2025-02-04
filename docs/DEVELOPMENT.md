# Development Guidelines

This guide covers development practices for Finternet Runtime components and how to request new capabilities.

## Table of Contents

1. [Driver Development](#driver-development)
   - Capabilities and Features
   - Implementation Guide

2. [Module Development](#module-development)
   - Capabilities and Features
   - Implementation Guide

3. [Requesting New Capabilities](#requesting-new-capabilities)
   - Platform Layer
   - Driver Layer
   - Process Layer

4. [Best Practices & Guidelines](#best-practices)
   - Security
   - Performance
   - Testing
   - API Design

5. [Contributing](#contributing-guidelines)
   - Development Process
   - Code Standards
   - Version Control

## Driver Development

### Driver Capabilities

1. **Storage Operations**
   - Key-value storage access
   - Data serialization
   - Cache management
   - Custom storage patterns

2. **HTTP Integration**
   - External API calls
   - RESTful services
   - Webhook handling
   - Custom protocols

3. **Asset Management**
   - Token operations
   - Balance tracking
   - Ownership transfer
   - Metadata handling

4. **Security Features**
   - Access control
   - Permission management
   - Rate limiting
   - Validation rules

### Creating a Driver

1. Use the creation script:
```bash
./scripts/create-driver.sh my-driver
```

2. Implement the interface:
```rust
use component::units::driver::*;

struct Component;

impl component::units::driver::Guest for Component {
    fn intend(input: String) -> Result<String, DriverError> {
        // Create transaction intent
    }

    fn done(input: String) -> Result<(), DriverError> {
        // Finalize transaction
    }

    fn transfer(from: String, to: String, value: String) -> Result<(), DriverError> {
        // Handle transfer
    }

    fn view(input: String) -> Result<String, DriverError> {
        // View asset state
    }

    fn bind(input: String, existing: Option<String>) -> Result<String, DriverError> {
        // Bind to path
    }
}
```

3. Build the driver:
```bash
cargo component build --release
# The driver will be available in the `target/wasm32-wasip1/release`
```



## Module Development

### Module Capabilities

1. **Asset Operations**
   - Descriptor management
   - Asset transfers
   - State queries
   - Complex operations
   - Operation chaining

2. **Workflow Management**
   - Transaction sequences
   - Business rules
   - Multi-step processes
   - State transitions
   - Approval flows

3. **Driver Interactions**
   - Multi-driver operations
   - Cross-asset operations
   - Atomic transactions
   - Composite operations

### Creating a Module

1. Use the creation script:
```bash
./scripts/create-program.sh my-program
```

2. Implement the interface:
```rust
use component::units::*;

struct Component;

impl component::units::module::Guest for Component {
    fn main(input: String) -> Result<String, UserError> {
        // Program logic
    }
}
```

3. Build the module:
```bash
cargo component build --release
# The module will be available in the `target/wasm32-wasip1/release`
```


## Requesting New Capabilities

### Platform Layer Capabilities

To request new platform layer capabilities:

1. **Open an Issue**
   - Describe the capability needed
   - Provide use cases
   - Suggest implementation approach

2. **Required Information**
   ```markdown
   ### Capability Request
   
   **Type:** Platform Layer
   
   **Description:**
   [Detailed description of the capability]
   
   **Use Cases:**
   1. [Use case 1]
   2. [Use case 2]
   
   **Proposed Interface:**
   ```rust
   fn new_capability(param: Type) -> Result<Output, Error>;
   ```
   
   **Security Considerations:**
   [Security implications]
   ```

### Driver Layer Capabilities

For new driver layer features:

1. **Open an Issue**
   ```markdown
   ### Driver Capability Request
   
   **Type:** Driver Layer
   
   **Description:**
   [Detailed description]
   
   **Interface Extension:**
   ```rust
   // New trait or function
   ```
   
   **Compatibility:**
   [Impact on existing drivers]
   ```

### Process Layer Capabilities

For process layer enhancements:

1. **Open an Issue**
   ```markdown
   ### Process Feature Request
   
   **Type:** Process Layer
   
   **Description:**
   [Detailed description]
   
   **Module Impact:**
   [How modules will use this]
   
   **Security Model:**
   [Security considerations]
   ```

## Best Practices

### Driver Development

1. **Error Handling**
   ```rust
   fn handle_operation() -> Result<(), DriverError> {
       // Use specific error variants
       if !authorized {
           return Err(DriverError::PermissionDenied("...".into()));
       }
       // Operation logic
       Ok(())
   }
   ```

2. **State Management**
   ```rust
   // Use clear state transitions
   fn process_transaction(state: &mut State) -> Result<(), DriverError> {
       state.validate()?;
       state.update()?;
       state.commit()
   }
   ```

### Module Development

1. **Input Validation**
   ```rust
   fn validate_input(input: &str) -> Result<Input, UserError> {
       // Parse and validate
       let parsed: Input = serde_json::from_str(input)?;
       parsed.validate()?;
       Ok(parsed)
   }
   ```

2. **Resource Management**
   ```rust
   fn process_assets() -> Result<(), UserError> {
       // Acquire resources in defined order
       let asset1 = acquire_asset("1")?;
       let asset2 = acquire_asset("2")?;
       // Process
       // Release in reverse order
   }
   ```

## Testing Guidelines

### Driver Testing

1. **Unit Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       #[test]
       fn test_driver_operations() {
           // Test implementation
       }
   }
   ```

2. **Integration Tests**
   ```rust
   #[test]
   fn test_driver_integration() {
       // Setup runtime
       // Load driver
       // Execute operations
       // Verify results
   }
   ```

### Module Testing

1. **Workflow Tests**
   ```rust
   #[test]
   fn test_complete_workflow() {
       // Setup test environment
       // Execute workflow
       // Verify results
   }
   ```

2. **Error Handling Tests**
   ```rust
   #[test]
   fn test_error_conditions() {
       // Test various error conditions
       // Verify proper error handling
   }
   ```

## Contributing Guidelines

1. **Code Style**
   - Follow Rust style guidelines
   - Use descriptive names
   - Document public interfaces
   - Add unit tests

2. **Pull Requests**
   - Create feature branch from main
   - Follow conventional commits
   - Include tests and documentation
   - Update relevant examples

3. **Documentation**
   - Update relevant docs
   - Include code examples
   - Document breaking changes
   - Add architecture diagrams if needed

## Security Guidelines

### Access Control
```rust
// Always implement proper access control
fn protected_operation(ctx: &Context) -> Result<(), Error> {
    // Verify permissions first
    ctx.verify_permissions("operation_name")?;
    
    // Proceed with operation
    perform_operation()
}
```

### Input Sanitization
```rust
// Sanitize all inputs
fn process_input(input: &str) -> Result<ProcessedInput, Error> {
    // Validate input format
    let validated = validate_input(input)?;
    
    // Check bounds and constraints
    validate_bounds(&validated)?;
    
    // Sanitize for storage/processing
    sanitize_for_storage(&validated)
}
```

### Error Handling Best Practices
```rust
// Use specific error types
#[derive(Debug, thiserror::Error)]
pub enum OperationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("System error: {0}")]
    SystemError(String),
}

// Implement proper error handling
fn handle_operation() -> Result<(), OperationError> {
    // Validate preconditions
    validate_input().map_err(OperationError::InvalidInput)?;
    
    // Check permissions
    check_permissions().map_err(OperationError::PermissionDenied)?;
    
    // Perform operation
    perform_operation().map_err(OperationError::SystemError)
}
```

## Performance Guidelines

### Resource Management
```rust
// Implement proper resource cleanup
struct ResourceManager {
    connection: Connection,
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        // Cleanup resources
        self.connection.close();
    }
}
```

### Optimization Tips
1. **Memory Usage**
   - Use appropriate data structures
   - Implement proper cleanup
   - Monitor memory usage

2. **Processing Efficiency**
   - Batch operations when possible
   - Use async for I/O operations
   - Implement caching where appropriate

## Debugging and Monitoring

### Logging Best Practices
```rust
// Use appropriate log levels
fn process_transaction(tx: Transaction) -> Result<(), Error> {
    tracing::debug!(?tx, "Starting transaction processing");
    
    if let Err(e) = validate_transaction(&tx) {
        tracing::error!(?tx, error = ?e, "Transaction validation failed");
        return Err(e);
    }
    
    tracing::info!(?tx, "Transaction processed successfully");
    Ok(())
}
```

### Metrics Collection
```rust
// Implement metrics collection
fn monitored_operation() -> Result<(), Error> {
    let start = std::time::Instant::now();
    
    let result = perform_operation();
    
    // Record operation duration
    let duration = start.elapsed();
    metrics::histogram!("operation.duration", duration);
    
    result
}
```

## API Design Guidelines

### REST API Best Practices
1. **Endpoint Design**
   - Use clear, consistent naming
   - Follow REST conventions
   - Version your APIs
   - Document all endpoints

2. **Response Format**
```rust
#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: Option<T>,
    error: Option<String>,
}

fn handle_request<T>(operation: impl Fn() -> Result<T, Error>) -> ApiResponse<T> {
    match operation() {
        Ok(data) => ApiResponse {
            status: "success".into(),
            data: Some(data),
            error: None,
        },
        Err(e) => ApiResponse {
            status: "error".into(),
            data: None,
            error: Some(e.to_string()),
        },
    }
}
```

## Testing Strategy

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operation() {
        // Arrange
        let input = prepare_test_input();
        
        // Act
        let result = process_operation(input);
        
        // Assert
        assert_matches!(result, Ok(_));
    }
    
    #[test]
    fn test_error_conditions() {
        // Test various error conditions
        let invalid_input = prepare_invalid_input();
        assert_matches!(
            process_operation(invalid_input),
            Err(OperationError::InvalidInput(_))
        );
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_workflow() {
        // Setup
        let runtime = setup_test_runtime().await;
        
        // Execute workflow
        let result = execute_workflow(&runtime).await;
        
        // Verify
        assert_workflow_result(result);
    }
}
```

### Performance Testing
```rust
#[cfg(test)]
mod perf_tests {
    use criterion::{criterion_group, criterion_main, Criterion};

    fn benchmark_operation(c: &mut Criterion) {
        c.bench_function("process_transaction", |b| {
            b.iter(|| {
                let tx = prepare_test_transaction();
                process_transaction(tx)
            })
        });
    }

    criterion_group!(benches, benchmark_operation);
    criterion_main!(benches);
}
```

## Version Control Guidelines

1. **Branch Strategy**
   - `main`: Production-ready code
   - `develop`: Integration branch
   - `feature/*`: New features
   - `bugfix/*`: Bug fixes
   - `release/*`: Release preparation

2. **Commit Messages**
   ```
   <type>(<scope>): <description>

   [optional body]

   [optional footer(s)]
   ```
   Types:
   - feat: New feature
   - fix: Bug fix
   - docs: Documentation
   - style: Formatting
   - refactor: Code restructuring
   - test: Adding tests
   - chore: Maintenance

3. **Code Review Process**
   - Create detailed PR description
   - Add relevant labels
   - Request appropriate reviewers
   - Address all comments
   - Ensure CI passes
   - Keep PRs focused and manageable
