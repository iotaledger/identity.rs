//! Resolve a DID
//! cargo run --example resolve

use anyhow::Result;
use identity_iota::{core::did::DID, resolver::TangleResolver};
#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = TangleResolver::with_nodes(vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"]);
    let did = DID::parse("did:iota:com:A6HyGa6eXPFgPJkzyqgYPRAsqhoQtkX2bhLf9z4XQNhV")?;
    let res = resolver.document(&did).await?;
    println!("{:?}", res);
    Ok(())
}
