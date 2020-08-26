//! Publish new did document and read it from the tangle
//! cargo run --example resolve_did
use anyhow::Result;
use identity_core::did::DID;
use identity_resolver::resolver::{NetworkNodes, ResolutionMetadata, Resolver};

#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = Resolver::new(NetworkNodes::Com(vec![
        "http://localhost:14265",
        "https://nodes.comnet.thetangle.org:443",
    ]));
    // Mainnet: did:iota:123456789abcdefghi
    // Comnet: did:iota:com:123456789abcdefghi
    // Devnet: did:iota:dev:123456789abcdefghi
    let did = DID::parse_from_str("did:iota:com:123456789abcdefghi")?;
    let document = resolver.resolve(did, ResolutionMetadata {}).await?;
    println!("Document: {:?}", document);
    Ok(())
}
