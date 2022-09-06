use identity_iota::core::ToJson;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::did::MethodScope;
use identity_iota::iota_core::IotaClientExt;
use identity_iota::iota_core::IotaDocument;
use identity_iota::iota_core::IotaIdentityClientExt;
use identity_iota::iota_core::IotaVerificationMethod;
use identity_iota::iota_core::NetworkName;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

// The endpoint of the IOTA node to use.
static NETWORK_ENDPOINT: &str = "https://127.0.0.1:14265";

/// Demonstrates how to create a DID Document and publish it in a new Alias Output.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(NETWORK_ENDPOINT, None)?.finish()?;

  // Create a new secret manager backed by a Stronghold.
  let secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build("./example-strong.hodl")?,
  );

  // Get an address from the secret manager. The address needs to hold funds.
  let address: Address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;

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
