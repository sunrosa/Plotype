#!/bin/bash

cargo llvm-cov --open --ignore-filename-regex test\.rs
