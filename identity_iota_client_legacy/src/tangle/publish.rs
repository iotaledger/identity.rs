// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core_legacy::document::IotaDocument;
use identity_iota_core_legacy::document::IotaVerificationMethod;

/// Determines whether an updated document needs to be published as an integration or diff message.
#[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
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

    let old_capability_invocation_set: Vec<Option<&IotaVerificationMethod>> = old_doc.extract_signing_keys();
    let new_capability_invocation_set: Vec<Option<&IotaVerificationMethod>> = new_doc.extract_signing_keys();

    if old_capability_invocation_set != new_capability_invocation_set {
      Some(PublishType::Integration)
    } else {
      Some(PublishType::Diff)
    }
  }
}

#[cfg(test)]
mod test {
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::DID;
  use identity_did::verification::MethodScope;
  use identity_iota_core_legacy::did::IotaDIDUrl;
  use identity_iota_core_legacy::document::IotaVerificationMethod;

  use crate::Result;

  use super::*;

  // Returns a document with an embedded capability invocation method, and a generic verification method,
  // that also has as an attached capability invocation verification relationship.
  fn document() -> IotaDocument {
    let initial_keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let mut old_doc: IotaDocument = IotaDocument::new_with_options(&initial_keypair, None, Some("embedded")).unwrap();

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let method2: IotaVerificationMethod =
      IotaVerificationMethod::new(old_doc.id().to_owned(), keypair.type_(), keypair.public(), "generic").unwrap();

    let method2_url: IotaDIDUrl = method2.id().clone();
    old_doc.insert_method(method2, MethodScope::VerificationMethod).unwrap();
    old_doc
      .attach_method_relationship(
        &method2_url,
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

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let method2: IotaVerificationMethod =
      IotaVerificationMethod::new(old_doc.id().to_owned(), keypair.type_(), keypair.public(), "test-2")?;

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

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::new(new_doc.id().to_owned(), keypair.type_(), keypair.public(), "embedded")?;

    new_doc
      .remove_method(&new_doc.id().to_url().join("#embedded").unwrap())
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

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let method_updated: IotaVerificationMethod =
      IotaVerificationMethod::new(new_doc.id().to_owned(), keypair.type_(), keypair.public(), "generic")?;

    assert!(new_doc
      .core_document_mut()
      .verification_method_mut()
      .update(method_updated));

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

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519)?;
    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::new(new_doc.id().to_owned(), keypair.type_(), keypair.public(), "test-2")?;

    new_doc
      .insert_method(verif_method2, MethodScope::authentication())
      .unwrap();

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }

  #[test]
  fn test_publish_type_add_non_capability_invocation_relationship() -> Result<()> {
    let old_doc: IotaDocument = document();
    let mut new_doc: IotaDocument = old_doc.clone();
    let method_url: IotaDIDUrl = new_doc.resolve_method("generic", None).unwrap().id().clone();

    new_doc
      .attach_method_relationship(
        &method_url,
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

    let keypair: KeyPair = KeyPair::new(KeyType::X25519)?;
    let method: IotaVerificationMethod =
      IotaVerificationMethod::new(new_doc.id().to_owned(), keypair.type_(), keypair.public(), "kex-0")?;

    new_doc.insert_method(method, MethodScope::authentication()).unwrap();

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }

  #[test]
  fn test_publish_type_update_method_with_non_update_method_type2() -> Result<()> {
    let mut old_doc = document();

    let keypair: KeyPair = KeyPair::new(KeyType::X25519)?;
    let method: IotaVerificationMethod =
      IotaVerificationMethod::new(old_doc.id().to_owned(), keypair.type_(), keypair.public(), "kex-0")?;

    old_doc
      .insert_method(method, MethodScope::capability_invocation())
      .unwrap();

    let mut new_doc = old_doc.clone();

    // Replace the key material in the new method.
    let keypair_new: KeyPair = KeyPair::new(KeyType::X25519)?;

    let method_new: IotaVerificationMethod = IotaVerificationMethod::new(
      new_doc.id().to_owned(),
      keypair_new.type_(),
      keypair_new.public(),
      "kex-0",
    )?;

    assert!(new_doc
      .core_document_mut()
      .capability_invocation_mut_unchecked()
      .update(method_new.into()));

    assert!(matches!(PublishType::new(&old_doc, &new_doc), Some(PublishType::Diff)));

    Ok(())
  }
}
