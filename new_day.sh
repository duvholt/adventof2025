#!/bin/env bash

if [ "$#" -ne 1 ]; then
    echo "Day parameter missing"
    exit 1
fi

DAY=$1

echo "Creating day $DAY"

cp -r input/example "input/$DAY"
cp -r src/example.rs "src/day$DAY.rs"

sed -i "s/input\/example\/real/input\/$DAY\/real/g" "src/day$DAY.rs"

sed -i "/pub mod example;/a \
pub mod day$DAY;" src/lib.rs

sed -i "/map\.insert(\"example-2\", example::part2);/a \
    map.insert(\"$DAY-1\", day$DAY::part1); \
\n    map.insert(\"$DAY-2\", day$DAY::part2);" src/lib.rs

cargo fmt
