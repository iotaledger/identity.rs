---
title: Getting Started with Rust
sidebar_label: Getting Started
description: Getting started with the IOTA Identity Rust Library.
image: /img/Identity_icon.png
keywords:
- Rust
- Identity
- clone
- build
---
## Introduction

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized digital identity, also known as Self-Sovereign Identity (SSI). It implements standards such as the W3C [Decentralized Identifiers (DID)](https://www.w3.org/TR/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) and the DIF [DIDComm Messaging](https://identity.foundation/didcomm-messaging/spec/) specifications alongside supporting methods. This framework can be used to create and authenticate digital identities, creating a trusted connection and sharing verifiable information, establishing trust in the digital world.

The individual libraries are developed to be agnostic about the utilized [Distributed Ledger Technology (DLT)](https://en.wikipedia.org/wiki/Distributed_ledger), except the [IOTA](https://www.iota.org) integration and higher level libraries. Written in stable Rust, it has strong guarantees of memory safety and process integrity while maintaining exceptional performance.

:::warning 
This library is currently in its **beta stage** and **under development** and might undergo large changes!
Until a formal third-party security audit has taken place, the [IOTA Foundation](https://www.iota.org/) makes no guarantees to the fitness of this library. As such, it is to be seen as **experimental** and not ready for real-world applications.
Nevertheless, we are very interested in feedback about user experience, design and implementation, and encourage you to reach out with any concerns or suggestions you may have.
:::


## Documentation and Resources

- [Examples in /examples folder](https://github.com/iotaledger/identity.rs/tree/main/examples): Practical code snippets to get you started with the library.
- [IOTA Identity Experience Team Website](https://iota-community.github.io/X-Team_IOTA_Identity/): Website for a collaborative effort to provide help, guidance and spotlight to the IOTA Identity Community through offering feedback and introducing consistent workflows around IOTA Identity.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.51)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.51)

## Getting Started

If you want to include IOTA Identity in your project, simply add it as a dependency in your `Cargo.toml`:
```rust
[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
```

To try out the [examples](https://github.com/iotaledger/identity.rs/tree/main/examples), you can also do this:

1. Clone the repository, e.g. through `git clone https://github.com/iotaledger/identity.rs `
2. Build the repository with `cargo build `
3. Run your first example using `cargo run --example getting_started `

If you would like to build the [API Reference](https://wiki.iota.org/identity.rs/libraries/rust/api_reference) yourself from source, you can do so using:

```rust
cargo doc --document-private-items --no-deps --open
```
