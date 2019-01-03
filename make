#!/usr/bin/env bash
cargo build --release
python setup.py build_ext --inplace
