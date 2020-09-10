//! Publish new did document and read it from the tangle
//! cargo run --example resolve_did
use anyhow::Result;
use identity_resolver::resolver::{NetworkNodes, ResolutionInputMetadata, Resolver};

#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = Resolver::new(NetworkNodes::Com(vec![
        "http://localhost:14265",
        "https://nodes.comnet.thetangle.org:443",
    ]))?;
    // Mainnet: did:iota:123456789abcdefghi
    // Comnet: did:iota:com:123456789abcdefghi
    // Devnet: did:iota:dev:123456789abcdefghi
    let resolution_result = resolver
        .resolve(
            "did:iota:com:123456789abcdefghij".into(),
            ResolutionInputMetadata::default(),
        )
        .await?;
    println!("{:#?}", resolution_result);
    if let Some(document) = resolution_result.did_document {
        println!("Document: {:?}", document);
    }
    Ok(())
}
