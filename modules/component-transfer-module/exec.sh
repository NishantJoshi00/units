#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Execution/Execute <<EOM
{
  "name": "ttot",
  "input": "{ \"path1\": \"/mono/person1\", \"path2\": \"/mono/person2\", \"amount\": 10 }",
  "type": "WASM",
  "binary": "$(base64 -w0 -i ./target/wasm32-wasip1/release/component_user_module.wasm || base64 -i target/wasm32-wasip1/release/component_user_module.wasm)"
}
EOM
