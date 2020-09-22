use identity_core::{
    did::DID,
    document::DIDDocument,
    utils::{Context, KeyData, PublicKey, Service, ServiceEndpoint, Subject},
};

use std::{collections::HashSet, str::FromStr};

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

/// test doc creation via the `DIDDocument::new` method.
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

    let mut public_keys = HashSet::new();
    public_keys.insert(public_key);

    let mut did_doc_2 = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:123456789abcdefghi"),
        public_keys,
        ..Default::default()
    }
    .init();

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

#[test]
fn test_doc_diff_timestamps() {
    let json_str = setup_json("doc");

    let doc1 = DIDDocument::default();

    let doc2 = DIDDocument::from_str(&json_str);

    let mut doc2 = doc2.unwrap();
    doc2.update_time();
    let diff = doc1.diff(&doc2).unwrap();

    let merge = doc1.merge(diff).unwrap();

    assert_eq!(merge, doc2);
}

#[test]
fn test_diff_merge_from_string() {
    let diff_str = setup_json("diff");

    let mut doc = DIDDocument {
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

    doc.update_service(service);

    let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());

    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghi#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:123456789abcdefghi".into(),
        key_data,
        ..Default::default()
    }
    .init();

    doc.update_public_key(public_key);

    let diff = DIDDocument::get_diff_from_str(diff_str).unwrap();

    let def_doc = DIDDocument::default();

    let res = def_doc.merge(diff).unwrap();

    assert_eq!(doc, res);
}

#[test]
fn test_doc_metadata() {
    use std::collections::HashMap;

    let json_str = setup_json("doc");
    let result_str = setup_json("metadata");

    let doc = DIDDocument::from_str(&json_str);

    assert!(doc.is_ok());

    let doc = doc.unwrap();
    let mut metadata = HashMap::new();
    metadata.insert("some".into(), "metadata".into());
    metadata.insert("some_more".into(), "metadata_stuff".into());

    let doc = doc.supply_metadata(metadata).unwrap();

    let res_doc = DIDDocument::from_str(&result_str).unwrap();

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

    let mut pub_keys = HashSet::new();
    pub_keys.insert(public_key);
    pub_keys.insert(key1);

    let did_doc_2 = DIDDocument {
        public_keys: pub_keys,
        ..did_doc.clone()
    };

    let diff = did_doc.diff(&did_doc_2).unwrap();

    let json = serde_json::to_string(&diff).unwrap();

    assert_eq!(json_str, json);

    let diff = DIDDocument::get_diff_from_str(json).unwrap();

    let new_doc = did_doc.merge(diff).unwrap();

    assert_eq!(did_doc_2, new_doc);
}
