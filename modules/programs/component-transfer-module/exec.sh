#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Execution/Execute <<EOM
{
  "input": "{ \"path1\": \"/mono/person1\", \"path2\": \"/mono/person2\", \"amount\": 10 }",
  "binary": "$(base64 -w0 -i ./target/wasm32-wasip1/release/component_transfer_module.wasm || base64 -i target/wasm32-wasip1/release/component_transfer_module.wasm)"
}
EOM
