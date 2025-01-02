#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Execution/Execute <<EOM
{
  "name": "ttot",
  "input": "{ \"path1\": \"/nars/person1\", \"path2\": \"/nars/person2\", \"amount\": 10 }",
  "type": "WASM",
  "binary": "$(base64 -i target/wasm32-unknown-unknown/release/user_module.wasm)"
}
EOM
