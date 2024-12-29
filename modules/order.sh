#!/bin/bash

set -e

pushd driver/ || exit

make build

bash load-driver.sh
bash bind1.sh
bash bind2.sh

popd || exit

pushd user-module/ || exit

make build

bash exec.sh

popd || exit
