#!/bin/bash
set -e
BASEDIR=$(realpath $(dirname $0))
for i in {01..06}
do
    echo "Day $i"
    echo "------"
    cd $BASEDIR/day$i
    cargo run -q < data.txt
    echo
done
