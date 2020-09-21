//! Publish new did document and read it from the tangle
//! cargo run --example resolve_did_stream
use anyhow::Result;
use bytestream::*;
use identity_resolver::resolver::{NetworkNodes, ResolutionInputMetadata, Resolver};
use std::io::Cursor;

#[smol_potat::main]
async fn main() -> Result<()> {
    let resolver = Resolver::new(NetworkNodes::Com(vec![
        "http://localhost:14265",
        "https://nodes.comnet.thetangle.org:443",
    ]))?;
    // Mainnet: did:iota:123456789abcdefghi
    // Comnet: did:iota:com:123456789abcdefghi
    // Devnet: did:iota:dev:123456789abcdefghi
    let mut buffer = Vec::<u8>::new();
    resolver
        .resolve_stream(
            "did:iota:com:123456789abcdefghij".into(),
            ResolutionInputMetadata::default(),
            &mut buffer,
        )
        .await?;
    println!("{:?}", buffer);
    if buffer.is_empty() {
        print!("Resolution was unsuccessful");
        return Ok(());
    }
    let mut cursor = Cursor::new(buffer);
    let did_document_string = String::read_from(&mut cursor, ByteOrder::BigEndian)?;
    println!("{}", did_document_string);
    Ok(())
}
