`checkexec` is a tool to conditionally execute commands 
based on modified timestamps of a target and a dependency list. 

`checkexec` pairs well with [`just`](https://github.com/casey/just) to give the dependency
resolution functionality natively built into `make` but missing from `just`.

##### Why not use `make`

Makefile violates the single responsibility principle by being both a build 
recipe tool and a dependency manager. `just` solves that problem, but it
does not have built-in functionality to resolve dependencies. `checkexec` fills that gap.

# Installation

    cargo install checkexec

# Usage

Here's a simple example to compile a c program only if its source file has been updated.

    checkexec build/my-c-program src/my-c-program.c -- cc -o build/my-c-program src/my-c-program.c

By default, `checkexec` executes the command directly, not in a shell. If you want to use a shell, specify it explicitly.

    checkexec build/my-c-program src/my-c-program.c -- /bin/bash "cc -o build/my-c-program src/my-c-program.c && cp build/my-c-program /usr/local/bin/my-c-program"

Now, let's see `checkexec` in a `Justfile`.

    # Justfile
    build:
        checkexec target/debug/myprogram src/main.rs -- cargo build

This file will only build the program if the source file has been updated, whereas the default `cargo build` will always build the program.