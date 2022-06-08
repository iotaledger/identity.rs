// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::common::Object;

use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::FmtJson;
use identity_did::did::CoreDID;
use identity_did::document::CoreDocument;
use lazy_static::lazy_static;

use crate::error::Result;

/// An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
/// merged with one or more diff messages.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StardustDocument(CoreDocument<CoreDID>);

lazy_static! {
  static ref PLACEHOLDER_DID: CoreDID = {
    CoreDID::parse("did:stardust:00000000000000000000000000000000").unwrap()
  };
}

impl StardustDocument {
  /// Constructs an empty DID Document with a [`placeholder_did`] identifier.
  pub fn new() -> StardustDocument {
    Self(
      // Constructing an empty DID Document is infallible.
      CoreDocument::builder(Object::default())
        .id(Self::placeholder_did().clone())
        .build()
        .expect("empty StardustDocument constructor failed")
    )
  }

  pub fn placeholder_did() -> &'static CoreDID {
    &PLACEHOLDER_DID
  }
}


impl Display for StardustDocument {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt_json(f)
  }
}


#[cfg(test)]
mod tests {
  use identity_core::crypto::KeyType;
  use super::*;

  #[test]
  fn test_new() {
    let document: StardustDocument = StardustDocument::new();
    assert_eq!(document.0.id(), StardustDocument::placeholder_did());
  }

  use iota_client::{
    bee_block::{
      output::{
        feature::{IssuerFeature, MetadataFeature, SenderFeature},
        unlock_condition::{
          GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition, UnlockCondition,
        },
        AliasId, AliasOutputBuilder, Feature, Output, OutputId,
      },
      payload::{transaction::TransactionEssence, Payload},
    },
    constants::SHIMMER_TESTNET_BECH32_HRP,
    request_funds_from_faucet,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
  };
  use iota_client::crypto::keys::bip39;

  /// In this example we will create an alias output
  #[tokio::test]
  async fn main() -> Result<()> {
    let endpoint = "http://localhost:14265";
    let faucet = format!("{endpoint}/api/plugins/faucet/v1/enqueue");

    // let keypair = identity_core::crypto::KeyPair::new(KeyType::Ed25519).unwrap();
    // println!("PrivateKey: {}", keypair.private().to_string());
    // let mnemonic = bip39::wordlist::encode(keypair.private().as_ref(),&bip39::wordlist::ENGLISH).unwrap();
    let mnemonic = "veteran provide abstract express quick another fee dragon trend extend cotton tail dog truly angle napkin lunch dinosaur shrimp odor gain bag media mountain";
    println!("Mnemonic: {}", mnemonic);
    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&mnemonic)?);

    // Create a client instance.
    let client = Client::builder()
      .with_node(endpoint)?
      .with_node_sync_disabled()
      .finish()
      .await?;

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    request_funds_from_faucet(
      &faucet,
      &address.to_bech32(SHIMMER_TESTNET_BECH32_HRP),
    )
      .await?;
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    //////////////////////////////////
    // create new alias output
    //////////////////////////////////
    let outputs = vec![
      AliasOutputBuilder::new_with_amount(1_000_000, AliasId::null())?
        .with_state_index(0)
        .with_foundry_counter(0)
        .add_feature(Feature::Sender(SenderFeature::new(address)))
        .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
        .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
          StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
          address,
        )))
        .finish_output()?,
    ];

    let block = client
      .block()
      .with_secret_manager(&secret_manager)
      .with_outputs(outputs)?
      .finish()
      .await?;

    println!(
      "Transaction with new alias output sent: http://localhost:14265/api/v2/blocks/{}",
      block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // create second transaction with the actual AliasId (BLAKE2b-160 hash of the Output ID that created the alias)
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap());
    let alias_id = AliasId::from(alias_output_id);
    let outputs = vec![
      AliasOutputBuilder::new_with_amount(1_000_000, alias_id)?
        .with_state_index(1)
        .with_foundry_counter(0)
        .add_feature(Feature::Sender(SenderFeature::new(address)))
        .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
        .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
          StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
          address,
        )))
        .finish_output()?,
    ];

    let block = client
      .block()
      .with_secret_manager(&secret_manager)
      .with_input(alias_output_id.into())?
      .with_outputs(outputs)?
      .finish()
      .await?;
    println!(
      "Transaction with alias id set sent: http://localhost:14265/api/v2/blocks/{}",
      block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
  }

  // helper function to get the output id for the first alias output
  fn get_alias_output_id(payload: &Payload) -> OutputId {
    match payload {
      Payload::Transaction(tx_payload) => {
        let TransactionEssence::Regular(regular) = tx_payload.essence();
        for (index, output) in regular.outputs().iter().enumerate() {
          if let Output::Alias(_alias_output) = output {
            return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
          }
        }
        panic!("No alias output in transaction essence")
      }
      _ => panic!("No tx payload"),
    }
  }
}
