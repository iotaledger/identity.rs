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
use crate::StardustCoreDocument;
use crate::StardustDID;
use crate::StardustDocument;
use crate::StardustDocumentMetadata;

use super::StateMetadataEncoding;
use super::StateMetadataVersion;

pub(crate) static PLACEHOLDER_DID: Lazy<CoreDID> = Lazy::new(|| CoreDID::parse("did:0:0").unwrap());

/// Magic bytes used to mark DID documents.
const DID_MARKER: &[u8] = b"DID";

/// Intermediate representation of the DID document as it is contained in the state metadata of
/// an Alias Output.
///
/// DID instances in the document are replaced by the `PLACEHOLDER_DID`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub(crate) struct StateMetadataDocument {
  document: CoreDocument,
  metadata: StardustDocumentMetadata,
}

impl StateMetadataDocument {
  /// Transforms the document into a [`StardustDocument`] by replacing all placeholders with `original_did`.
  pub fn into_stardust_document(self, original_did: &StardustDID) -> Result<StardustDocument> {
    let Self { document, metadata } = self;
    let core_document: StardustCoreDocument = document.try_map(
      // Replace placeholder identifiers.
      |did| {
        if did == PLACEHOLDER_DID.as_ref() {
          Ok(original_did.clone())
        } else {
          // TODO: wrap error?
          StardustDID::try_from_core(did)
        }
      },
      // Do not modify properties.
      Ok,
    )?;
    Ok(StardustDocument::from((core_document, metadata)))
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
    // Check marker.
    let marker: &[u8] = data.get(0..3).ok_or(identity_did::Error::InvalidDocument(
      "expected data to have at least length 3",
      None,
    ))?;
    if marker != DID_MARKER {
      return Err(Error::InvalidStateMetadata("missing `DID` marker"));
    }

    // Check version.
    let version: StateMetadataVersion = StateMetadataVersion::try_from(*data.get(3).ok_or(
      identity_did::Error::InvalidDocument("expected data to have at least length 4", None),
    )?)?;
    if version != StateMetadataVersion::V1 {
      return Err(Error::InvalidStateMetadata("unsupported version"));
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
/// `[marker, version, encoding, data]`.
fn add_flags_to_message(mut data: Vec<u8>, version: StateMetadataVersion, encoding: StateMetadataEncoding) -> Vec<u8> {
  let mut buffer: Vec<u8> = Vec::with_capacity(1 + DID_MARKER.len() + 1 + data.len());
  buffer.extend_from_slice(DID_MARKER);
  buffer.push(version as u8);
  buffer.push(encoding as u8);
  buffer.append(&mut data);
  buffer
}

impl From<StardustDocument> for StateMetadataDocument {
  /// Transforms a [`StardustDocument`] into its state metadata representation by replacing all
  /// occurrences of its did with a placeholder.
  fn from(document: StardustDocument) -> Self {
    let StardustDocument { document, metadata } = document;
    let id: StardustDID = document.id().clone();
    let core_document: CoreDocument = document.map(
      // Replace self-referential identifiers with a placeholder, but not others.
      |did| {
        if did == id {
          PLACEHOLDER_DID.clone()
        } else {
          CoreDID::from(did)
        }
      },
      // Do not modify properties.
      |o| o,
    );
    StateMetadataDocument {
      document: core_document,
      metadata,
    }
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::OneOrSet;
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::DID;
  use identity_did::verification::MethodScope;

  use crate::state_metadata::PLACEHOLDER_DID;
  use crate::StardustDID;
  use crate::StardustDocument;
  use crate::StardustService;
  use crate::StardustVerificationMethod;
  use crate::StateMetadataDocument;
  use crate::StateMetadataEncoding;

  struct TestSetup {
    document: StardustDocument,
    did_self: StardustDID,
    did_foreign: StardustDID,
  }

  fn test_document() -> TestSetup {
    let did_self =
      StardustDID::parse("did:stardust:0x8036235b6b5939435a45d68bcea7890eef399209a669c8c263fac7f5089b2ec6").unwrap();
    let did_foreign =
      StardustDID::parse("did:stardust:0x71b709dff439f1ac9dd2b9c2e28db0807156b378e13bfa3605ce665aa0d0fdca").unwrap();

    let mut document: StardustDocument = StardustDocument::new_with_id(did_self.clone());
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    document
      .insert_method(
        StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "did-self").unwrap(),
        MethodScope::VerificationMethod,
      )
      .unwrap();

    let keypair_foreign: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    document
      .insert_method(
        StardustVerificationMethod::new(
          did_foreign.clone(),
          keypair_foreign.type_(),
          keypair_foreign.public(),
          "did-foreign",
        )
        .unwrap(),
        MethodScope::authentication(),
      )
      .unwrap();

    assert!(document.insert_service(
      StardustService::builder(Object::new())
        .id(document.id().to_url().join("#my-service").unwrap())
        .type_("RevocationList2022")
        .service_endpoint(Url::parse("https://example.com/xyzabc").unwrap())
        .build()
        .unwrap()
    ));

    assert!(document.insert_service(
      StardustService::builder(Object::new())
        .id(did_foreign.to_url().join("#my-foreign-service").unwrap())
        .type_("RevocationList2022")
        .service_endpoint(Url::parse("https://example.com/0xf4c42e9da").unwrap())
        .build()
        .unwrap()
    ));

    document
      .also_known_as_mut()
      .append(Url::parse("did:example:abc").unwrap());
    document
      .also_known_as_mut()
      .append(Url::parse("did:example:xyz").unwrap());

    let controllers = OneOrSet::try_from(vec![did_foreign.clone(), did_self.clone()]).unwrap();
    *document.core_document_mut().controller_mut() = Some(controllers);

    TestSetup {
      document,
      did_self,
      did_foreign,
    }
  }

  #[test]
  fn test_transformation_roundtrip() {
    let TestSetup {
      document,
      did_self,
      did_foreign,
    } = test_document();

    let state_metadata_doc: StateMetadataDocument = StateMetadataDocument::from(document.clone());

    assert_eq!(
      state_metadata_doc
        .document
        .resolve_method("#did-self", None)
        .unwrap()
        .id()
        .did(),
      PLACEHOLDER_DID.as_ref()
    );

    assert_eq!(
      state_metadata_doc
        .document
        .resolve_method("#did-foreign", None)
        .unwrap()
        .id()
        .did(),
      did_foreign.as_ref()
    );

    assert_eq!(
      state_metadata_doc
        .document
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-foreign-service")
        .unwrap()
        .id()
        .did(),
      did_foreign.as_ref()
    );

    assert_eq!(
      state_metadata_doc
        .document
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-service")
        .unwrap()
        .id()
        .did(),
      PLACEHOLDER_DID.as_ref()
    );

    let controllers = state_metadata_doc.document.controller().unwrap();
    assert_eq!(controllers.get(0).unwrap(), did_foreign.as_ref());
    assert_eq!(controllers.get(1).unwrap(), PLACEHOLDER_DID.as_ref());

    let stardust_document = state_metadata_doc.into_stardust_document(&did_self).unwrap();
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
