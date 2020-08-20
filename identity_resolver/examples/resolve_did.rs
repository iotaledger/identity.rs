//! Publish new did document and read it from the tangle
//! cargo run --example resolve_did
use anyhow::Result;
use identity_core::did::DID;
use identity_resolver::resolver::{ResolutionMetadata, Resolver};

#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = Resolver {
        nodes: vec!["http://localhost:14265"],
    };
    let did = DID::parse_from_str("did:iota:123456789abcdefghi")?;
    let document = resolver.resolve(did, ResolutionMetadata {}).await?;
    println!("Document: {:?}", document);
    Ok(())
}
