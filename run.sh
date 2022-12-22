#!/bin/bash
set -e
BASEDIR=$(realpath $(dirname $0))
for i in {01..21}
do
    echo "Day $i"
    echo "------"
    cd $BASEDIR/day$i
    cargo run --profile=release-with-overflow-checks -q < data.txt
    echo
done
