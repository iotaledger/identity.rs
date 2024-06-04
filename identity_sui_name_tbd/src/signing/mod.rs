// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::crypto::keys::bip44::Bip44;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use sui_sdk::types::crypto::DefaultHash;
use sui_sdk::types::crypto::Signature;
use sui_sdk::types::crypto::SignatureScheme;
use sui_sdk::types::transaction::TransactionData;

const ACCOUNT_INDEX: u32 = 0;
const INTERNAL_ADDRESS: bool = false;
const ADDRESS_INDEX: u32 = 0;

pub async fn sign_tx(tx_data: &TransactionData) -> anyhow::Result<Signature> {
  // should be part of the instance
  let stronghold_secret_manager = StrongholdSecretManager::builder()
    .password("secure_password".to_string())
    .build("test.stronghold")
    .expect("Failed to create temporary stronghold");

  // build intent message to sign
  let intent = Intent::sui_transaction();
  let intent_msg = IntentMessage::new(intent, &tx_data);
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

  Ok(signature)
}
