//!
//! cargo run --example client_send_transfer
use identity_core::{
    common::{OneOrMany, ToJson as _},
    did::{DIDDocument as Document, DIDDocumentBuilder as DocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::{decode_b58, encode_b58},
};
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder, SendTransferResponse},
    did::IotaDID,
    error::Result,
    network::Network,
};
use iota::{client::Transfer, transaction::bundled::Address};

const PUBLIC: &str = "66jqh9UeDQ5p88YZvv8F9qNuUxQPQfuYLc6njpsVC95u";
const SECRET: &str = "5E7sd95ArqK52mmt6BVXw7Hod8vAgSkSSqwTamYH1HhtdQrBrL8EqCmYbui4Bg12oZtLU6oXkq2qXS4q7BWd8JFm";

#[smol_potat::main]
async fn main() -> Result<()> {
    let keypair: KeyPair = KeyPair::new(decode_b58(PUBLIC)?.into(), decode_b58(SECRET)?.into());

    println!("[+] Public > {}", encode_b58(keypair.public()));
    println!("[+]");

    println!("[+] Secret > {}", encode_b58(keypair.secret()));
    println!("[+]");

    let did: IotaDID = IotaDID::with_network(keypair.public().as_ref(), "com")?;

    println!("[+] DID > {}", did);
    println!("[+]");

    let key: DID = format!("{}#key-1", did).parse()?;

    println!("[+] Key > {}", key);
    println!("[+]");

    let method: PublicKey = PublicKeyBuilder::default()
        .id(key.clone())
        .controller(did.clone().into())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(PUBLIC.to_string()))
        .build()
        .unwrap();

    let doc: Document = DocumentBuilder::default()
        .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        .id(did.clone().into())
        .public_keys(vec![method])
        .auth(vec![key.into()])
        .build()
        .unwrap();

    println!("[+] Doc > {}", doc.to_json_pretty()?);
    println!("[+]");

    let address: Address = did.create_address()?;
    let message: String = doc.to_json()?;

    let transfer: Transfer = Transfer {
        address,
        value: 0,
        message: Some(message),
        tag: None,
    };

    println!("[+] Transfer > {:#?}", transfer);
    println!("[+]");

    let client: Client = ClientBuilder::new()
        .network(Network::Comnet)
        .node("https://nodes.comnet.thetangle.org:443")
        .build()?;

    println!("[+] Client > {:#?}", client);
    println!("[+]");

    let response: SendTransferResponse = client
        .send_transfer()
        // enable trace debug messages
        .trace(true)
        // send the transfer to the tangle
        .send(transfer)
        .await?;

    println!("[+] Response > {:#?}", response);
    println!("[+]");

    Ok(())
}
