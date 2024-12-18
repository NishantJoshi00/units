This is the list of things that we need to build for the first POC of the project.

- [ ] The Unified Ledger Implementation
  - [ ] The monolithic state of the system.
    - [ ] The Program Runtime
    - [ ] The Driver Runtime
    - [ ] The Resolver ( That provides driver discovery for the program runtime )
  - [ ] The gRPC API
    - [ ] An API to load programs
    - [ ] An API to load drivers
    - [ ] An API to bind drivers to identifiers
  - [ ] (Optional) An event pipeline to notify the system changes via a channel to the outside world.

The server API should expose 3 services

- The Driver Service

  - LoadDriver
  - UnloadDriver

- Bind Service

  - BindDriver
  - UnbindDriver

- Execution Service
  - ExecuteProgram
