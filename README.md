# Rust Lifetimes Guide

This repository contains examples to accompany the Medium article "How Lifetimes Work in Rust." It provides practical demonstrations of Rust's lifetime concepts to help developers understand and apply them effectively.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup & Build Instructions](#setup--build-instructions)
- [Overview of Examples](#overview-of-examples)
- [How to Contribute](#how-to-contribute)
- [License](#license)

## Prerequisites

To run the examples in this repository, you need the Rust toolchain installed, version 1.65 or later. Check your Rust version by running:

```sh
rustc --version
```

If you don’t have Rust installed or need to update it, visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for instructions.

## Setup & Build Instructions

This repository is set up as a Cargo workspace, with each example in its own folder under `examples/`. Follow these steps to run the examples:

### Step 1: Clone the Repository

First, download the repository to your computer:

```sh
git clone https://github.com/kaly7dev/rust-lifetimes-guides.git
cd rust-lifetimes-guides
```

### Step 2: Run an Example

Each example is a separate project. You can run them in two ways:

#### Option 1: From the Root Directory
Use the `cargo run --package` command to run an example without navigating into its folder:

```sh
cargo run --package example1_simple_scope
```

Replace `example1_simple_scope` with `example2_struct_borrow` or `example3_generic_lifetimes` to run the other examples.

#### Option 2: From the Example’s Folder
Navigate to an example’s directory and run it directly:

```sh
cd examples/example1_simple_scope
cargo run
```

Repeat this for the other examples by changing the folder name.


## Overview of Examples

This repository includes three examples, each highlighting a different aspect of Rust lifetimes:

### Example 1: Simple Scope
- **Location**: `examples/example1_simple_scope`
- **Description**: Shows the basics of lifetimes by demonstrating how the scope of variables affects reference validity. Perfect for beginners learning the fundamentals.

### Example 2: Struct Borrow
- **Location**: `examples/example2_struct_borrow`
- **Description**: Explores lifetimes in structs that hold references, ensuring references stay valid as long as the struct exists. A step up in complexity.

### Example 3: Generic Lifetimes
- **Location**: `examples/example3_generic_lifetimes`
- **Description**: Dives into advanced lifetime usage with generic functions or types, showing how to specify lifetime parameters for safe code.

Each example has its own `main.rs` file with Rust code and a `Cargo.toml` file to manage the project.

## How to Contribute

We’d love your help to make this repository better! To contribute:

1. **Fork the Repository**: Click "Fork" on GitHub to create your own copy.
2. **Create a Branch**: Make a new branch for your changes (e.g., `git checkout -b my-improvement`).
3. **Make Changes**: Edit the examples, add new ones, or improve the README.
4. **Commit**: Save your changes with clear messages (e.g., `git commit -m "Added new lifetime example"`).
5. **Push**: Send your branch to your fork (e.g., `git push origin my-improvement`).
6. **Open a Pull Request**: Submit your changes for review on the original repository.

Please document your code well and follow Rust’s coding style for consistency.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.