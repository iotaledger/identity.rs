// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::JcsEd25519Signature2020;
use identity_core::crypto::SignatureData;
use identity_core::crypto::SignatureOptions;
use identity_core::utils::decode_b58;
use identity_did::document;
use identity_did::verifiable::LdSuite;
use identity_did::verifiable::Properties;

struct TestVector {
  document_unsigned: &'static str,
  document_signed: &'static str,
  secret_b58: &'static str,
  signature: &'static str,
}

const TVS: &[TestVector] = &include!("fixtures/jcs_ed25519.rs");

type Document = document::Document<Properties<Object>, (), ()>;

#[test]
fn test_jcs_ed25519_can_sign_and_verify() {
  for tv in TVS {
    let secret = decode_b58(tv.secret_b58).unwrap();
    let mut unsigned: Document = Document::from_json(tv.document_unsigned).unwrap();
    let signed: Document = Document::from_json(tv.document_signed).unwrap();
    let method = unsigned.try_resolve("#key-1").unwrap();
    let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());
    let suite: LdSuite<_> = LdSuite::new(JcsEd25519Signature2020);

    suite.sign(&mut unsigned, options, &secret).unwrap();

    assert!(suite.verify(&unsigned).is_ok());
    assert_eq!(unsigned.proof().unwrap().data().as_str(), tv.signature);
    assert_eq!(unsigned.to_jcs().unwrap(), signed.to_jcs().unwrap());
  }
}

#[test]
fn test_jcs_ed25519_fails_when_key_is_mutated() {
  for tv in TVS {
    let secret = decode_b58(tv.secret_b58).unwrap();
    let mut document: Document = Document::from_json(tv.document_unsigned).unwrap();
    let method = document.try_resolve("#key-1").unwrap();
    let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());
    let suite: LdSuite<_> = LdSuite::new(JcsEd25519Signature2020);

    suite.sign(&mut document, options, &secret).unwrap();

    assert!(suite.verify(&document).is_ok());
    assert_eq!(document.proof().unwrap().data().as_str(), tv.signature);

    document.proof_mut().unwrap().verification_method = "#key-2".into();

    assert!(suite.verify(&document).is_err());
  }
}

#[test]
fn test_jcs_ed25519_fails_when_signature_is_mutated() {
  for tv in TVS {
    let secret = decode_b58(tv.secret_b58).unwrap();
    let mut document: Document = Document::from_json(tv.document_unsigned).unwrap();
    let method = document.try_resolve("#key-1").unwrap();
    let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());
    let suite: LdSuite<_> = LdSuite::new(JcsEd25519Signature2020);

    suite.sign(&mut document, options, &secret).unwrap();

    assert!(suite.verify(&document).is_ok());
    assert_eq!(document.proof().unwrap().data().as_str(), tv.signature);

    document
      .proof_mut()
      .unwrap()
      .set_data(SignatureData::Signature("foo".into()));

    assert!(suite.verify(&document).is_err());
  }
}
