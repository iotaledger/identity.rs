![banner](https://github.com/iotaledger/identity.rs/raw/HEAD/documentation/static/img/Banner/banner_identity.svg)

<p align="center">
  <a href="https://iota.stackexchange.com/" style="text-decoration:none;"><img src="https://img.shields.io/badge/StackExchange-9cf.svg?logo=stackexchange" alt="StackExchange"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/badge/Discord-9cf.svg?logo=discord" alt="Discord"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/discord/397872799483428865" alt="Discord"></a>
  <a href="https://github.com/iotaledger/identity.rs/blob/HEAD/LICENSE" style="text-decoration:none;"><img src="https://img.shields.io/github/license/iotaledger/identity.rs.svg" alt="Apache 2.0 license"></a>
  <img src="https://deps.rs/repo/github/iotaledger/identity.rs/status.svg" alt="Dependencies">
  <a href='https://coveralls.io/github/iotaledger/identity.rs?branch=dev'><img src='https://coveralls.io/repos/github/iotaledger/identity.rs/badge.svg?branch=dev' alt='Coverage Status' /></a>

</p>

<p align="center">
  <a href="#introduction">Introduction</a> ◈
  <a href="#bindings">Bindings</a> ◈
  <a href="#documentation-and-resources">Documentation & Resources</a> ◈
  <a href="#getting-started">Getting Started</a> ◈
  <a href="#example-creating-an-identity">Example</a> ◈
  <a href="#roadmap-and-milestones">Roadmap</a> ◈
  <a href="#contributing">Contributing</a>
</p>

---

## Introduction

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized digital identity, also known as Self-Sovereign Identity (SSI). It implements standards such as the W3C [Decentralized Identifiers (DID)](https://www.w3.org/TR/did-core/), [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/), and the DIF [DIDComm Messaging](https://identity.foundation/didcomm-messaging/spec/) specifications alongside supporting methods. This framework can be used to create and authenticate digital identities, creating a trusted connection and sharing verifiable information, establishing trust in the digital world.

The individual libraries are developed to be agnostic about the utilized [Distributed Ledger Technology (DLT)](https://en.wikipedia.org/wiki/Distributed_ledger), with the exception of the [IOTA](https://www.iota.org) integration and higher level libraries. Written in stable Rust, it has strong guarantees of memory safety and process integrity while maintaining exceptional performance.

> ⚠️ **WARNING** ⚠️
>
> This library is currently in its **beta stage** and **under development** and might undergo large changes!
> Until a formal third-party security audit has taken place, the [IOTA Foundation](https://www.iota.org/) makes no guarantees to the fitness of this library. As such, it is to be seen as **experimental** and not ready for real-world applications.
> Nevertheless, we are very interested in feedback about user experience, design and implementation, and encourage you to reach out with any concerns or suggestions you may have.

## Bindings

[Foreign Function Interface (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface) Bindings of this [Rust](https://www.rust-lang.org/) library to other programming languages are a work in progress (see Roadmap below). Currently available bindings are:

- [Web Assembly](https://github.com/iotaledger/identity.rs/blob/HEAD/bindings/wasm/) (JavaScript/TypeScript)

## Documentation and Resources

- [API Reference](https://wiki.iota.org/identity.rs/libraries/rust/api_reference): Package documentation (cargo docs).
- [Identity Documentation Pages](https://wiki.iota.org/identity.rs/introduction): Supplementing documentation with context around identity and simple examples on library usage.
- [Examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples): Practical code snippets to get you started with the library.
- [IOTA Identity Experience Team Website](https://iota-community.github.io/X-Team_IOTA_Identity/): Website for a collaborative effort to provide help, guidance and spotlight to the IOTA Identity Community through offering feedback and introducing consistent workflows around IOTA Identity.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.62)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.62)

## Getting Started

If you want to include IOTA Identity in your project, simply add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
identity_iota = { version = "0.6" }
```

To try out the [examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples), you can also do this:

1. Clone the repository, e.g. through `git clone https://github.com/iotaledger/identity.rs`
2. Build the repository with `cargo build `
3. Run your first example using `cargo run --example getting_started`

## Example: Creating an Identity

The following code creates and publishes a new IOTA DID Document to the Tangle Mainnet.

_Cargo.toml_

```toml
[package]
name = "iota_identity_example"
version = "1.0.0"
edition = "2021"

[dependencies]
identity_iota = { version = "0.7" }
iota-client = { version = "2.0.0-beta.3", default-features = false, features = ["tls", "stronghold"] }
tokio = { version = "1", features = ["full"] }
```

_main._<span></span>_rs_

```rust,no_run
use identity_iota::core::ToJson;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::did::MethodScope;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaVerificationMethod;
use identity_iota::iota::NetworkName;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

// The endpoint of the IOTA node to use.
static NETWORK_ENDPOINT: &str = "https://127.0.0.1:14265";

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(NETWORK_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build("./example-strong.hodl")?,
  );

  // Get an address from the secret manager. The address needs to hold funds.
  let address: Address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-1")?;
  document.insert_method(method, MethodScope::VerificationMethod)?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;
  println!("Alias Output: {}", alias_output.to_json()?);

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Published DID document: {:#}", document);

  Ok(())
}
```

_Example output_

```json
{
  "doc": {
    "id": "did:iota:rms:0x4113f08e360a3c1725bb1f93d94f6e807a1ef88f091d45f93513c3e88dac3248",
    "verificationMethod": [
      {
        "id": "did:iota:rms:0x4113f08e360a3c1725bb1f93d94f6e807a1ef88f091d45f93513c3e88dac3248#key-1",
        "controller": "did:iota:rms:0x4113f08e360a3c1725bb1f93d94f6e807a1ef88f091d45f93513c3e88dac3248",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "z7BoQerJn9NxwcA4KHGK9CP5FJRRZJBmsxrGPvWiyuFGG"
      }
    ]
  },
  "meta": {
    "created": "2022-09-06T12:12:11Z",
    "updated": "2022-09-06T12:12:11Z"
  }
}
```

## Roadmap and Milestones

For detailed development progress, see the IOTA Identity development [kanban board](https://github.com/orgs/iotaledger/projects/8/views/5).

IOTA Identity is in heavy development, and will naturally change as it matures and people use it. The chart below isn't meant to be exhaustive, but rather helps to give an idea for some of the areas of development and their relative completion:

#### Basic Framework

|                                       Feature                                       | Not started | In Research | In Development | Done |          Notes           |
| :---------------------------------------------------------------------------------: | :---------: | :---------: | :------------: | :--: | :----------------------: |
| [IOTA DID Method](https://wiki.iota.org/identity.rs/specs/did/iota_did_method_spec) |             |             |                |  ✔️  | Finished implementation. |
|           [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)            |             |             |                |  ✔️  | Finished implementation. |
|                                       Account                                       |             |             |                |  ✔️  | Finished implementation. |
| [Identity Agent](https://github.com/iotaledger/identity.rs/tree/dev/identity_agent) |             |             |       🔶       |      |                          |
|         [DIDComm](https://wiki.iota.org/identity.rs/specs/didcomm/overview)         |             |             |       🔶       |      |  In-progress with Agent  |
|                                Selective Disclosure                                 |             |     🔶      |                |      |                          |
|                                Zero Knowledge Proofs                                |             |     🔶      |                |      |                          |
|                                Support Embedded Rust                                |             |     🔶      |                |      |                          |
| [WASM Bindings](https://github.com/iotaledger/identity.rs/blob/HEAD/bindings/wasm)  |             |             |                |  ✔️  | Finished implementation. |
|    [Code Examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples)    |             |             |                |  ✔️  |                          |
|       [Documentation Portal](https://wiki.iota.org/identity.rs/introduction)        |             |             |       🔶       |      |                          |

#### Next Milestones

At the current state, the framework is in beta. As the framework matures we expect to support more and more types of applications. We recommend no use in real-world applications until the consumed libraries are audited, but experimentation and Proof-of-Concept projects are encouraged at the different stages.

The next milestone is the release of version 1.0, which will stabilize the APIs, support backwards compatibility and versioned identities. This makes updating to future versions much easier. In addition it will provide full documentation coverage and the release will be audited.

Afterwards, we are already planning a future update containing privacy enhancing features such as Selective Disclosure and Zero Knowledge Proofs.

## Contributing

We would love to have you help us with the development of IOTA Identity. Each and every contribution is greatly valued!

Please review the [contribution](https://wiki.iota.org/identity.rs/contribute) and [workflow](https://wiki.iota.org/identity.rs/workflow) sections in the [IOTA Wiki](https://wiki.iota.org/).

To contribute directly to the repository, simply fork the project, push your changes to your fork and create a pull request to get them included!

The best place to get involved in discussions about this framework or to look for support at is the `#identity` channel on the [IOTA Discord](https://discord.iota.org). You can also ask questions on our [Stack Exchange](https://iota.stackexchange.com/).
