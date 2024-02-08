
use examples::get_address_with_funds;
use examples::random_stronghold_path;
use examples::MemStorage;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::NetworkName;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsDocumentExtPQC;
use identity_iota::storage::KeyIdMemstore;
use identity_iota::storage::KeyType;
use identity_iota::verification::jws::JwsAlgorithmPQ;
use identity_iota::verification::MethodScope;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::output::AliasOutput;

// The API endpoint of an IOTA node, e.g. Hornet.
const API_ENDPOINT: &str = "http://localhost:14265";
// The faucet endpoint allows requesting funds for testing purposes.
const FAUCET_ENDPOINT: &str = "http://localhost:8091/api/enqueue";


// const api_endpoint: &str = "https://api.testnet.shimmer.network";
// const faucet_endpoint: &str = "https://faucet.testnet.shimmer.network/api/enqueue";



async fn create_did(client: &Client, secret_manager: &SecretManager, storage: &MemStorage, key_type: KeyType, alg: JwsAlgorithmPQ ) -> anyhow::Result<(Address, IotaDocument, String)> {

  // Get an address with funds for testing.
  let address: Address = get_address_with_funds(&client, &secret_manager, FAUCET_ENDPOINT).await?;

  // Get the Bech32 human-readable part (HRP) of the network.
  let network_name: NetworkName = client.network_name().await?;
  
  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  let mut document: IotaDocument = IotaDocument::new(&network_name);

  // New Verification Method containing a BBS+ key
  let fragment = document.generate_method_pqc(
    &storage, 
    key_type, 
    alg, 
    None, 
    MethodScope::VerificationMethod
  ).await?;

  // Construct an Alias Output containing the DID document, with the wallet address
  // set as both the state controller and governor.
  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  // Publish the Alias Output and get the published DID document.
  let document: IotaDocument = client.publish_did_output(&secret_manager, alias_output).await?;
  println!("Published DID document: {document:#}");

  Ok((address, document, fragment))
}


/// Demonstrates how to create an Anonymous Credential with BBS+.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ===========================================================================
  // Step 1: Create identitiy for the issuer.
  // ===========================================================================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;


  let mut secret_manager_issuer = SecretManager::Stronghold(StrongholdSecretManager::builder()
  .password(Password::from("secure_password_1".to_owned()))
  .build(random_stronghold_path())?);

  
  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let (_, issuer_document, fragment_issuer): (Address, IotaDocument, String) = 
  create_did(&client, &mut secret_manager_issuer, &storage_issuer, JwkMemStore::ML_DSA_KEY_TYPE, JwsAlgorithmPQ::ML_DSA_44).await?;

  Ok(())
}