# Peparatives Guide

Before getting to the practical use cases, you need Rust installed on your computer. This includes:

* `rustup`: Rust toolchain management tool.
* `rustc`: Rust compiler.
* `cargo`: Rust's package manager.

## Linux and macOS

To download the whole Rust toolchain (which you will need) run:

```sh
curl https://sh.rustup.rs -sSf | sh
```

## Windows

_Follow the installation steps: [forge.rust-lang.org/infra/other-installation-methods](https://forge.rust-lang.org/infra/other-installation-methods.html)_

### Add Rust to your system PATH

In the Rust development environment, all tools are installed to the `~/.cargo/bin` directory, and this is where you will find the Rust toolchain, including rustc, cargo, and rustup.

Note:

> Is customary for Rust developers to include this directory in their PATH environment variable. During installation rustup will attempt to configure the PATH. Because of differences between platforms, command shells, and bugs in rustup, the modifications to PATH may not take effect until the terminal is restarted, the user is logged out, or it may not succeed at all

### Options

1. Restart your terminal and/or restart your computer, or if you don't feel like it you can also add Rust to PATH manually:

    ```sh
    source $HOME/.cargo/env
    ```

2. Add to your bash/zsh profile:

    ```sh
    export PATH="$HOME/.cargo/bin:$PATH"
    ```

## Hands on

To confirm that your set up is working well and that you are ready to start coding some Rust open a terminal on the root of this directory (where this file you are reading now is located) and run:

```console
$ cargo build

Finished dev [unoptimized + debuginfo] target(s) in 2.64s
```

to build the project.

To get a greet from _ferris_ run:

```console
$ cargo run
   Compiling rust-workshop v0.1.0 (/Users/francesc/Wiris/projects/rust-workshop)
    Finished dev [unoptimized + debuginfo] target(s) in 1.46s
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

As a bonus, run

```sh
cargo test 
```

to run the test set for the exercises on this workshop. **Don't panic!** None of the tests pass at this point but you are good to go to [Practical Cases.md](./PRACTICAL_CASE.md) and start coding your way through!

## Extra: VSCode as IDE

During this Workshop we reccomend using [Visual Studio Code](https://code.visualstudio.com) as the IDE to implement your solution(s). However feel free to skip this section and use any other IDE you like.

### Download

To download and install VSCode to your computer go to <https://code.visualstudio.com/Download> and choose the appropiate installer for your platform.

### Extensions

Once VSCode is installed open this project folder with it. Then go to the _Extensions_ on the left panel and install all the reccomended extensions for this project.  

### Debugging the project
