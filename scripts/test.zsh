#!/usr/bin/env zsh

clear
cargo test --workspace
while inotifywait -qe close_write */src/** || true; do
    clear
    cargo test --workspace
done
