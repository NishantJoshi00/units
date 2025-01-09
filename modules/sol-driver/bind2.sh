#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Bind/Bind <<EOM
{
  "driver_name": "mono",
  "path": "/mono/person2",
  "account_info": "{ \"name\": \"Jane Alek\", \"amount\": 100 }"
}
EOM
