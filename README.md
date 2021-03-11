![banner](./.meta/identity_banner.png)

<p align="center">
  <a href="https://iota.stackexchange.com/" style="text-decoration:none;"><img src="https://img.shields.io/badge/StackExchange-9cf.svg?logo=stackexchange" alt="StackExchange"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/badge/Discord-9cf.svg?logo=discord" alt="Discord"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/discord/397872799483428865" alt="Discord"></a>
  <a href="https://github.com/iotaledger/identity.rs/blob/master/LICENSE" style="text-decoration:none;"><img src="https://img.shields.io/github/license/iotaledger/bee.svg" alt="Apache 2.0 license"></a>
  <img src="https://deps.rs/repo/github/iotaledger/identity.rs/status.svg" alt="Dependencies">
</p>

<p align="center">
  <a href="#introduction">Introduction</a> ◈
  <a href="#documentation-and-resources">Documentation & Resources</a> ◈
  <a href="#getting-started">Getting Started</a> ◈
  <a href="#example-creating-an-identity">Example</a> ◈
  <a href="#roadmap-and-milestones">Roadmap</a> ◈
  <a href="#contributing">Contributing</a>
</p>

---

## Introduction

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized digital identity, also known as Self-Sovereign Identity (SSI). It implements standards such as the W3C [Decentralized Identifiers (DID)](https://www.w3.org/TR/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) and the DIF [DIDComm Messaging](https://identity.foundation/didcomm-messaging/spec/) specifications alongside supporting methods. This framework can be used to create and authenticate digital identities, creating a trusted connection and sharing verifiable information, establishing trust in the digital world.

The individual libraries are developed to be agnostic about the utilized [Distributed Ledger Technology (DLT)](https://en.wikipedia.org/wiki/Distributed_ledger), with the exception of the [IOTA](https://www.iota.org) integration and higher level libraries. Written in stable Rust, it has strong guarantees of memory safety and process integrity while maintaining exceptional performance.

> :warning: **WARNING** :warning:
> 
> This library is currently in its **alpha stage** and **under development** and might undergo large changes!
> Until a formal third-party security audit has taken place, the [IOTA Foundation](https://www.iota.org/) makes no guarantees to the fitness of this library. As such, it is to be seen as **experimental** and not ready for real-world applications.
> Nevertheless, we are very interested in feedback about user experience, design and implementation, and encourage you to reach out with any concerns or suggestions you may have.

## Documentation and Resources

- [API Reference](https://identity-docs.iota.org/docs/identity/index.html): Package documentation (cargo docs).
- [Identity Documentation Pages](https://identity-docs.iota.org/welcome.html): Supplementing documentation with context around identity and simple examples on library usage.
- [Examples in /examples folder](https://github.com/iotaledger/identity.rs/tree/main/examples): Practical code snippets to get you started with the library.
- [IOTA Identity Experience Team Website](https://iota-community.github.io/X-Team_IOTA_Identity/): Website for a collaborative effort to provide help, guidance and spotlight to the IOTA Identity Community through offering feedback and introducing consistent workflows around IOTA Identity.

## Getting Started

If you want to include IOTA Identity in your project, simply add it as a dependency in your `cargo.toml`:
```rust
[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
```

To try out the [examples](https://github.com/iotaledger/identity.rs/tree/main/examples), you can also do this:

1. Clone the repository, e.g. through `git clone https://github.com/iotaledger/identity.rs `
2. Build the repository with `cargo build `
3. Run your first example using `cargo run --example getting_started `

If you would like to build the [API Reference](https://identity-docs.iota.org/docs/identity/index.html) yourself from source, you can do so using:
```rust
cargo doc --document-private-items --no-deps --open
```

## Example: Creating an Identity

*Cargo.toml*
```rust
[package]
name = "iota_identity_example"
version = "1.0.0"
edition = "2018"

[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
smol = { version = "0.1", features = ["tokio02"] }
smol-potat = { version = "0.3" }
```
*main.*<span></span>*rs*
```rust
use identity::crypto::KeyPair;
use identity::iota::{Client, Document, Network, Result, TangleRef};

#[smol_potat::main] // Using this allows us to have an async main function.
async fn main() -> Result<()> {

  // Create a DID Document (an identity).
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Create a client to interact with the IOTA Tangle.
  let client: Client = Client::new()?;

  // Use the client to publish the DID Document to the IOTA Tangle.
  document.publish(&client).await?;

  // Print the DID Document IOTA transaction link.
  let network: Network = document.id().into();
  let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());
  println!("DID Document Transaction > {}", explore);

  Ok(())
}
```
*Example output*
```rust
DID Document Transaction > https://explorer.iota.org/mainnet/transaction/YARETIQBJLER9BC9U9MOAXEBWVIHXYRMJAYFQSJDCPQXXWSEKQOFKGFMXYCNXPLTRAYQZTCLJJRXZ9999
```
The output link points towards the DID Document transaction, viewable through the IOTA Tangle Explorer, see [here](https://explorer.iota.org/mainnet/transaction/YARETIQBJLER9BC9U9MOAXEBWVIHXYRMJAYFQSJDCPQXXWSEKQOFKGFMXYCNXPLTRAYQZTCLJJRXZ9999). You can see the full DID Document as transaction payload.


## Roadmap and Milestones

For detailed development progress, see also the IOTA Identity development [canban board](https://github.com/iotaledger/identity.rs/projects/3).

IOTA Identity is in heavy development, and will naturally change as it matures and people use it. The chart below isn't meant to be exhaustive, but rather helps to give an idea for some of the areas of development and their relative completion:

#### Basic Framework

| Feature                   | Not started | In Research | In Development | Done | Notes                                                                |
| :------------------------- | :---------: | :------: | :---------------: | :-:  | :-------------------------------------------------------------------- |
| DID Document Manager      |             |          |                        | :heavy_check_mark: | Finished implementation. |
| IOTA Integration          |             |          |                        | :heavy_check_mark: | Finished implementation. |
| Resolver                  |             |          |                        | :heavy_check_mark: | Finished implementation. |
| Stronghold Integration    |             |          | :large_orange_diamond: |                    | Basically done, mostly testing right now. |
| [DID Comms](https://identity.foundation/didcomm-messaging/spec/)                 |             |          | :large_orange_diamond: |                    | Partially done. |
| [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)    |             |          |                        | :heavy_check_mark: | Finished implementation. |
| VC Comms                  | :large_orange_diamond: | |                      |                    | |
| Schema Validation         | :large_orange_diamond: | |                      |                    | |
| C FFI Bindings            | :large_orange_diamond: | |                      |                    | |
| JavaScript FFI Bindings   |             |          | :large_orange_diamond: |                    | Initial implementation done. |
| [WASM Bindings](https://github.com/iotaledger/identity.rs/tree/main/bindings/wasm)             |             | :large_orange_diamond: |          |                    | |
| [Code Examples](https://github.com/iotaledger/identity.rs/tree/main/examples)             |             |          | :large_orange_diamond: |                    | Working on more exhaustive examples. |
| [API Reference](https://identity-docs.iota.org/docs/identity/index.html)             |             |          | :large_orange_diamond: |                    | |
| [mdBook Documentation](https://identity-docs.iota.org/welcome.html)      |             |          | :large_orange_diamond: |                    | |

#### Extended Features (2021+)

| Feature                   | Not started | In Research | In Development | Done | Notes                                                                |
| :------------------------- | :---------: | :------: | :---------------: | :-:  | :-------------------------------------------------------------------- |
| Mobile App Wrapper        | :large_orange_diamond: | |                      |                    | |
| VC Standardization        |             | :large_orange_diamond: |          |                    | Needs quite a bit of research. |
| Identity Agent            |             | :large_orange_diamond: |          |                    | |
| Pairwise DID              | :large_orange_diamond: | |                      |                    | |
| Zero-Knowledge Proofs     | :large_orange_diamond: | |                      |                    | |
| Trust Fabric              | :large_orange_diamond: | |                      |                    | |
| eID Integrations          | :large_orange_diamond: | |                      |                    | |
| IoT Reputation System     | :large_orange_diamond: | |                      |                    | |
| Identity for Objects      | :large_orange_diamond: | |                      |                    | |

#### Planned Milestones

At the current state, the framework is in alpha. As the framework matures we expect to support more and more types of applications. We recommend no use in real-world applications until the consumed libraries are audited, but experimentation and Proof-of-Concept projects are encouraged at the different stages.

| Milestone                 | Topic                                                      | Completion     | Notes                                                                                              |
| :----------------- | :-------------------------------------------------------- | :----- | :------------------------------------------------------------------------------------------------ |
|    1:heavy_check_mark: | [DID](https://www.w3.org/TR/did-core/)                     | Q4 2020 | As the DID standard is implemented and the IOTA ledger is integrated first experimentations are possible. DIDs can be created, updated and ownership can be proven. This allows simple experimentations where ownership of an identity is the main requirement. |
|    2:heavy_check_mark: | [VCs](https://www.w3.org/TR/vc-data-model/)                | Q4 2020 | With the Verifiable Credentials standard implemented, not only ownership can be proven, but also other attributes. At this stage PoCs are possible similarly to [Selv](https://selv.iota.org). However, the communications between actors are not yet implemented, identities are not easily recognized nor are credential layouts standardized. Real-world applications are possible at this stage (after audit), but require extra effort. |
|    3:large_orange_diamond: | [DID Comms](https://identity.foundation/didcomm-messaging/spec/) | Q1 2021 | Once the communications between DID actors have been implemented, any application using identity can communicate out-of-the-box in an interoperable manner. This makes applications easier to develop, yet as mentioned in Milestone 2, identities are still not easily recognized nor are the credential layouts standarized. Real-world applications are therefore easier to develop (after audit), but scaling the application outside of a consortium is difficult. |
|    4+             | TBD                                                        | TBD     | TBD                                                                                                |

## Contributing

We would love to have you help us with the development of IOTA Identity. Each and every contribution is greatly valued!

To contribute directly to the repository, simply fork the project, push your changes to your fork and create a pull request to get them included!

The best place to get involved in discussions about this framework or to look for support at is the `#identity-discussion` channel on the [IOTA Discord](http://discord.iota.org). You can also ask questions on our [Stack Exchange](https://iota.stackexchange.com/).
