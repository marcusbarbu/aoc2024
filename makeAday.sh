#!/bin/sh
DAY=$1
DAY_PATH="./static/day$DAY"

mkdir -p $DAY_PATH
touch $DAY_PATH/test_input
touch $DAY_PATH/real_input
pushd days
cargo new day$DAY
popd
