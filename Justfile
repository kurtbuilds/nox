help:
    @just --list --unsorted

run *args:
    cargo run {{ args }}
alias r := run

install:
    cargo install --path cli
