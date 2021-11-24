// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::verification::MethodRef;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;

use crate::did::IotaDocument;

// Method types allowed to sign a DID document update.
pub const UPDATE_METHOD_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];

/// Determines whether an updated document needs to be published as an integration or diff message.
#[derive(Clone, Copy, Debug)]
pub enum PublishType {
  Integration,
  Diff,
}

impl PublishType {
  /// Compares two versions of a document and returns whether it needs to be published
  /// as an integration or diff message. If `None` is returned, no update is required.
  ///
  /// Note: A newly created document must always be published as an integration message, and
  /// this method does not handle this case.
  pub fn new(old_doc: &IotaDocument, new_doc: &IotaDocument) -> Option<PublishType> {
    if old_doc == new_doc {
      return None;
    }

    let old_capability_invocation_set: Vec<Option<&VerificationMethod>> = Self::extract_signing_keys(old_doc);
    let new_capability_invocation_set: Vec<Option<&VerificationMethod>> = Self::extract_signing_keys(new_doc);

    if old_capability_invocation_set != new_capability_invocation_set {
      Some(PublishType::Integration)
    } else {
      Some(PublishType::Diff)
    }
  }

  fn extract_signing_keys(document: &IotaDocument) -> Vec<Option<&VerificationMethod>> {
    document
      .as_document()
      .capability_invocation()
      .iter()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(method) => Some(method),
        MethodRef::Refer(did_url) => document.as_document().resolve_method(did_url),
      })
      .filter(|method| {
        if let Some(method) = method {
          UPDATE_METHOD_TYPES.contains(&method.key_type())
        } else {
          true
        }
      })
      .collect()
  }
}

#[cfg(test)]
mod test {
  use identity_core::crypto::merkle_key::Sha256;
  use identity_core::crypto::KeyCollection;
  use identity_core::crypto::KeyPair;
  use identity_did::did::DID;
  use identity_did::verification::MethodScope;

  use crate::did::IotaVerificationMethod;
  use crate::tangle::TangleRef;
  use crate::Result;

  use super::*;

  // Returns a document with an embedded capability invocation method, and a generic verification method,
  // that also has as an attached capability invocation verification relationship.
  fn document() -> IotaDocument {
    let initial_keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let method: IotaVerificationMethod = IotaVerificationMethod::from_keypair(&initial_keypair, "embedded").unwrap();

    let mut old_doc: IotaDocument = IotaDocument::from_verification_method(method).unwrap();

    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(old_doc.did().to_owned(), keypair.type_(), keypair.public(), "generic").unwrap();

    let method3_url = method2.id();

    old_doc.insert_method(method2, MethodScope::VerificationMethod).unwrap();
    old_doc
      .attach_method_relationship(
        method3_url,
        identity_did::verification::MethodRelationship::CapabilityInvocation,
      )
      .unwrap();

    old_doc
  }

  #[test]
  fn test_publish_type_insert_new_embedded_capability_invocation_method() -> Result<()> {
    let old_doc = document();

    assert!(matches!(PublishType::new(&old_doc, &old_doc), None));

    let mut new_doc = old_doc.clone();

    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(old_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-2")?;

    new_doc
      .insert_method(method2, MethodScope::capability_invocation())
      .unwrap();

    assert!(matches!(
      PublishType::new(&old_doc, &new_doc),
      Some(PublishType::Integration)
    ));

    Ok(())
  }

  #[test]
  fn test_publish_type_update_key_material_of_existing_embedded_method() -> Result<()> {
    let old_doc = document();

    let mut new_doc = old_doc.clone();

    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "embedded")?;

    new_doc
      .remove_method(new_doc.did().to_url().join("#embedded").unwrap())
      .unwrap();
    new_doc
      .insert_method(verif_method2, MethodScope::capability_invocation())
      .unwrap();

    assert!(matches!(
      PublishType::new(&old_doc, &new_doc),
      Some(PublishType::Integration)
    ));

    Ok(())
  }

  #[test]
  fn test_publish_type_update_key_material_of_existing_generic_method() -> Result<()> {
    let old_doc = document();

    let mut new_doc = old_doc.clone();

    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let method_updated: IotaVerificationMethod =
      IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "generic")?;

    assert!(unsafe {
      new_doc
        .as_document_mut()
        .verification_method_mut()
        .update(method_updated.into())
    });

    assert!(matches!(
      PublishType::new(&old_doc, &new_doc),
      Some(PublishType::Integration)
    ));

    Ok(())
  }

  #[test]
  fn test_publish_type_add_non_capability_invocation_method() -> Result<()> {
    let old_doc = document();

    let mut new_doc = old_doc.clone();

    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-2")?;

    new_doc
      .insert_method(verif_method2, MethodScope::authentication())
      .unwrap();

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }

  #[test]
  fn test_publish_type_add_non_capability_invocation_relationship() -> Result<()> {
    let old_doc = document();

    let mut new_doc = old_doc.clone();

    let method_url = new_doc.resolve_method("generic").unwrap().id();

    new_doc
      .attach_method_relationship(
        method_url,
        identity_did::verification::MethodRelationship::AssertionMethod,
      )
      .unwrap();

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }

  #[test]
  fn test_publish_type_update_method_with_non_update_method_type() -> Result<()> {
    let old_doc = document();

    let mut new_doc = old_doc.clone();

    let collection = KeyCollection::new_ed25519(8)?;
    let method: IotaVerificationMethod =
      IotaVerificationMethod::create_merkle_key::<Sha256>(new_doc.did().to_owned(), &collection, "merkle")?;

    new_doc.insert_method(method, MethodScope::authentication()).unwrap();

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }

  #[test]
  fn test_publish_type_update_method_with_non_update_method_type2() -> Result<()> {
    let mut old_doc = document();

    let collection = KeyCollection::new_ed25519(8)?;
    let method: IotaVerificationMethod =
      IotaVerificationMethod::create_merkle_key::<Sha256>(old_doc.did().to_owned(), &collection, "merkle")?;

    old_doc
      .insert_method(method, MethodScope::capability_invocation())
      .unwrap();

    let mut new_doc = old_doc.clone();

    // Replace the key collection.
    let new_collection = KeyCollection::new_ed25519(8)?;

    let method_new: IotaVerificationMethod =
      IotaVerificationMethod::create_merkle_key::<Sha256>(new_doc.did().to_owned(), &new_collection, "merkle")?;

    assert!(unsafe {
      new_doc
        .as_document_mut()
        .capability_invocation_mut()
        .update(method_new.into())
    });

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }
}
