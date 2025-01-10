#!/bin/bash

grpcurl -plaintext -d @ localhost:8080 finternet.DriverDetails/SendDetails<<EOM
{
}
EOM