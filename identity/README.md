# identity

## IOTA Identity
IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized identity, also known as Self Sovereign Identity (SSI), through the [W3C Decentralized Identifiers (DID)](https://w3c.github.io/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) standards alongside supporting methods, utilizing the [IOTA Distributed Ledger](https://www.iota.org).


## Example
```rust
use identity::iota::Client;
use identity::iota::IotaDocument;
use identity::crypto::KeyPair;
use identity::iota::Result;

#[smol_potat::main]
async fn main() -> Result<()> {

  // Create a client to interact with the IOTA Tangle.
  let client: Client = Client::new()?;

  // Create a DID Document (an identity).
  let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder()
    .authentication_tag("key-1")
    .did_network(client.network().as_str())
    .build()?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Use the client to publish the DID Document to the IOTA Tangle.
  let transaction: _ = client.publish_document(&document).await?;

  // Print the DID Document transaction link.
  println!("DID Document Transaction > {}", client.transaction_url(&transaction));

  Ok(())
}

```

**Output**: Example DID Document in the [Tangle Explorer](https://explorer.iota.org/mainnet/transaction/LESUXJUMJCOWGHU9CQQUIHCIPYELOBMHZT9CHCYHJPO9BONQ9IQIFJSREYNOCTYCTQYBHBMBBWJJZ9999).

## Documentation & Community Resources
- [identity.rs](https://github.com/iotaledger/identity.rs): Rust source code of this library on GitHub.
- [Identity Documentation Pages](https://identity.docs.iota.org/welcome.html): Supplementing documentation with simple examples on library usage to get you started.
- [More Examples](https://github.com/iotaledger/identity.rs/tree/dev/examples): Practical examples to get started with the library.
- [IOTA Identity Experience Team Website](https://iota-community.github.io/X-Team_IOTA_Identity/): Website of aforementioned team.

## Structure (Temporary)

- Resources
  - Docs Link (Website & User Guide)
  - X-Team
- Simple Example
- Architecture/Overview
- Get



License: Apache-2.0
