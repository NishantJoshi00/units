#!/bin/bash

# for mono driver

grpcurl -plaintext -d @ localhost:8080 finternet.Driver/LoadDriver <<EOM
{
  "driver_name": "mono",
  "driver_version": "0.1.0",
  "driver_type": "WASM",
  "driver_binary": "$(base64 -w0 -i target/wasm32-unknown-unknown/release/driver.wasm || base64 -i target/wasm32-unknown-unknown/release/driver.wasm)"
}
EOM

# grpcurl -plaintext -d @ localhost:8080 finternet.Driver/UnloadDriver <<EOM
# {
#   "driver_name": "mono",
#   "driver_version": "0.1.0"
# }
# EOM

# for beta driver

# grpcurl -plaintext -d @ localhost:8080 finternet.Driver/LoadDriver <<EOM
# {
#   "driver_name": "beta",
#   "driver_version": "0.1.0",
#   "driver_type": "WASM",
#   "driver_binary": "$(base64 -w0 -i target/wasm32-unknown-unknown/release/driver.wasm || base64 -i target/wasm32-unknown-unknown/release/driver.wasm)"
# }
# EOM

# grpcurl -plaintext -d @ localhost:8080 finternet.Driver/UnloadDriver <<EOM
# {
#   "driver_name": "beta",
#   "driver_version": "0.1.0"
# }
# EOM


