#!/bin/bash
set -e
BASEDIR=$(realpath $(dirname $0))
for i in {01..14}
do
    echo "Day $i"
    echo "------"
    cd $BASEDIR/day$i
    cargo run $1 -q < data.txt
    echo
done
