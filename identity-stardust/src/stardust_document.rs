// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::str::FromStr;
use identity_core::common::Object;

use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::{FmtJson, FromJson};
use identity_core::utils::{Base, BaseEncoding};
use identity_did::did::{CoreDID, DID};
use identity_did::document::CoreDocument;
use iota_client::bee_block::Block;
use iota_client::bee_block::output::{AliasId, Output, OutputId};
use iota_client::bee_block::output::feature::MetadataFeature;
use iota_client::bee_block::payload::Payload;
use iota_client::bee_block::payload::transaction::TransactionEssence;
use lazy_static::lazy_static;

use crate::error::Result;

/// An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
/// merged with one or more diff messages.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StardustDocument(CoreDocument<CoreDID>);

// Tag is 64-bytes long now, matching the hex-encoding of the Alias ID (without 0x prefix).
lazy_static! {
  static ref PLACEHOLDER_DID: CoreDID = {
    CoreDID::parse("did:stardust:0000000000000000000000000000000000000000000000000000000000000000").unwrap()
  };
}

impl StardustDocument {
  /// Constructs an empty DID Document with a [`placeholder_did`] identifier.
  pub fn new() -> StardustDocument {
    Self(
      // PANIC: constructing an empty DID Document is infallible, caught by tests otherwise.
      CoreDocument::builder(Object::default())
        .id(Self::placeholder_did().clone())
        .build()
        .expect("empty StardustDocument constructor failed")
    )
  }

  /// Returns the placeholder DID of newly constructed DID Documents,
  /// `"did:stardust:0000000000000000000000000000000000000000000000000000000000000000"`.
  // TODO: generalise to take network name?
  pub fn placeholder_did() -> &'static CoreDID {
    &PLACEHOLDER_DID
  }

  /// Constructs a DID from an Alias ID.
  ///
  /// Uses the hex-encoding of the Alias ID as the DID tag.
  pub fn alias_id_to_did(id: &AliasId) -> Result<CoreDID> {
    // TODO: encode manually to avoid "0x" hex prefix?
    CoreDID::parse(format!("did:stardust:{id}")).map_err(Into::into)
  }

  pub fn did_to_alias_id(did: &CoreDID) -> Result<AliasId> {
    // TODO: just use 0x in the tag as well?
    // Prepend 0x manually.
    AliasId::from_str(&format!("0x{}", did.method_id())).map_err(Into::into)
  }

  // TODO: can hopefully remove if the publishing logic is wrapped.
  pub fn did_from_block(block: &Block) -> Result<CoreDID> {
    let id: AliasId = AliasId::from(get_alias_output_id(block.payload().unwrap()));

    // Manually encode to hex to avoid 0x prefix.
    let hex: String = BaseEncoding::encode(id.as_slice(), Base::Base16Lower);
    CoreDID::parse(format!("did:stardust:{hex}")).map_err(Into::into)
  }

  fn parse_block(block: &Block) -> (AliasId, &MetadataFeature, bool) {
    match block.payload().unwrap() {
      Payload::Transaction(tx_payload) => {
        let TransactionEssence::Regular(regular) = tx_payload.essence();
        for (index, output) in regular.outputs().iter().enumerate() {
          if let Output::Alias(alias_output) = output {
            let metadata = alias_output.features().metadata().expect("no metadata");
            let (alias_id, first) = if alias_output.alias_id().is_null() {
              // First Alias Output, compute ID.
              (AliasId::from(OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap()), true)
            } else {
              (alias_output.alias_id().clone(), false)
            };
            return (alias_id, metadata, first);
          }
        }
        panic!("No alias output in transaction essence")
      }
      _ => panic!("No tx payload"),
    };
  }

  /// Deserializes a JSON-encoded `StardustDocument` from an Alias Output block.
  pub fn deserialize_from_block(block: &Block) -> Result<StardustDocument> {
    let (alias_id, metadata, first) = Self::parse_block(&block);

    let did: CoreDID = Self::alias_id_to_did(&alias_id)?;
    let json_slice: &[u8] = metadata.data();

    // Replace the placeholder DID in the Document content for the first Alias Output block.
    if first {
      let json = String::from_utf8(json_slice.to_vec()).unwrap();
      let replaced = json.replace(Self::placeholder_did().as_str(), did.as_str());
      StardustDocument::from_json(&replaced).map_err(Into::into)
    } else {
      StardustDocument::from_json_slice(json_slice).map_err(Into::into)
    }
  }
}


impl Display for StardustDocument {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt_json(f)
  }
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

#[cfg(test)]
mod tests {
  use identity_core::crypto::KeyType;
  use super::*;

  #[test]
  fn test_new() {
    let document: StardustDocument = StardustDocument::new();
    assert_eq!(document.0.id(), StardustDocument::placeholder_did());
  }
}
