//!
//! cargo run --example client_read_transactions
use identity_core::utils::{decode_b58, encode_b58};
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder, ReadTransactionsResponse},
    did::IotaDID,
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

    let client: Client = ClientBuilder::new()
        .network(Network::Comnet)
        .node("https://nodes.comnet.thetangle.org:443")
        .build()?;

    println!("[+] Client > {:#?}", client);
    println!("[+]");

    let response: ReadTransactionsResponse = client
        .read_transactions(&did)
        // enable trace debug messages
        .trace(true)
        // read transactions from the tangle
        .send()
        .await?;

    println!("[+] Response > {:#?}", response.messages);
    println!("[+]");

    Ok(())
}
