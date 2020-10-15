use identity_core::utils::{decode_b58, encode_b58};
use identity_crypto::{self as crypto, signature::Ed25519, PublicKey, SecretKey, Sign, Verify};

use crate::{
    document::LinkedDataDocument,
    error::Result,
    signature::{SignatureSuite, SignatureValue},
};

/// An implementation of the 2020 JCS Ed25519 Signature Suite
///
/// [Specification](https://identity.foundation/JcsEd25519Signature2020/)
pub struct JcsEd25519Signature2020;

impl Sign for JcsEd25519Signature2020 {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> crypto::Result<Vec<u8>> {
        Ed25519
            .sign(message, secret)
            .map_err(|error| crypto::Error::SignError(error.into()))
    }
}

impl Verify for JcsEd25519Signature2020 {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> crypto::Result<bool> {
        Ed25519
            .verify(message, signature, public)
            .map_err(|error| crypto::Error::SignError(error.into()))
    }
}

impl SignatureSuite for JcsEd25519Signature2020 {
    fn signature(&self) -> &'static str {
        "JcsEd25519Signature2020"
    }

    fn to_signature_value(&self, signature: Vec<u8>) -> Result<SignatureValue> {
        Ok(SignatureValue::Signature(encode_b58(&signature)))
    }

    fn from_signature_value(&self, signature: &str) -> Result<Vec<u8>> {
        decode_b58(signature).map_err(Into::into)
    }

    fn create_verify_hash(
        &self,
        document: &dyn LinkedDataDocument,
        _options: &dyn LinkedDataDocument,
    ) -> Result<Vec<u8>> {
        let mut object = document.to_object()?;

        if let Some(proof) = object.get_mut("proof").and_then(|proof| proof.as_object_mut()) {
            proof.remove("signatureValue");
        }

        self.canonicalize(&object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use identity_core::common::{SerdeInto as _, Timestamp};
    use identity_crypto::Proof;
    use std::convert::TryFrom;

    use crate::{
        canonicalize::{CanonicalJson, Canonicalize},
        signature::{SignatureOptions, SignatureProof},
    };

    #[test]
    fn test_vector_1() {
        let sk: SecretKey =
            decode_b58("z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b")
                .unwrap()
                .into();
        let created = Timestamp::try_from("2020-01-01T00:00:00Z").unwrap();

        let input = serde_json::json!({
            "foo": "bar",
            "proof": {
                "type": "JcsEd25519Signature2020"
            }
        });

        let signed = serde_json::json!({
            "signatureValue": "4VCNeCSC4Daru6g7oij3QxUL2CS9FZkCYWRMUKyiLuPPK7GWFrM4YtYYQbmgyUXgGuxyKY5Wn1Mh4mmaRkbah4i4",
            "type": "JcsEd25519Signature2020",
            "proofPurpose": "assertionMethod",
            "created": created.to_string(),
        });

        let options = SignatureOptions {
            created: Some(created),
            ..Default::default()
        };
        let suite = SignatureProof::with_options(JcsEd25519Signature2020, options);
        let proof = suite.create(&input, &sk).unwrap();
        let actual = CanonicalJson.canonicalize(&proof).unwrap();
        let expected = CanonicalJson.canonicalize(&signed.serde_into().unwrap()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vector_2() {
        let sk: SecretKey =
            decode_b58("z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b")
                .unwrap()
                .into();
        let created = Timestamp::try_from("2020-01-01T00:00:00Z").unwrap();

        let input = serde_json::json!({
            "id": "did:example:abcd",
            "publicKey": [
                {
                    "id": "did:example:abcd#key-1",
                    "type": "JcsEd25519Signature2020",
                    "controller": "foo-issuer",
                    "publicKeyBase58": "not-a-real-pub-key"
                }
            ],
            "authentication": null,
            "service": [
                {
                    "id": "schema-id",
                    "type": "schema",
                    "serviceEndpoint": "service-endpoint"
                }
            ],
            "proof": {
                "type": "JcsEd25519Signature2020"
            }
        });

        let signed = serde_json::json!({
            "signatureValue": "4qtzqwFxFYUifwfpPhxR6AABn94KnzWF768jcmjHHH8JYtUb4kAXxG6PttmJAbn3b6q1dfraXFdnUc1z2EGHqWdt",
            "type": "JcsEd25519Signature2020",
            "proofPurpose": "assertionMethod",
            "created": created.to_string(),
        });

        let options = SignatureOptions {
            created: Some(created),
            ..Default::default()
        };
        let suite = SignatureProof::with_options(JcsEd25519Signature2020, options);
        let proof = suite.create(&input, &sk).unwrap();
        let actual = CanonicalJson.canonicalize(&proof).unwrap();
        let expected = CanonicalJson.canonicalize(&signed.serde_into().unwrap()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn sign_and_verify() {
        let sk: SecretKey =
            decode_b58("z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b")
                .unwrap()
                .into();
        let pk: PublicKey = decode_b58("4CcKDtU1JNGi8U4D8Rv9CHzfmF7xzaxEAPFA54eQjRHF")
            .unwrap()
            .into();

        let input = serde_json::json!({"id": "did:example:123"});

        let suite = SignatureProof::new(JcsEd25519Signature2020);
        let proof = suite.create(&input, &sk).unwrap();

        assert!(suite.verify(&input, &proof, &pk).is_ok());
    }

    #[test]
    fn modified_key() {
        let sk: SecretKey =
            decode_b58("z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b")
                .unwrap()
                .into();

        let input = serde_json::json!({
            "id": "did:example:123",
            "publicKey": [
                {
                    "id": "did:example:123#key-1",
                    "type": "JcsEd25519Key2020",
                    "controller": "did:example:123",
                    "publicKeyBase58": "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c"
                }
            ],
            "service": [
                {
                    "id": "schemaID",
                    "type": "schema",
                    "serviceEndpoint": "schemaID"
                }
            ],
            "proof": {
                "created": "2020-04-17T18:03:18Z",
                "verificationMethod": "did:example:123#key-1",
                "nonce": "7bc22433-2ea4-4d30-abf2-2652bebb26c7",
                "type": "JcsEd25519Signature2020"
            }
        });

        let noop = PublicKey::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        let suite = SignatureProof::new(JcsEd25519Signature2020);
        let proof = suite.create(&input, &sk).unwrap();

        assert!(suite.verify(&input, &proof, &noop).is_err());
    }

    #[test]
    fn modified_signature() {
        let sk: SecretKey =
            decode_b58("z3nisqMdwW7nZdWomCfUyRaezHzKEBfwRbvaMcJAqaMSbmjxuRfS5qz4ff3QAf1A5sZT4ZMcxYoGjN4K1VsDV7b")
                .unwrap()
                .into();
        let pk: PublicKey = decode_b58("4CcKDtU1JNGi8U4D8Rv9CHzfmF7xzaxEAPFA54eQjRHF")
            .unwrap()
            .into();

        let input = serde_json::json!({
            "id": "did:example:123",
            "publicKey": [
                {
                    "id": "did:example:123#key-1",
                    "type": "JcsEd25519Key2020",
                    "controller": "did:example:123",
                    "publicKeyBase58": "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c"
                }
            ],
            "service": [
                {
                    "id": "schemaID",
                    "type": "schema",
                    "serviceEndpoint": "schemaID"
                }
            ],
            "proof": {
                "created": "2020-04-17T18:03:18Z",
                "verificationMethod": "did:example:123#key-1",
                "nonce": "7bc22433-2ea4-4d30-abf2-2652bebb26c7",
                "type": "JcsEd25519Signature2020"
            }
        });

        let suite = SignatureProof::new(JcsEd25519Signature2020);
        let mut proof = suite.create(&input, &sk).unwrap();

        let signature = format!("-{}-", &proof["signatureValue"]);
        proof.insert("signatureValue".into(), signature.into());

        assert!(suite.verify(&input, &proof, &pk).is_err());
    }
}
