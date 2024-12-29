#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Driver/LoadDriver <<EOM
{
  "driver_name": "mono",
  "driver_version": "0.1.0",
  "driver_type": "WASM",
  "driver_binary": "$(base64 -i target/wasm32-unknown-unknown/release/driver.wasm)"
}
EOM
