#!/bin/bash

# install cargo-make if not already installed
cargo make --version 2> /dev/null
if [ "$?" == "101" ]; then
  echo "Installing cargo-make..."
  cargo install cargo-make 2> /dev/null
  version=$(cargo make --version)
  echo "$version installed"
fi

if [ -z "$1" ]; then
  # run default tests
  cargo make --makefile test-flow.toml main
else
  # run flow from arg
  cargo make --makefile test-flow.toml $1
fi