use core::convert::TryInto as _;
use did_doc::{Error, Method, Result, Sign, SignatureData, SuiteName, Verify};
use ed25519_zebra::{Signature, SigningKey, VerificationKey, VerificationKeyBytes};
use rand::rngs::OsRng;
use serde::Serialize;
use sha2::{digest::Output, Sha256};
use subtle::ConstantTimeEq;

use crate::{
    crypto::KeyPair,
    utils::{decode_b58, encode_b58, jcs_sha256},
};

const SIGNATURE_NAME: &str = "JcsEd25519Signature2020";
const SIGNATURE_SIZE: usize = 64;
const PUBLIC_KEY_BYTES: usize = 32;
const SECRET_KEY_BYTES: usize = 32;

/// An implementation of the [JCS Ed25519 Signature 2020][SPEC1] signature suite
/// for [Linked Data Proofs][SPEC2].
///
/// [SPEC1]: https://identity.foundation/JcsEd25519Signature2020/
/// [SPEC2]: https://w3c-ccg.github.io/ld-proofs/
#[derive(Clone, Copy, Debug)]
pub struct JcsEd25519Signature2020;

impl JcsEd25519Signature2020 {
    /// The name of the signature suite.
    pub const NAME: &'static str = SIGNATURE_NAME;

    /// Generates a new [`KeyPair`] appropriate for this signature suite.
    pub fn new_keypair() -> KeyPair {
        let secret: SigningKey = SigningKey::new(OsRng);
        let public: VerificationKey = (&secret).into();
        let public: VerificationKeyBytes = public.into();

        KeyPair::new(public.as_ref().to_vec().into(), secret.as_ref().to_vec().into())
    }

    /// Signs the given `data` with `secret` and returns a digital signature.
    pub fn sign_data<T>(data: &T, secret: &[u8]) -> Result<SignatureData>
    where
        T: Serialize,
    {
        Self::normalize(data)
            .and_then(|data| ed25519_sign(&data, secret))
            .map_err(|_| Error::message("Cannot Sign Document"))
            .map(|data| encode_b58(&data))
            .map(SignatureData::Signature)
    }

    /// Verifies the authenticity of `data` using `signature` and `public`.
    pub fn verify_data<T>(data: &T, signature: &SignatureData, public: &[u8]) -> Result<()>
    where
        T: Serialize,
    {
        let signature: &str = signature
            .try_signature()
            .ok_or_else(|| Error::message("Signature Data Not Found"))?;

        let signature: Vec<u8> = decode_b58(&signature).map_err(|_| Error::message("Invalid Signature Data"))?;
        let verified: Vec<u8> = ed25519_verify(&signature, public)?;
        let digest: _ = Self::normalize(data)?;

        if digest[..].ct_eq(&verified[..]).into() {
            Ok(())
        } else {
            Err(Error::message("Invalid Signature Digest"))
        }
    }

    fn normalize<T>(data: &T) -> Result<Output<Sha256>>
    where
        T: Serialize,
    {
        jcs_sha256(data).map_err(|_| Error::message("Cannot Serialize Document"))
    }
}

impl SuiteName for JcsEd25519Signature2020 {
    fn name(&self) -> String {
        Self::NAME.to_string()
    }
}

impl Sign for JcsEd25519Signature2020 {
    fn sign<T>(&self, data: &T, secret: &[u8]) -> Result<SignatureData>
    where
        T: Serialize,
    {
        Self::sign_data(data, secret)
    }
}

impl Verify for JcsEd25519Signature2020 {
    fn verify<T, U>(&self, data: &T, signature: &SignatureData, method: &Method<U>) -> Result<()>
    where
        T: Serialize,
        U: Serialize,
    {
        Self::verify_data(data, signature, &method.key_data().try_decode()?)
    }
}

fn pubkey(slice: &[u8]) -> Result<VerificationKey> {
    if slice.len() < PUBLIC_KEY_BYTES {
        return Err(Error::message("Invalid Key Format"));
    }

    slice[..PUBLIC_KEY_BYTES]
        .try_into()
        .map_err(|_| Error::message("Invalid Key Format"))
}

fn seckey(slice: &[u8]) -> Result<SigningKey> {
    if slice.len() < SECRET_KEY_BYTES {
        return Err(Error::message("Invalid Key Format"));
    }

    slice[..SECRET_KEY_BYTES]
        .try_into()
        .map_err(|_| Error::message("Invalid Key Format"))
}

// output = <SIGNATURE><MESSAGE>
fn ed25519_sign(message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
    let key: SigningKey = seckey(secret)?;
    let sig: [u8; SIGNATURE_SIZE] = key.sign(message).into();

    Ok([&sig, message].concat())
}

// signature = <SIGNATURE><MESSAGE>
fn ed25519_verify(signature: &[u8], public: &[u8]) -> Result<Vec<u8>> {
    if signature.len() < SIGNATURE_SIZE {
        return Err(Error::message("Invalid Key Format"));
    }

    let key: VerificationKey = pubkey(public)?;
    let (sig, msg): (&[u8], &[u8]) = signature.split_at(SIGNATURE_SIZE);
    let sig: Signature = sig.try_into().map_err(|_| Error::message("Invalid Key Format"))?;

    key.verify(&sig, msg)
        .map_err(|_| Error::message("Invalid Key Format"))?;

    Ok(msg.to_vec())
}

#[cfg(test)]
mod tests {
    const UNSIGNED: &str = r##"
    {
        "id": "did:example:123",
        "verificationMethod": [
            {
                "id": "did:example:123#key-1",
                "type": "JcsEd25519Key2020",
                "controller": "did:example:123",
                "publicKeyBase58": "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c"
            }
        ],
        "service": [
            {
                "id": "did:schema:id",
                "type": "schema",
                "serviceEndpoint": "https://example.com"
            }
        ]
    }
    "##;

    const SIGNED: &str = r##"
    {
        "id": "did:example:123",
        "verificationMethod": [
            {
                "id": "did:example:123#key-1",
                "type": "JcsEd25519Key2020",
                "controller": "did:example:123",
                "publicKeyBase58": "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c"
            }
        ],
        "service": [
            {
                "id": "did:schema:id",
                "type": "schema",
                "serviceEndpoint": "https://example.com"
            }
        ],
        "proof": {
            "verificationMethod": "#key-1",
            "type": "JcsEd25519Signature2020",
            "signatureValue": "piKnvB438vWsinW1dqq2EYRzcYFuR7Qm9X8t2S6TPPLDokLwcFBXnnERk6jmS8RXKTJnXKWw1Q9oNhYTwbR7vJkaJT8ZGgwDHNxa6mrMNsQsWkM4rg6EYY99xQko7FnpAMn"
        }
    }
    "##;

    use super::{ed25519_sign, ed25519_verify, JcsEd25519Signature2020};

    use crate::{
        convert::FromJson as _,
        did_doc::{LdSuite, SignatureData, SignatureOptions, VerifiableDocument},
        utils::decode_b58,
    };

    const PUBLIC_B58: &str = "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c";
    const SECRET_B58: &str = "3qsrFcQqVuPpuGrRkU4wkQRvw1tc1C5EmEDPioS1GzQ2pLoThy5TYS2BsrwuzHYDnVqcYhMSpDhTXGst6H5ttFkG";

    #[rustfmt::skip]
    const SIGNATURE_HELLO: &[u8] = &[12, 203, 235, 144, 80, 6, 163, 39, 181, 17, 44, 123, 250, 162, 165, 145, 135, 132, 32, 152, 24, 168, 55, 80, 84, 139, 153, 101, 102, 27, 157, 29, 70, 124, 64, 120, 250, 172, 186, 163, 108, 27, 208, 248, 134, 115, 3, 154, 222, 165, 31, 93, 33, 108, 212, 92, 191, 14, 21, 40, 251, 103, 241, 10, 104, 101, 108, 108, 111];

    const SIGNATURE_DOCUMENT: &str = "piKnvB438vWsinW1dqq2EYRzcYFuR7Qm9X8t2S6TPPLDokLwcFBXnnERk6jmS8RXKTJnXKWw1Q9oNhYTwbR7vJkaJT8ZGgwDHNxa6mrMNsQsWkM4rg6EYY99xQko7FnpAMn";

    #[test]
    fn test_ed25519_can_sign_and_verify() {
        let public: Vec<u8> = decode_b58(PUBLIC_B58).unwrap();
        let secret: Vec<u8> = decode_b58(SECRET_B58).unwrap();

        let signature = ed25519_sign(b"hello", &secret).unwrap();
        let verified = ed25519_verify(&signature, &public).unwrap();

        assert_eq!(&signature, SIGNATURE_HELLO);
        assert_eq!(&verified, b"hello");
    }

    #[test]
    fn test_jcsed25519signature2020_can_sign_and_verify() {
        let secret = decode_b58(SECRET_B58).unwrap();
        let mut unsigned: VerifiableDocument = VerifiableDocument::from_json(UNSIGNED).unwrap();
        let signed: VerifiableDocument = VerifiableDocument::from_json(SIGNED).unwrap();
        let method = unsigned.try_resolve("#key-1").unwrap();
        let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());
        let suite: LdSuite<_> = LdSuite::new(JcsEd25519Signature2020);

        suite.sign(&mut unsigned, options, &secret).unwrap();

        assert!(suite.verify(&unsigned).is_ok());
        assert_eq!(
            unsigned.properties().proof().unwrap().data().as_str(),
            SIGNATURE_DOCUMENT
        );

        assert_eq!(
            serde_jcs::to_vec(&unsigned).unwrap(),
            serde_jcs::to_vec(&signed).unwrap()
        );
    }

    #[test]
    fn test_jcsed25519signature2020_fails_when_key_is_mutated() {
        let secret = decode_b58(SECRET_B58).unwrap();
        let mut document: VerifiableDocument = VerifiableDocument::from_json(UNSIGNED).unwrap();
        let method = document.try_resolve("#key-1").unwrap();
        let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());
        let suite: LdSuite<_> = LdSuite::new(JcsEd25519Signature2020);

        suite.sign(&mut document, options, &secret).unwrap();

        assert!(suite.verify(&document).is_ok());
        assert_eq!(
            document.properties().proof().unwrap().data().as_str(),
            SIGNATURE_DOCUMENT
        );

        document.proof_mut().unwrap().verification_method = "#key-2".into();

        assert!(suite.verify(&document).is_err());
    }

    #[test]
    fn test_jcsed25519signature2020_fails_when_signature_is_mutated() {
        let secret = decode_b58(SECRET_B58).unwrap();
        let mut document: VerifiableDocument = VerifiableDocument::from_json(UNSIGNED).unwrap();
        let method = document.try_resolve("#key-1").unwrap();
        let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());
        let suite: LdSuite<_> = LdSuite::new(JcsEd25519Signature2020);

        suite.sign(&mut document, options, &secret).unwrap();

        assert!(suite.verify(&document).is_ok());
        assert_eq!(
            document.properties().proof().unwrap().data().as_str(),
            SIGNATURE_DOCUMENT
        );

        document
            .proof_mut()
            .unwrap()
            .set_data(SignatureData::Signature("foo".into()));

        assert!(suite.verify(&document).is_err());
    }
}
