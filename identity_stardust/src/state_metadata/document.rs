// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_did::did::CoreDID;
use identity_did::document::CoreDocument;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Result;
use crate::Error;
use crate::StardustDocument;

use super::StateMetadataEncoding;
use super::StateMetadataVersion;

pub(crate) static PLACEHOLDER_DID: Lazy<CoreDID> = Lazy::new(|| CoreDID::parse("did:0:0").unwrap());

/// Magic bytes used to mark DID documents.
const DID_MARKER: &[u8] = b"DID";

/// The DID document as it is contained in the state metadata of an alias output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub(crate) struct StateMetadataDocument(CoreDocument);

impl StateMetadataDocument {
  /// Transforms the document into a [`StardustDocument`] by replacing all placeholders with `original_did`.
  pub fn into_stardust_document(self, original_did: &CoreDID) -> StardustDocument {
    let core_doc: CoreDocument = self.0;
    let core_document: CoreDocument = core_doc.map(
      // Replace placeholder identifiers.
      |did| {
        if did == PLACEHOLDER_DID.as_ref() {
          original_did.clone()
        } else {
          did
        }
      },
      // Do not modify properties.
      |o| o,
    );
    StardustDocument(core_document)
  }

  /// Pack a [`StateMetadataDocument`] into bytes, suitable for inclusion in
  /// an alias output's state metadata, according to the given `encoding`.
  pub fn pack(self, encoding: StateMetadataEncoding) -> Result<Vec<u8>> {
    let encoded_message_data: Vec<u8> = match encoding {
      StateMetadataEncoding::Json => self.to_json_vec()?,
    };

    // Prepend flags.
    let encoded_message_data_with_flags =
      add_flags_to_message(encoded_message_data, StateMetadataVersion::CURRENT, encoding);
    Ok(encoded_message_data_with_flags)
  }

  /// Unpack bytes into a [`StateMetadataDocument`].
  pub fn unpack(data: &[u8]) -> Result<Self> {
    // Check version.
    let version: StateMetadataVersion = StateMetadataVersion::try_from(*data.get(0).ok_or(
      identity_did::Error::InvalidDocument("expected data to have at least length 1", None),
    )?)?;
    if version != StateMetadataVersion::V1 {
      return Err(Error::InvalidMessageFlags);
    }

    // Check marker.
    let marker: &[u8] = data.get(1..4).ok_or(identity_did::Error::InvalidDocument(
      "expected data to have at least length 4",
      None,
    ))?;
    if marker != DID_MARKER {
      return Err(Error::InvalidMessageFlags);
    }

    // Decode data.
    let encoding: StateMetadataEncoding = StateMetadataEncoding::try_from(*data.get(4).ok_or(
      identity_did::Error::InvalidDocument("expected data to have at least length 5", None),
    )?)?;

    let inner: &[u8] = data.get(5..).ok_or(identity_did::Error::InvalidDocument(
      "expected data to have at least length 6",
      None,
    ))?;

    match encoding {
      StateMetadataEncoding::Json => StateMetadataDocument::from_json_slice(inner).map_err(Into::into),
    }
  }
}

/// Prepends the message flags and marker magic bytes to the data in the following order:
/// `[version, marker, encoding, data]`.
fn add_flags_to_message(mut data: Vec<u8>, version: StateMetadataVersion, encoding: StateMetadataEncoding) -> Vec<u8> {
  let mut buffer: Vec<u8> = Vec::with_capacity(1 + DID_MARKER.len() + 1 + data.len());
  buffer.push(version as u8);
  buffer.extend_from_slice(DID_MARKER);
  buffer.push(encoding as u8);
  buffer.append(&mut data);
  buffer
}

impl From<StardustDocument> for StateMetadataDocument {
  /// Transforms a [`StardustDocument`] into its state metadata representation by replacing all
  /// occurrences of its did with a placeholder.
  fn from(document: StardustDocument) -> Self {
    let core_document: CoreDocument = CoreDocument::from(document);
    let id: CoreDID = core_document.id().clone();
    let core_doc: CoreDocument = core_document.map(
      // Replace self-referential identifiers with a placeholder, but not others.
      |did| {
        if did == id {
          PLACEHOLDER_DID.clone()
        } else {
          did
        }
      },
      // Do not modify properties.
      |o| o,
    );
    StateMetadataDocument(core_doc)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::OneOrSet;
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::CoreDID;
  use identity_did::verification::MethodScope;

  use crate::state_metadata::PLACEHOLDER_DID;
  use crate::StardustDocument;
  use crate::StateMetadataDocument;
  use crate::StateMetadataEncoding;

  struct TestSetup {
    document: StardustDocument,
    original_did: CoreDID,
    other_did: CoreDID,
  }

  fn test_document() -> TestSetup {
    let original_did =
      CoreDID::parse("did:stardust:8036235b6b5939435a45d68bcea7890eef399209a669c8c263fac7f5089b2ec6").unwrap();
    let other_did =
      CoreDID::parse("did:stardust:71b709dff439f1ac9dd2b9c2e28db0807156b378e13bfa3605ce665aa0d0fdca").unwrap();

    let mut document: StardustDocument = StardustDocument::new();
    document.tmp_set_id(original_did.clone());

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    document
      .tmp_add_verification_method(
        document.tmp_id().clone(),
        &keypair,
        "#did-self",
        MethodScope::VerificationMethod,
      )
      .unwrap();

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    document
      .tmp_add_verification_method(
        other_did.clone(),
        &keypair,
        "#did-foreign",
        MethodScope::authentication(),
      )
      .unwrap();

    document
      .tmp_add_service(
        document.tmp_id().clone(),
        "#my-service",
        "RevocationList2030",
        identity_did::service::ServiceEndpoint::One(Url::parse("https://example.com/xyzabc").unwrap()),
      )
      .unwrap();

    document
      .tmp_add_service(
        other_did.clone(),
        "#my-foreign-service",
        "RevocationList2030",
        identity_did::service::ServiceEndpoint::One(Url::parse("https://example.com/0xf4c42e9da").unwrap()),
      )
      .unwrap();

    document
      .0
      .also_known_as_mut()
      .append(Url::parse("did:example:abc").unwrap());

    document
      .0
      .also_known_as_mut()
      .append(Url::parse("did:example:xyz").unwrap());

    let mut controllers = OneOrSet::new_one(other_did.clone());
    (&mut controllers).append(original_did.clone());
    let mut controllers = Some(controllers);
    std::mem::swap(&mut controllers, document.0.controller_mut());

    TestSetup {
      document,
      original_did,
      other_did,
    }
  }

  #[test]
  fn test_transformation_roundtrip() {
    let TestSetup {
      document,
      original_did,
      other_did,
    } = test_document();

    let state_metadata_doc: StateMetadataDocument = StateMetadataDocument::from(document.clone());

    assert_eq!(
      state_metadata_doc
        .0
        .resolve_method("#did-self", None)
        .unwrap()
        .id()
        .did(),
      PLACEHOLDER_DID.as_ref()
    );

    assert_eq!(
      state_metadata_doc
        .0
        .resolve_method("#did-foreign", None)
        .unwrap()
        .id()
        .did(),
      &other_did
    );

    assert_eq!(
      state_metadata_doc
        .0
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-foreign-service")
        .unwrap()
        .id()
        .did(),
      &other_did
    );

    assert_eq!(
      state_metadata_doc
        .0
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-service")
        .unwrap()
        .id()
        .did(),
      PLACEHOLDER_DID.as_ref()
    );

    let controllers = state_metadata_doc.0.controller().unwrap();
    assert_eq!(controllers.get(0).unwrap(), &other_did);
    assert_eq!(controllers.get(1).unwrap(), PLACEHOLDER_DID.as_ref());

    let stardust_document = state_metadata_doc.into_stardust_document(&original_did);

    assert_eq!(stardust_document, document);
  }

  #[test]
  fn test_packing_roundtrip() {
    let TestSetup { document, .. } = test_document();

    let state_metadata_doc: StateMetadataDocument = StateMetadataDocument::from(document);

    let packed_bytes: Vec<u8> = state_metadata_doc.clone().pack(StateMetadataEncoding::Json).unwrap();

    let unpacked_doc = StateMetadataDocument::unpack(&packed_bytes).unwrap();

    assert_eq!(state_metadata_doc, unpacked_doc);
  }
}
