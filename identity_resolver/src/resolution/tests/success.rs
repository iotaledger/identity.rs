// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::ProofOptions;
use identity_core::utils::BaseEncoding;
use identity_credential::presentation::Presentation;
use identity_credential::validator::CredentialValidator;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::SubjectHolderRelationship;
use identity_credential::validator::ValidatorDocument;
use identity_did::did::CoreDID;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::document::Document;
use identity_did::verifiable::VerifierOptions;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use serde::de::DeserializeOwned;

use crate::Resolver;

// VALID Setup: The private key of issuer did:iota:.., holder did:foo, issuer did:bar corresponds to the (hex decoding) of SECRET KEY in TEST 1, 2 and 3 of https://datatracker.ietf.org/doc/html/rfc8032#section-7 respectively.

const HOLDER_PRIVATE_KEY: [u8; 32] = [
  76, 205, 8, 155, 40, 255, 150, 218, 157, 182, 195, 70, 236, 17, 78, 15, 91, 138, 49, 159, 53, 171, 166, 36, 218, 140,
  246, 237, 79, 184, 166, 251,
];

// Not used, but can be useful for maintenance purposes 
const _ISSUER_IOTA_PRIVATE_KEY: [u8; 32] = [157, 97, 177, 157, 239, 253, 90, 96, 186, 132, 74, 244, 146, 236, 44, 196, 68, 73, 197, 105, 123, 50, 105, 25, 112, 59, 172, 3, 28, 174, 127, 96]; 

const _ISSUER_BAR_PRIVATE_KEY: [u8; 32] = [197, 170, 141, 244, 63, 159, 131, 123, 237, 183, 68, 47, 49, 220, 183, 177, 102, 211, 133, 53, 7, 111, 9, 75, 133, 206, 58, 46, 11, 68, 88, 247]; 


const HOLDER_DOC_JSON: &str = r#"{
    "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
    "verificationMethod": [
      {
        "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5#root",
        "controller": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "z586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5"
      }
    ]
  }"#;

const ISSUER_IOTA_DOC: &str = r#"{
    "doc": {
      "id": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
      "verificationMethod": [
        {
          "id": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA#issuerKey",
          "controller": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
        }
      ]
    },
    "meta": {
      "created": "2022-08-31T09:33:31Z",
      "updated": "2022-08-31T09:33:31Z"
    }
  }"#;

const ISSUER_BAR_DOC: &str = r#"{
    "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
    "verificationMethod": [
      {
        "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
        "controller": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "zHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr"
      }
    ]
  }"#;

const PRESENTATION_JSON: &str = r#"{
    "@context": "https://www.w3.org/2018/credentials/v1",
    "id": "https://example.org/credentials/3732",
    "type": "VerifiablePresentation",
    "verifiableCredential": [
      {
        "@context": "https://www.w3.org/2018/credentials/v1",
        "id": "https://example.edu/credentials/3732",
        "type": [
          "VerifiableCredential",
          "UniversityDegreeCredential"
        ],
        "credentialSubject": {
          "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
          "GPA": "4.0",
          "degree": {
            "name": "Bachelor of Science and Arts",
            "type": "BachelorDegree"
          },
          "name": "Alice"
        },
        "issuer": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "issuanceDate": "2022-08-31T08:35:44Z",
        "expirationDate": "2050-09-01T08:35:44Z",
        "proof": {
          "type": "JcsEd25519Signature2020",
          "verificationMethod": "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA#issuerKey",
          "signatureValue": "3d2aAPqjzaSQ2XbFtqLsauv2Ukdn4Hcevz2grNuJn4q4JbBmDHZpAvekVG12A3ZKRRTeKaBPguxXqcDaqujckWWz"
        }
      },
      {
        "@context": "https://www.w3.org/2018/credentials/v1",
        "id": "https://example.edu/credentials/3732",
        "type": [
          "VerifiableCredential",
          "UniversityDegreeCredential"
        ],
        "credentialSubject": {
          "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
          "GPA": "4.0",
          "degree": {
            "name": "Bachelor of Science and Arts",
            "type": "BachelorDegree"
          },
          "name": "Alice"
        },
        "issuer": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
        "issuanceDate": "2022-08-31T08:35:44Z",
        "expirationDate": "2050-09-01T08:35:44Z",
        "proof": {
          "type": "JcsEd25519Signature2020",
          "verificationMethod": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
          "signatureValue": "2iAYujqHLXP5csZzabdkfurpHaKT3Q8dnJDA4TL7pSJ7gjXLCb2tN7CF4ztKkCKmvY6VYG3pTuN1PeLGEFiQvuQr"
        }
      }      
    ],
    "holder": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5"
  }"#;

// Setup mock handlers:
#[derive(Debug, thiserror::Error)]
#[error("the document could not be resolved")]
struct ResolutionError;
// returns the deserialization of JSON (if the did matches) otherwise an error.
async fn resolve<D, DOC>(did: D, json: &str) -> Result<DOC, ResolutionError>
where
  D: DID + Send + Sync + 'static + Eq,
  DOC: Document + Send + Sync + 'static + DeserializeOwned,
  <DOC as Document>::D: PartialEq<D>,
{
  let doc: DOC = DOC::from_json(json).unwrap();
  (doc.id() == &did).then_some(doc).ok_or(ResolutionError)
}

async fn resolve_foo(did: CoreDID) -> Result<CoreDocument, ResolutionError> {
  resolve(did, HOLDER_DOC_JSON).await
}

async fn resolve_iota(did: StardustDID) -> Result<StardustDocument, ResolutionError> {
  resolve(did, ISSUER_IOTA_DOC).await
}

async fn resolve_bar(did: CoreDID) -> Result<CoreDocument, ResolutionError> {
  resolve(did, ISSUER_BAR_DOC).await
}

// Signs the presentation with the holder's did document and a challenge.
// Returns the signed presentation together with the challenge.
fn signed_presentation() -> (Presentation, String) {
  // deserialize the presentation and holder doc and sign the presentation with the holder's private key.
  let mut presentation: Presentation = Presentation::from_json(PRESENTATION_JSON).unwrap();
  let holder: CoreDocument = CoreDocument::from_json(HOLDER_DOC_JSON).unwrap();
  let challenge: String = "475a7984-1bb5-4c4c-a56f-822bccd46441".to_owned();
  holder
    .signer(
      KeyPair::try_from_private_key_bytes(identity_core::crypto::KeyType::Ed25519, &HOLDER_PRIVATE_KEY)
        .unwrap()
        .private(),
    )
    .options(ProofOptions::new().challenge(challenge.clone()))
    .method(holder.methods().next().unwrap().id())
    .sign(&mut presentation)
    .unwrap();
  (presentation, challenge)
}

async fn check_success_for_all_methods<DOC>(mut resolver: Resolver<DOC>)
where
  DOC: ValidatorDocument + From<CoreDocument> + From<StardustDocument> + Send + Sync,
{
  resolver.attach_handler(StardustDID::METHOD.to_owned(), resolve_iota);
  resolver.attach_handler("foo".to_owned(), resolve_foo);
  resolver.attach_handler("bar".to_owned(), resolve_bar);

  let (mut presentation, challenge): (Presentation, String) = signed_presentation();

  let issuer_iota_doc: StardustDocument = resolve_iota("did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".parse().unwrap()).await.unwrap(); 
  let issuer_bar_doc: CoreDocument = resolve_bar("did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr".parse().unwrap()).await.unwrap(); 


  for mut credential in presentation.clone().verifiable_credential.into_iter() {
    credential.expiration_date = Some(Timestamp::parse("2050-09-01T08:35:44Z").unwrap()); 
    let did_str: String = CredentialValidator::extract_issuer::<CoreDID,_>(&credential).map(|value| value.to_string()).unwrap_or("".into()); 
    if did_str == issuer_bar_doc.id().as_str() {
      issuer_bar_doc.signer(
        KeyPair::try_from_private_key_bytes(KeyType::Ed25519, &_ISSUER_BAR_PRIVATE_KEY).unwrap().private()
      )
      .options(ProofOptions::default())
      .method(issuer_bar_doc.methods().next().unwrap().id())
      .sign(&mut credential).unwrap(); 
    } else if did_str == issuer_iota_doc.id().as_str() {
      issuer_iota_doc.signer(
        KeyPair::try_from_private_key_bytes(KeyType::Ed25519, &_ISSUER_IOTA_PRIVATE_KEY).unwrap().private()
      )
      .options(ProofOptions::default())
      .method(issuer_iota_doc.methods().next().unwrap().id())
      .sign(&mut credential).unwrap(); 
    }

    println!("{}", credential.to_json_pretty().unwrap()); 
    }

  

  // resolve the DID documents of the presentation's holder and credential issuers.
  let holder_doc = resolver.resolve_presentation_holder(&presentation).await.unwrap();
  let issuer_docs = resolver.resolve_presentation_issuers(&presentation).await.unwrap();

  // check that verification works regardless of whether we first resolve and then pass holder/issuers to the method or
  // if resolution of missing documents is done internally.
  for pass_holder_as_arg in [true, false] {
    for pass_issuers_as_arg in [true, false] {
      let holder: Option<&DOC> = pass_holder_as_arg.then_some(&holder_doc);
      let issuers: Option<&[DOC]> = pass_issuers_as_arg.then_some(&issuer_docs);
      let validation_result = resolver
        .verify_presentation(
          &presentation,
          &PresentationValidationOptions::new()
            .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
            .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
          FailFast::FirstError,
          holder,
          issuers
        )
        .await; 
        if validation_result.is_err() {
          dbg!(holder); 
          dbg!(issuers);
          dbg!(&validation_result); 
       

          let issuer_iota: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
          let issuer_iota_decoded = BaseEncoding::decode(issuer_iota, identity_core::utils::Base::Base16Lower).unwrap(); 
          println!("issuer_iota_key: {:?}", issuer_iota_decoded); 

          let issuer_bar: &str = "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7";
          let issuer_bar_decoded = BaseEncoding::decode(issuer_bar, identity_core::utils::Base::Base16Lower).unwrap(); 
          println!("issuer_bar_key: {:?}", issuer_bar_decoded);

          panic!();
        }
    }
  }
}

#[tokio::test]
async fn presentation_verification_works() {
  let core_resolver: Resolver<CoreDocument> = Resolver::new();
  let dynamic_resolver: Resolver = Resolver::new();
  check_success_for_all_methods(core_resolver).await;
  check_success_for_all_methods(dynamic_resolver).await;
}
