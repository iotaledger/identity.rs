// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_did::did::DID;
use identity_did::verification::MethodScope;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;
use identity_iota::did::IotaVerificationMethod;
use identity_iota::tangle::TangleRef;

use crate::account::Account;
use crate::account::AccountBuilder;
use crate::account::Publish;
use crate::error::Result;
use crate::identity::IdentitySetup;

#[tokio::test]
async fn test_account_high_level() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let account1: Account = builder.create_identity(IdentitySetup::default()).await?;

  builder = builder.autopublish(false);

  let account2: Account = builder.create_identity(IdentitySetup::default()).await?;

  assert!(account1.autopublish());
  assert!(!account2.autopublish());

  let did1 = account1.did().to_owned();
  let did2 = account2.did().to_owned();
  account2.delete_identity().await?;

  assert!(matches!(
    builder.load_identity(did2).await.unwrap_err(),
    crate::Error::IdentityNotFound
  ));

  // Relase the lease on did1.
  std::mem::drop(account1);

  assert!(builder.load_identity(did1).await.is_ok());

  Ok(())
}

#[tokio::test]
async fn test_account_did_lease() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let did: IotaDID = {
    let account: Account = builder.create_identity(IdentitySetup::default()).await?;
    account.did().to_owned()
  }; // <-- Lease released here.

  // Lease is not in-use
  let _account = builder.load_identity(did.clone()).await.unwrap();

  // Lease is in-use
  assert!(matches!(
    builder.load_identity(did).await.unwrap_err(),
    crate::Error::IdentityInUse
  ));

  Ok(())
}

fn create_document() -> IotaDocument {
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
  let verif_method1: IotaVerificationMethod = IotaVerificationMethod::from_keypair(&keypair, "test-0").unwrap();

  IotaDocument::from_authentication(verif_method1).unwrap()
}

#[test]
fn test_publish_type() -> Result<()> {
  let old_doc = create_document();

  assert!(matches!(Publish::new(&old_doc, &old_doc), Publish::None));

  // Inserting a new capability invocation method results in an integration update.
  let mut new_doc = old_doc.clone();

  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let verif_method2: IotaVerificationMethod =
    IotaVerificationMethod::from_did(old_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-1")?;

  new_doc.insert_method(MethodScope::CapabilityInvocation, verif_method2);

  assert!(matches!(Publish::new(&old_doc, &new_doc), Publish::Integration));

  // Updating the key material of the existing verification method results in an integration update.
  let mut new_doc = old_doc.clone();

  let verif_method2: IotaVerificationMethod =
    IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-0")?;

  new_doc
    .remove_method(new_doc.did().to_url().join("test-0").unwrap())
    .unwrap();
  new_doc.insert_method(MethodScope::CapabilityInvocation, verif_method2);

  assert!(matches!(Publish::new(&old_doc, &new_doc), Publish::Integration));

  // Adding methods with relationships other than capability invocation
  // results in a diff update.
  let mut new_doc = old_doc.clone();

  let verif_method2: IotaVerificationMethod =
    IotaVerificationMethod::from_did(new_doc.did().to_owned(), keypair.type_(), keypair.public(), "test-1")?;

  new_doc.insert_method(MethodScope::Authentication, verif_method2);

  assert!(matches!(Publish::new(&old_doc, &new_doc), Publish::Diff));

  Ok(())
}
