#!/bin/sh
SCRIPT_DIR=$(realpath "$(dirname "$0")")

if [[ $# != 1 ]]; then
  echo "Please provide a year."
  echo "usage: $0 YEAR"
  exit 1
fi

if [[ ! "$1" =~ ^20([1-9]{2,})$ ]]; then
  echo "Argument '$1' is not a valid year."
  exit 1
fi

cd $SCRIPT_DIR
cargo init "aoc$1"
mkdir -p "aoc$1/src/bin"

## copy init templates
cp templates/Cargo.toml aoc$1/Cargo.toml
cp templates/lib.rs aoc$1/src/lib.rs
cp templates/main.rs aoc$1/src/main.rs
sed -i "s/{YEAR}/$1/" aoc$1/Cargo.toml
sed -i "s/{YEAR}/$1/" aoc$1/src/main.rs


