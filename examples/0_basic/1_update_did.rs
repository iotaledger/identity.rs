use identity_iota::core::ToJson;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::did::MethodScope;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaVerificationMethod;
use identity_iota::iota::NetworkName;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::crypto::keys::bip39;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;
use tokio::io::AsyncReadExt;

// The endpoint of the IOTA node to use.
static API_ENDPOINT: &str = "https://api.testnet.shimmer.network/";

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // Create a new Stronghold.
  let mut stronghold = StrongholdSecretManager::builder()
    .password("secure_password")
    .build("./example-strong.hodl")?;

  // Generate a mnemonic and store it in the Stronghold.
  let keypair = KeyPair::new(KeyType::Ed25519)?;
  let mnemonic = bip39::wordlist::encode(keypair.private().as_ref(), &bip39::wordlist::ENGLISH)
    .map_err(|err| anyhow::anyhow!("{err:?}"))?;
  stronghold.store_mnemonic(mnemonic).await?;

  // Create a new secret manager backed by the Stronghold.
  let secret_manager: SecretManager = SecretManager::Stronghold(stronghold);

  // Get an address from the secret manager.
  let address: Address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

  println!("Your wallet address is: {}", address.to_bech32(network_name.as_ref()));
  println!("Please request funds from https://faucet.testnet.shimmer.network/, then press Enter.");
  tokio::io::stdin().read_u8().await?;

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "#key-1")?;
  document.insert_method(method, MethodScope::VerificationMethod)?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;
  println!("Alias Output: {}", alias_output.to_json()?);

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Published DID document: {:#}", document);

  Ok(())
}
