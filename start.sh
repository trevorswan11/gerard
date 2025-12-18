#!/bin/bash
GERARD_PATH="~/gerard"

tmux kill-session -t gerard > /dev/null 2>&1
cd $GERARD_PATH
cargo build --release
tmux new-session -d -s gerard target/release/gerard
