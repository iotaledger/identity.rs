//!
//! cargo run --example client_create_document
use identity_core::{
    common::{OneOrMany, ToJson as _},
    did::{DIDDocument as Document, DIDDocumentBuilder as DocumentBuilder, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
    utils::{decode_b58, encode_b58},
};
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder, CreateDocumentResponse, ReadDocumentResponse, TransactionPrinter},
    did::{IotaDID, IotaDocument},
    error::Result,
    network::Network,
};

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
        .key_data(KeyData::PublicKeyBase58(encode_b58(keypair.public())))
        .build()
        .unwrap();

    let doc: Document = DocumentBuilder::default()
        .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
        .id(did.clone().into())
        .public_keys(vec![method])
        .auth(vec![key.clone().into()])
        .agreement(vec![key.clone().into()])
        .delegation(vec![key.clone().into()])
        .invocation(vec![key.into()])
        .build()
        .unwrap();

    println!("[+] Doc (unsigned) > {}", doc.to_json_pretty()?);
    println!("[+]");

    let mut doc: IotaDocument = IotaDocument::try_from_document(doc)?;

    doc.init_timestamps();
    doc.sign(keypair.secret())?;

    println!("[+] Doc (signed) > {:#}", doc);
    println!("[+]");

    let client: Client = ClientBuilder::new()
        .network(Network::Comnet)
        .node("https://nodes.comnet.thetangle.org:443")
        .build()?;

    println!("[+] Client > {:#?}", client);
    println!("[+]");

    let response: CreateDocumentResponse = client
        .create_document(&doc)
        // enable trace debug messages
        .trace(true)
        // publish the document to the tangle
        .send()
        .await?;

    println!("[+] Response > {:#?}", response);
    println!("[+]");

    let printer: TransactionPrinter<'_, _> = TransactionPrinter::hash(&response.tail);

    println!("[+]");
    println!("[+] DID Document Published >");
    println!("[+]   https://comnet.thetangle.org/transaction/{}", printer);
    println!("[+]");

    let response: ReadDocumentResponse = client
        .read_document(&did)
        // enable trace debug messages
        .trace(true)
        // fetch the document from the tangle
        .send()
        .await?;

    println!("[+] Response > {:#?}", response);
    println!("[+]");

    Ok(())
}
