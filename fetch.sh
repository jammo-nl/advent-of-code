#!/bin/sh
SCRIPT_DIR=$(realpath "$(dirname "$0")")

if [[ $# != 2 ]]; then
  echo "Please provide a year and day number."
  echo "usage: $0 YEAR DAY"
  exit 1
fi

if [[ ! "$1" =~ ^20([1-9]{2,})$ ]]; then
  echo "Argument '$1' is not a valid year."
  exit 1
fi

if [[ ! "$2" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  echo "Argument '$2' is not a valid day."
  exit 1
fi

if [[ -z "${AOC_SESSION-""}" ]]; then
  echo "No session token set in \$AOC_SESSION."
  exit 1
fi

mkdir -p "$SCRIPT_DIR/aoc$1/inputs"
URL="https://adventofcode.com/$1/day/$(("10#$2" + 0))/input"
curl "$URL" --cookie "session=$AOC_SESSION" -s | tee "$SCRIPT_DIR/aoc$1/inputs/day$2.txt"

