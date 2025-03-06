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

> [!NOTE]
> This version of the library is compatible with IOTA Rebased networks and in active development, for a version of the library compatible with IOTA Stardust networks check [here](https://github.com/iotaledger/identity.rs/)

## Introduction

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized digital identity, also known as Self-Sovereign Identity (SSI). It implements the W3C [Decentralized Identifiers (DID)](https://www.w3.org/TR/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) specifications. This library can be used to create, resolve and authenticate digital identities and to create verifiable credentials and presentations in order to share information in a verifiable manner and establish trust in the digital world. It does so while supporting secure storage of cryptographic keys, which can be implemented for your preferred key management system. Many of the individual libraries (Rust crates) are agnostic over the concrete DID method, with the exception of some libraries dedicated to implement the [IOTA DID method](https://docs.iota.org/references/iota-identity/iota-did-method-spec/), which is an implementation of decentralized digital identity on IOTA Rebased networks. Written in stable Rust, IOTA Identity has strong guarantees of memory safety and process integrity while maintaining exceptional performance.

<!-- ## Bindings

[Foreign Function Interface (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface) Bindings of this [Rust](https://www.rust-lang.org/) library to other programming languages:

- [Web Assembly](https://github.com/iotaledger/identity.rs/blob/feat/identity-rebased-alpha/bindings/wasm/identity_wasm/) (JavaScript/TypeScript) -->

## gRPC

We provide a collection of experimental [gRPC services](https://github.com/iotaledger/identity.rs/blob/feat/identity-rebased-alpha/bindings/grpc/)
## Documentation and Resources

- API References:
  - [Rust API Reference](https://iotaledger.github.io/identity.rs/identity_iota/index.html): Package documentation (cargo docs).
  <!-- - [Wasm API Reference](https://wiki.iota.org/identity.rs/libraries/wasm/api_reference/): Wasm Package documentation. -->
- [Identity Documentation Pages](https://docs.iota.org/iota-identity): Supplementing documentation with context around identity and simple examples on library usage.
- [Examples](https://github.com/iotaledger/identity.rs/blob/feat/identity-rebased-alpha/examples): Practical code snippets to get you started with the library.

## Universal Resolver

IOTA Identity includes a [Universal Resolver](https://github.com/decentralized-identity/universal-resolver/) driver implementation for the `did:iota` method. The Universal Resolver is a crucial component that enables the resolution of DIDs across different DID methods.

Our implementation allows for resolving IOTA DIDs through the standardized Universal Resolver interface, supporting multiple networks including testnet, devnet, and custom networks. The resolver is available as a Docker container for easy deployment and integration.

For more information and implementation details, visit our [Universal Resolver Driver Repository](https://github.com/iotaledger/uni-resolver-driver-iota).

### Quick Start with Docker

```bash
# Pull and run the Universal Resolver driver
docker run -p 8080:8080 iotaledger/uni-resolver-driver-iota

# Resolve a DID
curl -X GET http://localhost:8080/1.0/identifiers/did:iota:0xf4d6f08f5a1b80dd578da7dc1b49c886d580acd4cf7d48119dfeb82b538ad88a
```

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.83)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.83)

## Getting Started

If you want to include IOTA Identity in your project, simply add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
identity_iota = { git = "https://github.com/iotaledger/identity.rs.git", tag = "v1.6.0-alpha" }
```

To try out the [examples](https://github.com/iotaledger/identity.rs/blob/feat/identity-rebased-alpha/examples), you can also do this:

1. Clone the repository, e.g. through `git clone https://github.com/iotaledger/identity.rs`
2. Get the [IOTA binaries](https://github.com/iotaledger/iota/releases).
3. Start a local network for testing with `iota start --force-regenesis --with-faucet`.
4. Request funds with `iota client faucet`.
5. Publish a test identity package to your local network: `./identity_iota_core/scripts/publish_identity_package.sh`.
6. Get the `packageId` value from the output (the entry with `"type": "published"`) and pass this as `IOTA_IDENTITY_PKG_ID` env value.
7. Run the example to create a DID using `IOTA_IDENTITY_PKG_ID=(the value from previous step)  run --release --example 0_create_did`

## Example: Creating an Identity

The following code creates and publishes a new IOTA DID Document to a locally running private network.
See the [instructions](https://github.com/iotaledger/iota/docker/iota-private-network) on running your own private network for development.

_Cargo.toml_

<!--
Test this example using https://github.com/anko/txm: `txm README.md`

!test program
cd ../../..
mkdir tmp
cat | sed -e 's#identity_iota = { git = "[^"]*", tag = "[^"]*"#identity_iota = { path = "../identity_iota"#' > tmp/Cargo.toml
echo '[workspace]' >>tmp/Cargo.toml
-->
<!-- !test check Cargo Example -->

```toml
[package]
name = "iota_identity_example"
version = "1.0.0"
edition = "2021"

[dependencies]
anyhow = "1.0.62"
identity_iota = { git = "https://github.com/iotaledger/identity.rs.git", tag = "v1.6.0-alpha", features = ["memstore"] }
secret-storage = { git = "https://github.com/iotaledger/secret-storage.git", tag = "v0.2.0" }
iota-sdk = { git = "https://github.com/iotaledger/iota.git", package = "iota-sdk", tag = "v0.9.2-rc" }
rand = "0.8.5"
tokio = { version = "1", features = ["full"] }
```

_main._<span></span>_rs_

<!--
Test this example using https://github.com/anko/txm: `txm README.md`

!test program
cd ../..
mkdir tmp/src
cat > tmp/src/main.rs 
cd tmp
timeout 360 cargo build || (echo "Process timed out after 360 seconds" && exit 1)
-->
<!-- !test check Rust Example -->

```rust,no_run
use anyhow::Context;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::rebased::client::IdentityClient;
use identity_iota::iota::rebased::client::IdentityClientReadOnly;
use identity_iota::iota::rebased::transaction::Transaction;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwkStorage;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::storage::KeyType;
use identity_iota::storage::Storage;
use identity_iota::storage::StorageSigner;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use iota_sdk::IotaClientBuilder;
use iota_sdk::types::base_types::IotaAddress;
use secret_storage::Signer;
use tokio::io::AsyncReadExt;

/// Demonstrates how to create a DID Document and publish it in a new identity.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let iota_client = IotaClientBuilder::default()
    .build_localnet()
    .await
    .map_err(|err| anyhow::anyhow!(format!("failed to connect to network; {}", err)))?;

  // Create new storage and generate new key.
  let storage = Storage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let generate = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;
  let public_key_jwk = generate.jwk.to_public().expect("public components should be derivable");
  let signer = StorageSigner::new(&storage, generate.key_id, public_key_jwk);
  let sender_address = {
    let public_key = Signer::public_key(&signer).await?;
    IotaAddress::from(&public_key)
  };
  let package_id = std::env::var("IOTA_IDENTITY_PKG_ID")
    .map_err(|e| {
      anyhow::anyhow!("env variable IOTA_IDENTITY_PKG_ID must be set in order to run the examples").context(e)
    })
    .and_then(|pkg_str| pkg_str.parse().context("invalid package id"))?;

  // Create identity client with signing capabilities.
  let read_only_client = IdentityClientReadOnly::new_with_pkg_id(iota_client, package_id).await?;
  let identity_client = IdentityClient::new(read_only_client, signer).await?;

  println!("Your wallet address is: {}", sender_address);
  println!("Please request funds from http://127.0.0.1:9123/gas, wait for a couple of seconds and then press Enter.");
  tokio::io::stdin().read_u8().await?;

  // Create a new DID document with a placeholder DID.
  let mut unpublished: IotaDocument = IotaDocument::new(identity_client.network());
  unpublished
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  // Publish new DID document.
  let document = identity_client
    .publish_did_document(unpublished)
    .execute(&identity_client)
    .await?
    .output;

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
  }
}
```

## Roadmap and Milestones

For detailed development progress, see the IOTA Identity development [kanban board](https://github.com/orgs/iotaledger/projects/8/views/5).

## Contributing

We would love to have you help us with the development of IOTA Identity. Each and every contribution is greatly valued!

Please review the [contribution](https://wiki.iota.org/identity.rs/contribute) and [workflow](https://wiki.iota.org/identity.rs/workflow) sections in the [IOTA Wiki](https://wiki.iota.org/).

To contribute directly to the repository, simply fork the project, push your changes to your fork and create a pull request to get them included!

The best place to get involved in discussions about this library or to look for support at is the `#identity` channel on the [IOTA Discord](https://discord.iota.org). You can also ask questions on our [Stack Exchange](https://iota.stackexchange.com/).
