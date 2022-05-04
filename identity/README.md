## IOTA Identity

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized identity, also known as Self Sovereign Identity (SSI), through the [W3C Decentralized Identifiers (DID)](https://w3c.github.io/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) standards alongside supporting methods, utilizing the [IOTA Distributed Ledger](https://www.iota.org).

## Example

```rust
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::IotaDocument;
use identity::iota::Network;
use identity::iota::Result;
use identity::iota::TangleRef;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client to interact with the IOTA Tangle.
    // Create a new client connected to the Testnet (Chrysalis).
    // Node-syncing has to be disabled for now.
    let client: Client = Client::builder().node_sync_disabled().build().await?;


  // Create a DID Document (an identity).
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: IotaDocument = IotaDocument::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Use the client to publish the DID Document to the IOTA Tangle.
  document.publish(&client).await?;

  // Print the DID Document transaction link.
  let network = Network::from_did(document.id());
  let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());

  println!("DID Document Transaction > {}", explore);

  Ok(())
}
```

**Output**: Example DID Document in the [Tangle Explorer](https://explorer.iota.org/mainnet/transaction/LESUXJUMJCOWGHU9CQQUIHCIPYELOBMHZT9CHCYHJPO9BONQ9IQIFJSREYNOCTYCTQYBHBMBBWJJZ9999).

## Documentation & Community Resources

- [identity.rs](https://github.com/iotaledger/identity.rs): Rust source code of this library on GitHub.
- [Identity Documentation Pages](https://wiki.iota.org/identity.rs/introduction): Supplementing documentation with simple examples on library usage to get you started.
- [More Examples](https://github.com/iotaledger/identity.rs/tree/support/v0.5/examples): Practical examples to get started with the library.
- [IOTA Identity Experience Team Website](https://iota-community.github.io/X-Team_IOTA_Identity/): Website of aforementioned team.

## Structure (Temporary)

- Resources
  - Docs Link (Website & User Guide)
  - X-Team
- Simple Example
- Architecture/Overview
- Get

License: Apache-2.0
