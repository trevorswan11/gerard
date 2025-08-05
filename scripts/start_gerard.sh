#!/bin/bash
cd ~/gerard
tmux kill-session -t gerard-bot > /dev/null 2>&1
tmux new -d -s gerard-bot "cargo run --release"
