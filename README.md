Links: [Source](https://github.com/kurtbuilds/checkexec) | [crates.io](https://crates.io/crates/checkexec)

`checkexec` is a tool to conditionally execute commands 
based on modified timestamps of a target and a dependency list. 

`checkexec` pairs well with [`just`](https://github.com/casey/just) to offer a modular and
modern build process and command runner toolkit. `just` fixes numerous problems with
`make`, and `checkexec` adds back the conditional rebuild functionality of `make`.

#### Why not use `make`?

`make` has numerous usability problems which are discussed on the [`just` Readme](https://github.com/casey/just).
`make` also violates the single responsibility principle by being both a build 
recipe tool and a dependency manager. `just` solves that problem, but it
does not have built-in functionality to resolve dependencies. `checkexec` fills that gap.

# Use cases

`checkexec` is great for when you build files from other files. Instead of relying on an 
ecosystem specific tool, you can use `checkexec` as part of any build tool. Here are some examples:

- You build images as part of your build command, and don't want to recompile them unless you need to.
- You build C libaries as part of your Python, Rust, Node (or any other) build process.
- You build Sass/Less/SCSS files and don't want to re-build them unnecessarily.

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

Using this file, `just build` will only build the program if the source file has been updated, whereas the default `cargo build` usage will always build the program.
