#!/bin/bash
cd ~/gerard
tmux kill-session -t gerard > /dev/null 2>&1
cargo build --release
tmux new -d -s gerard "cargo run --release"
