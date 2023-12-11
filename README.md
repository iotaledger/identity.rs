![banner](https://github.com/iotaledger/identity.rs/raw/HEAD/.github/banner_identity.svg)

<p align="center">
  <a href="https://iota.stackexchange.com/" style="text-decoration:none;"><img src="https://img.shields.io/badge/StackExchange-9cf.svg?logo=stackexchange" alt="StackExchange"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/badge/Discord-9cf.svg?logo=discord" alt="Discord"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/discord/397872799483428865" alt="Discord"></a>
  <a href="https://github.com/iotaledger/identity.rs/blob/HEAD/LICENSE" style="text-decoration:none;"><img src="https://img.shields.io/github/license/iotaledger/identity.rs.svg" alt="Apache 2.0 license"></a>
  <img src="https://deps.rs/repo/github/iotaledger/identity.rs/status.svg" alt="Dependencies">
  <a href='https://coveralls.io/github/iotaledger/identity.rs?branch=main'><img src='https://coveralls.io/repos/github/iotaledger/identity.rs/badge.svg?branch=main' alt='Coverage Status' /></a>

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

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized digital identity, also known as Self-Sovereign Identity (SSI). It implements the W3C [Decentralized Identifiers (DID)](https://www.w3.org/TR/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) specifications. This library can be used to create, resolve and authenticate digital identities and to create verifiable credentials and presentations in order to share information in a verifiable manner and establish trust in the digital world. It does so while supporting secure storage of cryptographic keys, which can be implemented for your preferred key management system. Many of the individual libraries (Rust crates) are agnostic over the concrete DID method, with the exception of some libraries dedicated to implement the [IOTA DID method](https://wiki.iota.org/shimmer/identity.rs/specs/did/iota_did_method_spec/), which is an implementation of decentralized digital identity on the IOTA and Shimmer networks. Written in stable Rust, IOTA Identity has strong guarantees of memory safety and process integrity while maintaining exceptional performance.

## Bindings

[Foreign Function Interface (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface) Bindings of this [Rust](https://www.rust-lang.org/) library to other programming languages:

- [Web Assembly](https://github.com/iotaledger/identity.rs/blob/HEAD/bindings/wasm/) (JavaScript/TypeScript)

## Documentation and Resources

- API References:
  - [Rust API Reference](https://docs.rs/identity_iota/latest/identity_iota/): Package documentation (cargo docs).
  - [Wasm API Reference](https://wiki.iota.org/shimmer/identity.rs/libraries/wasm/api_reference/): Wasm Package documentation.
- [Identity Documentation Pages](https://wiki.iota.org/shimmer/identity.rs/introduction): Supplementing documentation with context around identity and simple examples on library usage.
- [Examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples): Practical code snippets to get you started with the library.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.65)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.65)

## Getting Started

If you want to include IOTA Identity in your project, simply add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
identity_iota = { version = "1.0.0" }
```

To try out the [examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples), you can also do this:

1. Clone the repository, e.g. through `git clone https://github.com/iotaledger/identity.rs`
2. Start a private Tangle as described in the [next section](#example-creating-an-identity)
3. Run the example to create a DID using `cargo run --release --example 0_create_did`

## Example: Creating an Identity

The following code creates and publishes a new IOTA DID Document to a locally running private network.
See the [instructions](https://github.com/iotaledger/hornet/tree/develop/private_tangle) on running your own private network.

_Cargo.toml_

```toml
[package]
name = "iota_identity_example"
version = "1.0.0"
edition = "2021"

[dependencies]
identity_iota = { version = "1.0.0" }
iota-sdk = { version = "1.0.2", default-features = true, features = ["tls", "client", "stronghold"] }
tokio = { version = "1", features = ["full"] }
```

_main._<span></span>_rs_

```rust,no_run
use identity_iota::core::ToJson;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::storage::Storage;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use iota_sdk::client::api::GetAddressesOptions;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::crypto::keys::bip39;
use iota_sdk::types::block::address::Bech32Address;
use iota_sdk::types::block::output::AliasOutput;
use iota_sdk::types::block::output::dto::AliasOutputDto;
use tokio::io::AsyncReadExt;

// The endpoint of the IOTA node to use.
static API_ENDPOINT: &str = "http://127.0.0.1:14265";

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // Create a new Stronghold.
  let stronghold = StrongholdSecretManager::builder()
    .password("secure_password".to_owned())
    .build("./example-strong.hodl")?;

  // Generate a mnemonic and store it in the Stronghold.
  let random: [u8; 32] = rand::random();
  let mnemonic =
    bip39::wordlist::encode(random.as_ref(), &bip39::wordlist::ENGLISH).map_err(|err| anyhow::anyhow!("{err:?}"))?;
  stronghold.store_mnemonic(mnemonic).await?;

  // Create a new secret manager backed by the Stronghold.
  let secret_manager: SecretManager = SecretManager::Stronghold(stronghold);

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  // Get an address from the secret manager.
  let address: Bech32Address = secret_manager
  .generate_ed25519_addresses(
    GetAddressesOptions::default()
      .with_range(0..1)
      .with_bech32_hrp((&network_name).try_into()?),
  )
  .await?[0];

  println!("Your wallet address is: {}", address);
  println!("Please request funds from http://127.0.0.1:8091/, wait for a couple of seconds and then press Enter.");
  tokio::io::stdin().read_u8().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // Insert a new Ed25519 verification method in the DID document.
  let storage: Storage<JwkMemStore, KeyIdMemstore> = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());
  document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address.into(), document, None).await?;
  println!("Alias Output: {}", AliasOutputDto::from(&alias_output).to_json_pretty()?);

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
    "id": "did:iota:tst:0xa947df036e78c2eada8b16e019d517c9e38d4b19cb0c1fa066e752c3074b715d",
    "verificationMethod": [
      {
        "id": "did:iota:tst:0xa947df036e78c2eada8b16e019d517c9e38d4b19cb0c1fa066e752c3074b715d#9KdQCWcvR8kmGPLFOYnTzypsDWsoUIvR",
        "controller": "did:iota:tst:0xa947df036e78c2eada8b16e019d517c9e38d4b19cb0c1fa066e752c3074b715d",
        "type": "JsonWebKey",
        "publicKeyJwk": {
          "kty": "OKP",
          "alg": "EdDSA",
          "kid": "9KdQCWcvR8kmGPLFOYnTzypsDWsoUIvR",
          "crv": "Ed25519",
          "x": "JJoYoeFWU7jWvdQmOKDvM4nZJ2cUbP9yhWZzFgd044I"
        }
      }
    ]
  },
  "meta": {
    "created": "2023-08-29T14:47:26Z",
    "updated": "2023-08-29T14:47:26Z",
    "governorAddress": "tst1qqd7kyu8xadzx9vutznu72336npqpj92jtp27uyu2tj2sa5hx6n3k0vrzwv",
    "stateControllerAddress": "tst1qqd7kyu8xadzx9vutznu72336npqpj92jtp27uyu2tj2sa5hx6n3k0vrzwv"
  }
}
```

## Roadmap and Milestones

For detailed development progress, see the IOTA Identity development [kanban board](https://github.com/orgs/iotaledger/projects/8/views/5).

## Contributing

We would love to have you help us with the development of IOTA Identity. Each and every contribution is greatly valued!

Please review the [contribution](https://wiki.iota.org/shimmer/identity.rs/contribute) and [workflow](https://wiki.iota.org/shimmer/identity.rs/workflow) sections in the [IOTA Wiki](https://wiki.iota.org/).

To contribute directly to the repository, simply fork the project, push your changes to your fork and create a pull request to get them included!

The best place to get involved in discussions about this library or to look for support at is the `#identity` channel on the [IOTA Discord](https://discord.iota.org). You can also ask questions on our [Stack Exchange](https://iota.stackexchange.com/).
