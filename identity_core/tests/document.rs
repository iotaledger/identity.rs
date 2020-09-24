use identity_core::{
    did::DID,
    document::DIDDocument,
    iota_network,
    utils::{Authentication, Context, IdCompare, KeyData, PublicKey, Service, ServiceEndpoint, Subject},
};

use std::str::FromStr;

use identity_diff::Diff;

const JSON_STR: &str = include_str!("document.json");

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
    let ctx = Context::new(vec![
        "https://w3id.org/did/v1".into(),
        "https://w3id.org/security/v1".into(),
    ]);

    let did = DID {
        method_name: "iota".into(),
        id_segments: vec!["123456789abcdefghi".into()],
        ..Default::default()
    }
    .init()
    .unwrap();

    assert_eq!(doc.context, ctx);
    assert_eq!(doc.id, did.into());
}

/// test doc creation.
#[test]
fn test_doc_creation() {
    let mut did_doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    let endpoint = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    let service = Service {
        id: "did:into:123#edv".into(),
        service_type: "EncryptedDataVault".into(),
        endpoint,
    }
    .init();

    let endpoint2 = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    let service2 = Service {
        id: "did:into:123#edv".into(),
        service_type: "IdentityHub".into(),
        endpoint: endpoint2,
    }
    .init();

    did_doc.update_service(service.clone());
    did_doc.update_service(service2.clone());

    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data,
        ..Default::default()
    }
    .init();

    did_doc.update_public_key(public_key.clone());

    let mut did_doc_2 = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();

    did_doc_2.update_public_key(public_key);

    did_doc_2.update_service(service);
    did_doc_2.update_service(service2);

    let did_doc = did_doc.init_timestamps().unwrap();

    // did_doc has timestamps while did_doc_2 does not.
    assert_ne!(did_doc, did_doc_2);
}

/// test serde_diff on the did document structure.
#[test]
fn test_doc_diff() {
    // old doc
    let old = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    // new doc.
    let mut new = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();

    let endpoint = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    let service = Service {
        id: "did:into:123#edv".into(),
        service_type: "EncryptedDataVault".into(),
        endpoint,
    }
    .init();

    new.update_service(service);

    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data,
        ..Default::default()
    }
    .init();

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
    let mut doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    // create an endpoint.
    let endpoint = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    // create a IdCompare<Service>
    let service = Service {
        id: "did:into:123#edv".into(),
        service_type: "EncryptedDataVault".into(),
        endpoint,
    }
    .init();

    // update the service.
    doc.update_service(service);

    // create some key data.
    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    // create a public key, IdCompare<PublicKey>.
    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data,
        ..Default::default()
    }
    .init();

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
    use std::collections::HashMap;

    // get the json string for a did doc.
    let json_str = setup_json("doc");
    // get the json string for the metadata.
    let result_str = setup_json("metadata");

    // generate a did doc from the json string.
    let doc = DIDDocument::from_str(&json_str);

    assert!(doc.is_ok());

    let doc = doc.unwrap();
    // create a new hashmap and insert the metadata.
    let mut metadata = HashMap::new();
    metadata.insert("some".into(), "metadata".into());
    metadata.insert("some_more".into(), "metadata_stuff".into());

    // add the metadata to the original doc.
    let doc = doc.supply_metadata(metadata).unwrap();

    // get the metadata doc string and create a new did doc from it.
    let res_doc = DIDDocument::from_str(&result_str).unwrap();

    // check to see if the two docs are the same.
    assert_eq!(doc, res_doc);
}

#[test]
fn test_realistic_diff() {
    let json_str = setup_json("diff2");

    let mut did_doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();
    let endpoint = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    let service = Service {
        id: "did:into:123#edv".into(),
        service_type: "EncryptedDataVault".into(),
        endpoint,
    }
    .init();

    let endpoint2 = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    let service2 = Service {
        id: "did:into:123#edv".into(),
        service_type: "IdentityHub".into(),
        endpoint: endpoint2,
    }
    .init();

    did_doc.update_service(service);
    did_doc.update_service(service2);

    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data,
        ..Default::default()
    }
    .init();

    did_doc.update_public_key(public_key.clone());

    let key_data_1 = KeyData::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----".into());
    let key1 = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data: key_data_1,
        ..Default::default()
    }
    .init();

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
    // key data.
    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    // service endpoint.
    let endpoint = ServiceEndpoint {
        context: "https://edv.example.com/".into(),
        ..Default::default()
    }
    .init();

    // create a public key.
    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data,
        ..Default::default()
    }
    .init();

    // create the authentication key.
    let auth = Authentication::Key(public_key.clone());

    // create a IdCompare<Service>
    let service = Service {
        id: "did:into:123#edv".into(),
        service_type: "EncryptedDataVault".into(),
        endpoint,
    }
    .init();

    // generate a did doc.
    let mut did_doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        ..Default::default()
    }
    .init();

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
    let failed_auth = vec![IdCompare::new(auth.clone()), IdCompare::new(auth)];

    assert_eq!(expected_length, did_doc.services.len());
    assert_eq!(expected_length, did_doc.public_keys.len());
    assert_eq!(expected_length, did_doc.agreement.len());

    assert_ne!(failed_auth, did_doc.agreement);
}

/// test doc with DID creation.
#[test]
fn test_doc_with_did_creation() {
    let mut did_doc = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        ..Default::default()
    }
    .init();

    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
    let mut auth_key = PublicKey {
        key_type: "RsaVerificationKey2018".into(),
        key_data,
        ..Default::default()
    }
    .init();
    auth_key.create_own_id(iota_network::Comnet, None).unwrap();

    let auth = Authentication::Key(auth_key.clone());

    did_doc.update_auth(auth);

    did_doc.create_id().unwrap();
    assert_eq!(
        did_doc.derive_did().unwrap().to_string(),
        "did:iota:com:5X7Uq87P7x6P2kdJEiNYun6npfHa21DiozoCWhuJtwPg"
    );
}
