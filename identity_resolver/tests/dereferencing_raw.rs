//! cargo test deref  -- --nocapture
use identity_resolver::dereferencing::{dereference_raw, DereferenceRawResult, Property};

use identity_core::{
    document::DIDDocument,
    utils::{Authentication, Context, KeyData, PublicKey, Service, ServiceEndpoint, Subject},
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deref_key() {
        // Create example did document
        let mut did_doc = DIDDocument {
            context: Context::from("https://w3id.org/did/v1"),
            id: Subject::from("did:iota:123456789abcdefghi"),
            ..Default::default()
        }
        .init();

        // Create example public key
        let key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
        let public_key_object = PublicKey {
            id: "did:iota:123456789abcdefghi#keys-1".into(),
            key_type: "RsaVerificationKey2018".into(),
            controller: "did:iota:123456789abcdefghi".into(),
            key_data,
            ..Default::default()
        }
        .init();
        did_doc.update_public_key(public_key_object.clone());

        let res = dereference_raw("did:iota:123456789abcdefghi#keys-1".into(), did_doc).unwrap();
        if let DereferenceRawResult::Property(prop) = res {
            match *prop {
                Property::PublicKey(key) => assert_eq!(public_key_object, key),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn deref_service() {
        // Create example did document
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
        };
        did_doc.update_service(service.clone());

        let res = dereference_raw("did:iota:123456789abcdefghi#edv".into(), did_doc).unwrap();
        if let DereferenceRawResult::Property(prop) = res {
            match *prop {
                Property::Service(serv) => assert_eq!(service, serv),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn deref_auth() {
        // Create example did document
        let mut did_doc = DIDDocument {
            context: Context::from("https://w3id.org/did/v1"),
            id: Subject::from("did:iota:123456789abcdefghi"),
            ..Default::default()
        }
        .init();
        let auth_key_data = KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into());
        let auth_key = PublicKey {
            id: "did:iota:123456789abcdefghi#keys-2".into(),
            key_type: "Ed25519VerificationKey2018".into(),
            controller: "did:iota:123456789abcdefghi".into(),
            key_data: auth_key_data,
            ..Default::default()
        }
        .init();
        let auth = Authentication::Key(auth_key);
        did_doc.update_auth(auth.clone());
        let res = dereference_raw("did:iota:123456789abcdefghi#keys-2".into(), did_doc).unwrap();
        if let DereferenceRawResult::Property(prop) = res {
            match *prop {
                Property::Authentication(authentication) => assert_eq!(auth, authentication),
                _ => assert!(false),
            }
        } else {
            assert!(false)
        }
    }
    #[test]
    fn deref_service_query() {
        // Create example did document
        let mut did_doc = DIDDocument {
            context: Context::from("https://w3id.org/did/v1"),
            id: Subject::from("did:example:123456789abcdefghi"),
            ..Default::default()
        }
        .init();
        let endpoint = ServiceEndpoint {
            context: "https://example.com/messages/8377464".into(),
            ..Default::default()
        }
        .init();
        let service = Service {
            id: "did:example:123456789abcdefghi#messages".into(),
            service_type: "MessagingService".into(),
            endpoint,
        };
        did_doc.update_service(service.clone());

        let res = dereference_raw(
            "did:example:123456789abcdefghi?service=messages&relative-ref=%2Fsome%2Fpath%3Fquery#frag".into(),
            did_doc,
        )
        .unwrap();
        if let DereferenceRawResult::Stringresult(res) = res {
            assert_eq!("https://example.com/messages/8377464/some/path?query#frag", res);
        } else {
            assert!(false)
        }
    }
}
