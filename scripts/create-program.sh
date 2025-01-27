#!/bin/bash

ARGUMENT=$1
PROGRAM_NAME=component-$1

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
cargo component new $PROGRAM_NAME --lib
popd

cp wit/driver.wit modules/programs/$PROGRAM_NAME/wit/
cp wit/types.wit modules/programs/$PROGRAM_NAME/wit/
rm modules/drivers/$PROGRAM_NAME/wit/world.wit
