#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Bind/Bind <<EOM
{
  "driver_name": "mono",
  "driver_version":"0.1.0",
  "path": "~/mono",
  "account_info": "{ \"name\": \"John Doe\", \"amount\": 30 }"
}
EOM

# {"name": "John Doe", "amount": 30}