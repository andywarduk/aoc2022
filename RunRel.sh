#!/bin/bash

if [ $1 -lt 1 -o $1 -gt 25 ]; then
	echo "Must give day number"
	exit 1
fi

daypad="$(printf %02d $1)"

cargo build --release --bin day$daypad --quiet

\time -v -o stats/day$daypad.txt target/release/day$daypad