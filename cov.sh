#!/bin/bash

cargo llvm-cov --lcov --output-path lcov.info --ignore-filename-regex test\.rs
