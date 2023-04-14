# Rust Workshop

[![wiris](https://custom-icon-badges.demolab.com/badge/Powered_by_the_WIRIS_team-red.svg?logo=heart&logoColor=white)](https://www.wiris.com/es/) [![rust](https://img.shields.io/badge/Rust-v1.68.0-orange.svg)](https://www.rust-lang.org/tools/install)

Welcome to our collection of Rust workshops!

## About

Here you will find all those workshops we did develop for anyone interested in learning [Rust](https://www.rust-lang.org/) for the very first time.

The whole repository has been organized as a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), so each workshop is located inside its own directory as `rust-workshop-xx`, where `xx` is the year when the workshop was created (22, 23, etc.).

## Setup

### Install Rust

First things first, you need to have the Rust toolchain installed on your computer. Doing so is as easy as executing the following command line:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Upgrade Rust

In case you already have Rust installed, make sure you got the latest stable version by running the following command:

```bash
rustup update
```

### Clone the repo

We assume you already have [git](https://git-scm.com/) installed on your computer. If this is not the case, just follow the [download instructions](https://git-scm.com/downloads) before continuing.

With the following command line, you will be able to clone up our workshops' repository in your local machine:

```bash
git clone https://github.com/wiris/rust-workshop.git
```

Great! You are ready to code ‍💻
