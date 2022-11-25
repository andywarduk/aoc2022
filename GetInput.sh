#!/bin/bash

if [ $1 -lt 1 -o $1 -gt 25 ]; then
	echo "Must give day number"
	exit 1
fi

day="$1"
daypad="$(printf %02d $1)"

curl -sS https://adventofcode.com/2022/day/$day/input -o inputs/day$daypad.txt

if [ $? -ne 0 ]; then
	echo "Failed to download"
fi
