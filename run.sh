#!/bin/bash

source_code=$1

cargo r $source_code
clang ./tests/output.ll -o ./tests/out
./tests/out
