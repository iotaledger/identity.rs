//! Resolve a DID
//! cargo run --example resolve

use anyhow::Result;
use identity_iota::{
    core::{
        did::DID,
        resolver::{IdentityResolver, ResolutionInput},
    },
    resolver::TangleResolver,
};
#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = TangleResolver::with_nodes(vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"]);
    let did = DID::from("did:iota:com:2BSaGCbQC8a664Eum7Et3wnpQADS43GopDoKs63aU8zS")?;
    let res = resolver.document(&did, &ResolutionInput::new()).await?;
    println!("{:?}", res);
    Ok(())
}
