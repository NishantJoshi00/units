#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Execution/Execute <<EOM
{
  "input": "{ \"path1\": \"/upi/person1\", \"path2\": \"/upi/person2\", \"amount\": 10 }",
  "binary": "$(base64 -w0 -i ./target/wasm32-wasip1/release/component_transfer_module.wasm || base64 -i target/wasm32-wasip1/release/component_transfer_module.wasm)"
}
EOM




# { "path1": "/mono/person1", "path2": "/mono/person2", "amount": 10 }
# '{"merchantCustomerId": "placeholder", "raw_message": "placeholder", "device_id": "placeholder", "num": "919962463212", "mobile_num": "919962463212", "device_finger_print": "placeholder", "upi_request_id": "placeholder", "bank_account_unique_id": "placeholder", "customer_vpa": "placeholder", "balance": "notupdated"}'