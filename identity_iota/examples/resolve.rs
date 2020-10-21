//! Resolve a DID
//! cargo run --example resolve

use anyhow::Result;
use identity_iota::{core::did::DID, resolver::TangleResolver};
#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = TangleResolver::with_nodes(vec![
        "http://localhost:14265",
        "https://nodes.thetangle.org:443",
        "https://iotanode.us:14267",
        "https://pow.iota.community:443",
    ]);
    let did = DID::parse("did:iota:8FuiFcUdXa8Y6yrUkJGjVz6FHiEqDHDRJe1Uwf1jKPZV")?;
    let res = resolver.document(&did).await?;
    println!("{:?}", res);
    Ok(())
}
