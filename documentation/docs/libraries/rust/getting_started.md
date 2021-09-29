---
title: Getting Started with Rust
sidebar_label: Getting Started
description: Getting started with the IOTA Identity Rust Library.
image: /img/Identity_icon.png
keywords:
- Rust
- Identity
---

## Requirements

- [Rust](https://www.rust-lang.org/) (>= 1.51)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.51)

## Include the Library

To include IOTA Identity in your project add it as a dependency in your `Cargo.toml`:

```rust
[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
```

## Examples

To try out the [examples](https://github.com/iotaledger/identity.rs/tree/main/examples), you should:

1. Clone the repository:
  
```bash
git clone https://github.com/iotaledger/identity.rs
```
2. Build the repository:

```bash
cargo build
```
3. Run your first example:

```bash
cargo run --example getting_started
```

## API Reference

If you would like to build the [API Reference](api_reference) yourself from source, you can do so by running the following command:

```rust
cargo doc --document-private-items --no-deps --open
```
