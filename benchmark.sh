#!/bin/bash

cargo build --release && hyperfine --warmup 5 -n "Multiplicative Persistence" -r 50 'cargo run --release'