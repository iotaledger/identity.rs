// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::IotaDocument;

/// Determines whether an updated document needs to be published as an integration or diff message.
#[derive(Clone, Copy, Debug)]
pub enum Publish {
  None,
  Integration,
  Diff,
}

impl Publish {
  /// Compares two versions of a document and returns whether it needs to be published
  /// as an integration or diff message.
  ///
  /// Note: A newly created document must always be published as an integration message, and
  /// this function does not handle this case.
  pub fn new(old_doc: &IotaDocument, new_doc: &IotaDocument) -> Publish {
    if old_doc == new_doc {
      Publish::None
    } else if old_doc.as_document().capability_invocation() != new_doc.as_document().capability_invocation() {
      Publish::Integration
    } else {
      Publish::Diff
    }
  }
}

#[cfg(test)]
mod test {
  use identity_core::crypto::KeyPair;
  use identity_did::did::DID;
  use identity_did::verification::MethodScope;

  use crate::did::IotaVerificationMethod;
  use crate::tangle::TangleRef;
  use crate::Result;

  use super::*;

  #[test]
  fn test_publish_type() -> Result<()> {
    let initial_keypair: KeyPair = KeyPair::new_ed25519()?;
    let method: IotaVerificationMethod = IotaVerificationMethod::from_keypair(&initial_keypair, "test-0")?;

    let old_doc: IotaDocument = IotaDocument::from_verification_method(method)?;

    assert!(matches!(Publish::new(&old_doc, &old_doc), Publish::None));

    // Inserting a new capability invocation method results in an integration update.
    let mut new_doc = old_doc.clone();

    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(old_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-1")?;

    new_doc.insert_method(verif_method2, MethodScope::CapabilityInvocation);

    assert!(matches!(Publish::new(&old_doc, &new_doc), Publish::Integration));

    // Updating the key material of the existing verification method results in an integration update.
    let mut new_doc = old_doc.clone();

    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-0")?;

    new_doc.remove_method(new_doc.did().to_url().join("#test-0").unwrap());
    new_doc.insert_method(verif_method2, MethodScope::CapabilityInvocation);

    assert!(matches!(Publish::new(&old_doc, &new_doc), Publish::Integration));

    // Adding methods with relationships other than capability invocation
    // results in a diff update.
    let mut new_doc = old_doc.clone();

    let verif_method2: IotaVerificationMethod =
      IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-1")?;

    new_doc.insert_method(verif_method2, MethodScope::Authentication);

    assert!(matches!(Publish::new(&old_doc, &new_doc), Publish::Diff));

    Ok(())
  }
}
