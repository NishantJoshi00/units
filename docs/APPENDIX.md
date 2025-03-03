# UNITS Runtime Glossary

## Core Concepts

### Burger Architecture
The three-tier architecture used by UNITS, consisting of:
- Process Layer (top)
- Driver Layer (middle)
- Platform Layer (bottom)

### Driver
A WebAssembly component that provides an interface to underlying assets and implements specific functionality. Drivers can be dynamically loaded and unloaded during runtime.

### Process/Program
A WebAssembly module containing business logic that operates on assets through the Virtual Asset Layer (VAL). These are user-submitted workflows.

### VAL (Virtual Asset Layer)
An abstraction layer that provides a unified interface for processes to interact with assets, regardless of the underlying driver implementation.

### Glue Code
Code that implements the interface bindings between WebAssembly components and the host runtime. Located in `src/runtime_v2/glue/`, it:
- Implements host functions that WebAssembly can call
- Translates between WebAssembly and native types
- Manages state and resources during cross-boundary calls
- Provides security boundaries between components

## WebAssembly Integration

### WIT (WebAssembly Interface Types)
Interface definition format used in `.wit` files that defines the contracts between WebAssembly components and the host system. For example:
```wit
interface storage {
    variant storage-error {
        not-found(string),
        invalid-key(string)
    }
    get: func(key: string) -> result<string, storage-error>;
}
```

### Component Model
WebAssembly's component model used for creating modular, reusable components with well-defined interfaces. Enables the use of structured interface types and component linking.

### Host Functions
Functions implemented in the glue code that WebAssembly components can call. They provide the actual implementation of interfaces defined in WIT files.

### Bindgen
Code generation tool that creates Rust bindings from WIT definitions. Used to generate type-safe interfaces between Rust and WebAssembly.

### Linker
System that connects WebAssembly imports to host functions. The glue code registers functions with the linker to make them available to WebAssembly modules.

## System Components

### Descriptor
A reference to an asset that includes:
- Driver name
- Driver version
- Account information
Used to track and manage asset access during operations.

### Resolver
A component that maps paths to specific drivers and account information. Responsible for determining which driver should handle specific asset operations.

### Platform Layer Components
- Storage System: Handles data persistence
- Blockchain Integrator: Manages blockchain operations
- Network Handlers: Manages network communication
- System Services: Core system functionality

## Technical Terms

### WASI (WebAssembly System Interface)
The system interface that allows WebAssembly modules to interact with the host system. UNITS provides WASI support for both drivers and processes.

### gRPC/gRPC-Web
The protocol used for API communication in UNITS. Supports both standard gRPC and gRPC-Web for browser compatibility.

## Operations

### Bind Operation
The process of associating a driver with a specific path, allowing processes to access assets through that path.

### Intent Operation
Creates a transaction intent, representing the first phase of a two-phase commit pattern.

### Transfer Operation
Moves assets between accounts using the same driver type.

### View Operation
Retrieves the current state of an asset without modification.

## Security Concepts

### Driver Context
Security context specific to a driver, containing:
- Driver information
- Permissions
- Resource limits

### Process Context
Security context for executing processes, including:
- User context
- Available drivers
- Resource limits

### JWT (JSON Web Token)
Used for authentication and authorization in the system's API endpoints.

## Storage Types

### Key-Value Storage
Primary storage mechanism supporting:
- Redis backend
- SQLite backend
- In-memory storage

### Resolver Storage
Specialized storage for path-to-driver mappings and associated metadata.

## Development Tools

### Terminal UI (TUI)
Command-line interface for development and testing, providing direct access to UNITS functionality.

### Web Dashboard
Visual interface for managing UNITS components, monitoring system state, and performing operations.

## Path Conventions

### Home Directory (~/)
Represents user-specific paths, automatically expanded to `/accounts/{user_id}/`.

### System Paths
- `/accounts/` - User account root
- Standard Unix-like path hierarchy for asset organization

## Error Categories

### Driver Errors
- PermissionDenied: Access control violations
- SystemError: Internal system failures
- InvalidInput: Input validation failures
- UnknownError: Unclassified errors

### User Errors
- PermissionDenied: User access violations
- SystemError: Runtime system errors
- InvalidInput: Process input validation failures
- Failure: General operation failures
- UnknownError: Unclassified errors

## Monitoring

### Health Checks
System health monitoring endpoints providing:
- Service status
- Component availability
- System metrics

### Event System
Structured logging and event tracking for:
- Operation monitoring
- Error tracking
- Audit trails
- Performance metrics

## Resource Management

### Driver Limits
- Maximum number of loaded drivers
- Operation timeouts
- Resource quotas

### Process Resources
- Memory limits
- Execution timeouts
- Asset access controls

## Interface Standards

### Driver Interface
Standard functions that all drivers must implement:
- intend()
- done()
- transfer()
- view()
- bind()

### Process Interface
Standard functions for process modules:
- main()
Plus access to the VAL interface

### Storage Interface
Standard storage operations:
- get()
- set()
- delete()
