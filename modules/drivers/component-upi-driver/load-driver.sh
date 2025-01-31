#!/bin/bash

# for mono driver

grpcurl -plaintext -d @ localhost:8080 finternet.Driver/LoadDriver <<EOM
{
  "driver_name": "upi",
  "driver_version": "0.1.0",
  "driver_binary": "$(base64 -w0 -i target/wasm32-wasip1/release/component_upi_driver.wasm || base64 -i target/wasm32-wasip1/release/component_upi_driver.wasm)"
}
EOM
