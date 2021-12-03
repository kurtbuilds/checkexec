<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/checkexec/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/checkexec.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/checkexec/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/checkexec.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/checkexec/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/checkexec/Run%20Tests?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/checkexec">
    <img src="https://img.shields.io/crates/d/checkexec?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/checkexec">
    <img src="https://img.shields.io/crates/v/checkexec?style=flat-square" alt="Crates.io" />
</a>

</p>

# Checkexec

`checkexec` is a tool to conditionally execute commands based on modified timestamps of a target and a dependency list.

This provides the behavior of Makefile, where a target is only run if the artifact is older than its dependencies.

# Examples

    checkexec build/my-c-program src/my-c-program.c -- cc -o build/my-c-program src/my-c-program.c
    # In this example, the arguments are: <target> <dependency list> -- <command>
    # The -- is a required separator.

By default, `checkexec` executes the command directly, not in a shell. If you need a shell, for example, to use `&&`,
call the shell explicitly.

    checkexec build/my-c-program src/my-c-program.c -- /bin/bash "cc -o build/my-c-program src/my-c-program.c && cp build/my-c-program /usr/local/bin/my-c-program"

You can also infer the dependency list, where checkexec will inspect each argument of the `<command>` for paths that
exist on the filesystem. `--infer` will cause checkexec to fail if it doesn't find any files.

    checkexec build/my-c-program --infer -- cc -o build/my-c-program src/my-c-program.c


# Installation

    cargo install checkexec

# Usage

`checkexec` is great for when you build files from other files. Instead of relying on
ecosystem-specific tools, you can use `checkexec` as part of any build tool. Here are some examples:

- You build, resize, or sample images as part of your build command, but don't want to rebuild them unless needed.
- You build C libaries as part of your Python, Rust, Node (or any other) build process.
- You build Sass/Less/SCSS files and don't want to re-build them unnecessarily.

`checkexec` pairs well with [`just`](https://github.com/casey/just) to offer a modular and
modern build process and command runner. `just` fixes numerous problems with
`make`, and `checkexec` adds back the conditional rebuild functionality of `make`.

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. 
Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. 
You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star!

<p align="right">(<a href="#top">back to top</a>)</p>