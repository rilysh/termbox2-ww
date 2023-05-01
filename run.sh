#!/usr/bin/env sh

cargo build --release && LD_LIBRARY_PATH=./target/release/ ./target/release/termbox2-ww
