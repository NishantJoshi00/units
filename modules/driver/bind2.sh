#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Bind/Bind <<EOM
{
  "driver_name": "mono",
  "driver_version":"0.1.0",
  "path": "/mono/person2",
  "account_info": "{ \"name\": \"Jane Alek\", \"amount\": 100 }"
}
EOM
