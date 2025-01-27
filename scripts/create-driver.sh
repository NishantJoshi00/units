#!/bin/bash

ARGUMENT=$1
DRIVER_NAME=component-$1

# check if there is space in the argument
if [[ $ARGUMENT == *" "* ]]; then
  echo "Error: The driver name cannot contain spaces."
  exit 1
fi
# check if there is any uppercase letter in the argument
if [[ $ARGUMENT =~ [A-Z] ]]; then
  echo "Error: The driver name cannot contain uppercase letters."
  exit 1
fi

pushd modules/drivers/
cargo component new $DRIVER_NAME --lib
popd

cp wit/driver.wit modules/drivers/$DRIVER_NAME/wit/
cp wit/types.wit modules/drivers/$DRIVER_NAME/wit/
rm modules/drivers/$DRIVER_NAME/wit/world.wit
