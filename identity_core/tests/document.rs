use identity_core::{
    did::{Authentication, DIDDocument, DIDDocumentBuilder, ServiceBuilder, ServiceEndpoint, DID},
    key::{KeyData, KeyType, PublicKeyBuilder},
};

use std::str::FromStr;

use identity_diff::Diff;

const JSON_STR: &str = include_str!("fixtures/did/document.json");

fn setup_json(key: &str) -> String {
    let json_str = json::parse(JSON_STR).unwrap();

    json_str[key].to_string()
}

/// Test a full Did document from String.
#[test]
fn test_parse_document() {
    let json_str = setup_json("doc");

    let doc = DIDDocument::from_str(&json_str);

    assert!(doc.is_ok());

    let doc = doc.unwrap();
    let ctx = vec![DID::BASE_CONTEXT.into(), DID::SECURITY_CONTEXT.into()].into();

    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456789abcdefghi".into()],
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(doc.context, ctx);
    assert_eq!(doc.id, did);
}

/// test doc creation via the `DIDDocument::new` method.
#[test]
fn test_doc_creation() {
    let mut did_doc = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let endpoint = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    let service = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("EncryptedDataVault")
        .endpoint(endpoint)
        .build()
        .unwrap();

    let endpoint2 = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    let service2 = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("IdentityHub")
        .endpoint(endpoint2)
        .build()
        .unwrap();

    did_doc.update_service(service.clone());
    did_doc.update_service(service2.clone());

    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    did_doc.update_public_key(public_key.clone());

    let mut did_doc_2 = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    did_doc_2.update_public_key(public_key);

    did_doc_2.update_service(service);
    did_doc_2.update_service(service2);

    did_doc.init_timestamps();

    // did_doc has timestamps while did_doc_2 does not.
    assert_ne!(did_doc, did_doc_2);
}

/// test serde_diff on the did document structure.
#[test]
fn test_doc_diff() {
    // old doc
    let old = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    // new doc.
    let mut new = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let endpoint = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    let service = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("EncryptedDataVault")
        .endpoint(endpoint)
        .build()
        .unwrap();

    new.update_service(service);

    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    new.update_public_key(public_key);

    let diff = old.diff(&new).unwrap();

    let old = old.merge(diff).unwrap();

    // check to see that the old and new docs cotain all of the same fields.
    assert_eq!(new.to_string(), old.to_string());
}

// test diffing the timestamps.
#[test]
fn test_doc_diff_timestamps() {
    // fetch the did doc in json format.
    let json_str = setup_json("doc");

    // create a default doc.
    let doc1 = DIDDocument::default();

    // generate a doc from the json string.
    let mut doc2 = DIDDocument::from_str(&json_str).unwrap();

    // call update time on the doc.
    doc2.update_time();

    // diff the doc with doc2.
    let diff = doc1.diff(&doc2).unwrap();

    // merge the doc diff into doc 1.
    let merge = doc1.merge(diff).unwrap();

    // check to see that the new doc is the same as doc2.
    assert_eq!(merge, doc2);
}

#[test]
fn test_diff_merge_from_string() {
    let diff_str = setup_json("diff");

    // create a doc.
    let mut doc = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    // create an endpoint.
    let endpoint = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    // create a IdCompare<Service>
    let service = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("EncryptedDataVault")
        .endpoint(endpoint)
        .build()
        .unwrap();

    // update the service.
    doc.update_service(service);

    // create a public key, IdCompare<PublicKey>.
    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    // add the public key to the did doc.
    doc.update_public_key(public_key);

    // get the diff from the json string and generate a did doc.
    let diff = DIDDocument::get_diff_from_str(diff_str).unwrap();

    // create a default empty did Doc.
    let def_doc = DIDDocument::default();

    // merge the diff with the default doc.
    let res = def_doc.merge(diff).unwrap();

    // check to see if the new doc is the same as the old doc.
    assert_eq!(doc, res);
}

#[test]
fn test_doc_metadata() {
    // get the json string for a did doc.
    let json_str = setup_json("doc");
    // get the json string for the metadata.
    let result_str = setup_json("metadata");

    // generate a did doc from the json string.
    let doc = DIDDocument::from_str(&json_str);

    assert!(doc.is_ok());

    let mut doc = doc.unwrap();
    // insert the metadata.
    doc.set_metadata("some", "metadata");
    doc.set_metadata("some_more", "metadata_stuff");

    // get the metadata doc string and create a new did doc from it.
    let res_doc = DIDDocument::from_str(&result_str).unwrap();

    // check to see if the two docs are the same.
    assert_eq!(doc, res_doc);
}

#[test]
fn test_realistic_diff() {
    let json_str = setup_json("diff2");

    let mut did_doc = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    let endpoint = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    let service = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("EncryptedDataVault")
        .endpoint(endpoint)
        .build()
        .unwrap();

    let endpoint2 = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    let service2 = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("IdentityHub")
        .endpoint(endpoint2)
        .build()
        .unwrap();

    did_doc.update_service(service);
    did_doc.update_service(service2);

    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    did_doc.update_public_key(public_key.clone());

    let key1 = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-2".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(KeyData::PublicKeyPem(
            "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into(),
        ))
        .build()
        .unwrap();

    let mut did_doc_2 = did_doc.clone();

    did_doc_2.update_public_key(public_key);
    did_doc_2.update_public_key(key1);

    let diff = did_doc.diff(&did_doc_2).unwrap();

    let json = serde_json::to_string(&diff).unwrap();

    assert_eq!(json_str, json);

    let diff = DIDDocument::get_diff_from_str(json).unwrap();

    let new_doc = did_doc.merge(diff).unwrap();

    assert_eq!(did_doc_2, new_doc);
}

// test that items in the did doc are unique by their subject/id.
#[test]
fn test_id_compare() {
    // service endpoint.
    let endpoint = ServiceEndpoint::Url("https://edv.example.com/".parse().unwrap());

    // create a public key.
    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghi#keys-1".parse().unwrap())
        .key_type(KeyType::Ed25519VerificationKey2018)
        .controller("did:iota:123456789abcdefghi".parse().unwrap())
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    // create the authentication key.
    let auth = Authentication::Key(public_key.clone());

    // create a IdCompare<Service>
    let service = ServiceBuilder::default()
        .id("did:into:123#edv".parse().unwrap())
        .service_type("EncryptedDataVault")
        .endpoint(endpoint)
        .build()
        .unwrap();

    // generate a did doc.
    let mut did_doc = DIDDocumentBuilder::default()
        .context(vec![DID::BASE_CONTEXT.into()])
        .id("did:iota:123456789abcdefghi".parse().unwrap())
        .build()
        .unwrap();

    // insert the service twice.
    did_doc.update_service(service.clone());
    did_doc.update_service(service);

    // insert the public key twice.
    did_doc.update_public_key(public_key.clone());
    did_doc.update_public_key(public_key);

    // insert both auths.
    did_doc.update_agreement(auth.clone());
    did_doc.update_agreement(auth.clone());

    // expect length of the services, keys and agreements.
    let expected_length = 1;

    // failed structure of the agreement field.
    let failed_auth = vec![auth.clone(), auth];

    assert_eq!(expected_length, did_doc.services.len());
    assert_eq!(expected_length, did_doc.public_keys.len());
    assert_eq!(expected_length, did_doc.agreement.len());

    assert_ne!(failed_auth, did_doc.agreement);
}
