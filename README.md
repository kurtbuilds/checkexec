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

`checkexec` is a tool to conditionally execute commands only when files in a dependency list have been updated.

This tool provides the behavior of `make` as a standalone executable, where a command is only run if any of its 
dependencies have been updated. Like `make`, `checkexec` runs a command only if the modified time of any dependency 
is newer than the modified time of the target. 

# Usage

The arguments are: `<target> <dependencies...> -- <command>`. The `--` is a required separator.

    checkexec build/my-bin src/my-program.c -- cc -o build/my-bin src/my-program.c

`checkexec` executes the command directly, so shell constructs like '&&' and '||' are not supported.
You can use `/bin/bash -c` as the command, but escaping is tricky. You're likely better off using two invocations of
`checkexec`.

You can infer the dependency list with `--infer`, where checkexec will inspect the arguments of `<command>` for 
accessible paths. `--infer` will fail if no files are found.

    checkexec build/my-bin --infer -- cc -o build/my-bin src/my-program.c

# Installation

    cargo install checkexec

# Usage Notes

`checkexec` is great for when you build files from other files. Instead of relying on
ecosystem-specific tools, you can use `checkexec` as part of any build tool. Here are some examples:

- You build, resize, or sample images as part of your build command, but don't want to rebuild them unless needed.
- You build C libaries as part of your Python, Rust, Node (or any other) build process.
- You build Sass/Less/SCSS files and don't want to re-build them unnecessarily.

`checkexec` pairs well with:

- [`just`](https://github.com/casey/just), creating a modular and modern build process and command runner. 
  `just` fixes numerous problems with `make`, and `checkexec` adds back the conditional rebuild functionality of `make`.
- [`fd`](https://github.com/sharkdp/fd), making it easy to specify a dependency file list. Example here:

```bash
# Only run your command if a rust file has changed. Note cargo does approximately the 
# same thing natively, but you can easily tailor this structure to a custom case.
checkexec target/debug/hello $(fd -e rs . src) -- cargo build
```

### Exit codes

`checkexec` exit codes behave as you would expect, specifically:

- 0 (success) if the command is not run (i.e. target is up to date)
- 1 if a provided dependency or the command is not found
- Otherwise, when the command is run, checkexec will pass through the command's exit code.

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. 
Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. 
You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star!

<p align="right">(<a href="#top">back to top</a>)</p>
