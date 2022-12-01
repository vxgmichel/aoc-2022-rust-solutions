#!/bin/bash
set -e
BASEDIR=$(realpath $(dirname $0))
for i in {01..01}
do
    echo "Day $i"
    echo "------"
    cd $BASEDIR/day$i
    cargo run --release -q < data.txt
    echo
done