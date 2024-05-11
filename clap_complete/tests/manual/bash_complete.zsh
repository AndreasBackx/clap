#!/usr/bin/env zsh

bash --rcfile <(
    echo "source ~/.bashrc"
    cargo run --example dynamic --features unstable-dynamic -- generate --binary "../target/debug/examples/dynamic" bash --behavior ""
)
