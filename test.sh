#!/bin/bash

set -e

cargo build
cargo run -- config/development.toml &
sleep 3
PID=$!

sleep 1

pushd ./modules/drivers/component-driver
cargo component build --release
bash load-driver.sh
bash bind1.sh
bash bind2.sh
popd

pushd ./modules/programs/component-transfer-module
cargo component build --release
bash exec.sh
popd

kill -9 $PID
