#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.Bind/Bind <<EOM
{
  "driver_name": "upi",
  "driver_version":"0.1.0",
  "path": "/upi/person1",
  "account_info": "{ \"merchantCustomerId\": \"placeholder\", \"raw_message\": \"placeholder\", \"device_id\": \"placeholder\", \"num\": \"919962463212\", \"mobile_num\": \"919962463212\", \"device_finger_print\": \"placeholder\", \"upi_request_id\": \"placeholder\", \"bank_account_unique_id\": \"placeholder\", \"customer_vpa\": \"placeholder\",\"balance\": \"dontknow\"  }"
}
EOM

# grpcurl -plaintext -d @ localhost:8080 finternet.PrintMessage/Print <<EOM
# {
#   "message":"hello grpc"
# }
# EOM