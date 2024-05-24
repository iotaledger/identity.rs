// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::str::FromStr;

use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use identity_sui_name_tbd::resolution::UnmigratedResolver;
use identity_sui_name_tbd::resolution::LOCAL_NETWORK;
use identity_sui_name_tbd::utils::get_client;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManage;
use iota_sdk::crypto::keys::bip39::Mnemonic;
use iota_sdk::crypto::keys::bip44::Bip44;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::crypto::DefaultHash;
use sui_sdk::types::crypto::Signature;
use sui_sdk::types::crypto::SignatureScheme;
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_sdk::types::transaction::Transaction;
use sui_sdk::types::transaction::TransactionData;

const ACCOUNT_INDEX: u32 = 0;
const INTERNAL_ADDRESS: bool = false;
const ADDRESS_INDEX: u32 = 0;
const TEST_MNEMONIC: &str =
  "result crisp session latin must fruit genuine question prevent start coconut brave speak student dismiss";

/// Creates a stronghold path in the temporary directory, whose exact location is OS-dependent.
pub fn stronghold_path() -> PathBuf {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push("001");
  file.set_extension("stronghold");
  file.to_owned()
}

// must be done and can only be done once to import test mnemonic
#[tokio::test]
#[ignore]
async fn can_import_the_test_mnemonic() -> anyhow::Result<()> {
  let stronghold_secret_manager = StrongholdSecretManager::builder()
    .password("secure_password".to_string())
    .build("test.stronghold")
    .expect("Failed to create temporary stronghold");

  stronghold_secret_manager
    .store_mnemonic(Mnemonic::from(TEST_MNEMONIC))
    .await?;

  Ok(())
}

#[tokio::test]
async fn can_initialize_resolver_for_unmigrated_alias_outputs() -> anyhow::Result<()> {
  let result = UnmigratedResolver::new(LOCAL_NETWORK).await;

  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
#[ignore]
async fn can_fetch_alias_output_by_object_id() -> anyhow::Result<()> {
  let resolver = UnmigratedResolver::new(LOCAL_NETWORK).await?;
  let result = resolver
    .get_alias_output("0x669c70a008a5e226813927a7b62a5029306d8f7e7366b0634ef6027b3dbda850")
    .await;

  dbg!(&result);
  assert!(result.is_ok());

  Ok(())
}

#[tokio::test]
async fn can_sign_a_message() -> anyhow::Result<()> {
  let stronghold_secret_manager = StrongholdSecretManager::builder()
    .password("secure_password".to_string())
    .build("test.stronghold")
    .expect("Failed to create temporary stronghold");

  // stronghold_secret_manager
  //   .store_mnemonic(Mnemonic::from(TEST_MNEMONIC))
  //   .await?;

  // serialized tx
  let data: &[u8] = &[
    0, 0, 2, 0, 8, 16, 39, 0, 0, 0, 0, 0, 0, 0, 32, 99, 128, 244, 235, 93, 122, 247, 240, 204, 187, 233, 12, 112, 87,
    11, 181, 255, 12, 156, 255, 214, 241, 218, 171, 221, 98, 11, 202, 210, 215, 253, 16, 2, 2, 0, 1, 1, 0, 0, 1, 1, 3,
    0, 0, 0, 0, 1, 1, 0, 115, 166, 179, 195, 62, 45, 99, 56, 61, 229, 198, 120, 108, 186, 202, 35, 31, 247, 137, 244,
    200, 83, 175, 109, 84, 203, 136, 61, 135, 128, 173, 192, 2, 86, 67, 123, 247, 153, 252, 197, 226, 203, 38, 161, 86,
    142, 90, 239, 112, 39, 9, 77, 143, 4, 87, 199, 140, 127, 174, 79, 156, 223, 83, 73, 240, 0, 0, 0, 0, 0, 0, 0, 0,
    32, 180, 177, 115, 103, 130, 89, 100, 56, 193, 53, 145, 165, 23, 70, 130, 185, 100, 51, 64, 35, 67, 10, 247, 136,
    16, 45, 57, 47, 146, 205, 41, 253, 86, 67, 123, 247, 153, 252, 197, 226, 203, 38, 161, 86, 142, 90, 239, 112, 39,
    9, 77, 143, 4, 87, 199, 140, 127, 174, 79, 156, 223, 83, 73, 240, 0, 0, 0, 0, 0, 0, 0, 0, 32, 180, 177, 115, 103,
    130, 89, 100, 56, 193, 53, 145, 165, 23, 70, 130, 185, 100, 51, 64, 35, 67, 10, 247, 136, 16, 45, 57, 47, 146, 205,
    41, 253, 115, 166, 179, 195, 62, 45, 99, 56, 61, 229, 198, 120, 108, 186, 202, 35, 31, 247, 137, 244, 200, 83, 175,
    109, 84, 203, 136, 61, 135, 128, 173, 192, 1, 0, 0, 0, 0, 0, 0, 0, 16, 39, 0, 0, 0, 0, 0, 0, 0,
  ];

  // build intent message to sign
  let msg: TransactionData = bcs::from_bytes(data)?;
  let intent = Intent::sui_transaction();
  let intent_msg = IntentMessage::new(intent, msg);
  let mut hasher = DefaultHash::default();
  hasher.update(bcs::to_bytes(&intent_msg)?);
  let digest = hasher.finalize().digest;

  // sign with sui  m/44'/784'/0'/0'/0'
  let bip44_chain = Bip44::new(784)
    .with_account(ACCOUNT_INDEX)
    .with_change(INTERNAL_ADDRESS as _)
    .with_address_index(ADDRESS_INDEX);
  let signed = stronghold_secret_manager.sign_ed25519(&digest, bip44_chain).await?;
  println!("signed - sui config: {:?}", &signed);

  dbg!(&signed);

  Ok(())
}

#[tokio::test]
async fn can_submit_a_tx() -> anyhow::Result<()> {
  let client = get_client(LOCAL_NETWORK).await?;

  let stronghold_secret_manager = StrongholdSecretManager::builder()
    .password("secure_password".to_string())
    .build("test.stronghold")
    .expect("Failed to create temporary stronghold");

  let sender = SuiAddress::from_str("0x936accb491f0facaac668baaedcf4d0cfc6da1120b66f77fa6a43af718669973")?;
  let get_flag_call = client
    .transaction_builder()
    .move_call(
      sender,                                                                                    // account
      ObjectID::from_str("0xfc5a7684cb42742fc0d88b4224b02ece1f971fe9fbac4ab620df831ff928e1ad")?, // p id
      "checkin",                                                                                 // module
      "get_flag",                                                                                // fn
      vec![],
      vec![],
      None, // The node will pick a gas object belong to the signer if not provided.
      10000000,
      None,
    )
    .await?;
  dbg!(&get_flag_call);

  // build intent message to sign
  let intent = Intent::sui_transaction();
  let intent_msg = IntentMessage::new(intent, &get_flag_call);
  let mut hasher = DefaultHash::default();
  hasher.update(bcs::to_bytes(&intent_msg)?);
  let digest = hasher.finalize().digest;

  // sign with sui  m/44'/784'/0'/0'/0'
  let bip44_chain = Bip44::new(784)
    .with_account(ACCOUNT_INDEX)
    .with_change(INTERNAL_ADDRESS as _)
    .with_address_index(ADDRESS_INDEX);
  let signed = stronghold_secret_manager.sign_ed25519(&digest, bip44_chain).await?;
  println!("signed - sui config: {:?}", &signed);

  dbg!(&signed);

  // convert to sui signature object
  let binding = [
    [SignatureScheme::ED25519.flag()].as_slice(),
    signed.signature().to_bytes().as_slice(),
    signed.public_key_bytes().to_bytes().as_slice(),
  ]
  .concat();
  let signature_bytes: &[u8] = binding.as_slice();

  let signature = Signature::from_bytes(signature_bytes)?;

  let response = client
    .quorum_driver_api()
    .execute_transaction_block(
      Transaction::from_data(get_flag_call, vec![signature]),
      SuiTransactionBlockResponseOptions::full_content(),
      Some(ExecuteTransactionRequestType::WaitForLocalExecution),
    )
    .await?;

  dbg!(&response);

  dbg!(&response.events);

  Ok(())
}
