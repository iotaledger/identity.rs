//! Resolve a DID
//! cargo run --example resolve

use anyhow::Result;
use identity_iota::{did::IotaDID, resolver::TangleResolver};
#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = TangleResolver::with_nodes(vec![
        "http://localhost:14265",
        "https://nodes.iota.org:443",
        "https://nodes.thetangle.org:443",
        "https://iotanode.us:14267",
        "https://pow.iota.community:443",
    ]);
    let did = IotaDID::parse("did:iota:com:HbuRS48djS5PbLQciy6iE9BTdaDTBM3GxcbGdyuv3TWo")?;
    let res = resolver.document(&did).await?;
    println!("{:?}", res);
    Ok(())
}
