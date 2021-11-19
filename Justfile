set dotenv-load := false

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    cargo run {{args}}
alias r := run

release:
    cargo build --release

install:
    cargo install --path .

bootstrap:
    cargo install cargo-bump

test:
    cargo test

check:
    cargo check

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || (echo You have untracked changes. Commit your changes before bumping the version. && exit 1)
    cargo bump {{level}}
    cargo update # This bumps Cargo.lock
    git commit -am "Bump {{level}} version"
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

publish:
    cargo publish

patch:
    just version patch
    just publish