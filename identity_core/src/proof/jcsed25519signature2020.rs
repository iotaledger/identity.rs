use core::convert::TryInto as _;
use did_doc::{Error, Result, SignatureData, SignatureSuite};
use ed25519_zebra::{Signature, SigningKey, VerificationKey, VerificationKeyBytes};
use rand::rngs::OsRng;
use serde::Serialize;
use sha2::{
    digest::{consts::U32, generic_array::GenericArray},
    Digest as _, Sha256,
};

use crate::{
    crypto::KeyPair,
    utils::{decode_b58, encode_b58},
};

const SIGNATURE_NAME: &str = "JcsEd25519Signature2020";
const SIGNATURE_SIZE: usize = 64;
const PUBLIC_KEY_BYTES: usize = 32;
const SECRET_KEY_BYTES: usize = 32;

#[derive(Clone, Copy, Debug)]
pub struct JcsEd25519Signature2020;

impl JcsEd25519Signature2020 {
    pub fn new_keypair() -> KeyPair {
        let secret: SigningKey = SigningKey::new(OsRng);
        let public: VerificationKey = (&secret).into();
        let public: VerificationKeyBytes = public.into();

        KeyPair::new(public.as_ref().to_vec().into(), secret.as_ref().to_vec().into())
    }

    fn normalize<T>(data: &T) -> Result<GenericArray<u8, U32>>
    where
        T: Serialize,
    {
        serde_jcs::to_vec(data)
            .map_err(|_| Error::message("Cannot Serialize Document"))
            .map(|json| Sha256::digest(&json))
    }
}

impl SignatureSuite for JcsEd25519Signature2020 {
    fn name(&self) -> &'static str {
        SIGNATURE_NAME
    }

    fn sign<T>(&self, message: &T, secret: &[u8]) -> Result<SignatureData>
    where
        T: Serialize,
    {
        Self::normalize(message)
            .and_then(|message| ed25519_sign(&message, secret))
            .map_err(|_| Error::message("Cannot Sign Document"))
            .map(|signature| encode_b58(&signature))
            .map(SignatureData::Signature)
    }

    fn verify<T>(&self, message: &T, signature: &SignatureData, public: &[u8]) -> Result<()>
    where
        T: Serialize,
    {
        let signature: &str = signature
            .try_signature()
            .ok_or_else(|| Error::message("Signature Data Not Found"))?;

        let signature: Vec<u8> = decode_b58(&signature).map_err(|_| Error::message("Invalid Signature Data"))?;
        let verified: Vec<u8> = ed25519_verify(&signature, public)?;
        let digest: GenericArray<u8, U32> = Self::normalize(message)?;

        if digest[..] == verified[..] {
            Ok(())
        } else {
            Err(Error::message("Invalid Signature Digest"))
        }
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
        did_doc::{SignatureData, SignatureOptions, VerifiableDocument},
        utils::{decode_b58, decode_hex, encode_hex},
    };

    const PUBLIC_B58: &str = "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c";
    const SECRET_B58: &str = "3qsrFcQqVuPpuGrRkU4wkQRvw1tc1C5EmEDPioS1GzQ2pLoThy5TYS2BsrwuzHYDnVqcYhMSpDhTXGst6H5ttFkG";

    const SIGNATURE_HELLO: &str = "0ccbeb905006a327b5112c7bfaa2a5918784209818a83750548b9965661b9d1d467c4078faacbaa36c1bd0f88673039adea51f5d216cd45cbf0e1528fb67f10a68656c6c6f";
    const SIGNATURE_DOCUMENT: &str = "piKnvB438vWsinW1dqq2EYRzcYFuR7Qm9X8t2S6TPPLDokLwcFBXnnERk6jmS8RXKTJnXKWw1Q9oNhYTwbR7vJkaJT8ZGgwDHNxa6mrMNsQsWkM4rg6EYY99xQko7FnpAMn";

    #[test]
    fn test_ed25519_can_sign_and_verify() {
        let public: Vec<u8> = decode_b58(PUBLIC_B58).unwrap();
        let secret: Vec<u8> = decode_b58(SECRET_B58).unwrap();
        let expected: Vec<u8> = decode_hex(SIGNATURE_HELLO).unwrap();

        let signature = ed25519_sign(b"hello", &secret).unwrap();
        let verified = ed25519_verify(&expected, &public).unwrap();

        assert_eq!(encode_hex(&signature), SIGNATURE_HELLO);
        assert_eq!(&verified, b"hello");
    }

    #[test]
    fn test_jcsed25519signature2020_can_sign_and_verify() {
        let secret = decode_b58(SECRET_B58).unwrap();
        let mut unsigned: VerifiableDocument = VerifiableDocument::from_json(UNSIGNED).unwrap();
        let signed: VerifiableDocument = VerifiableDocument::from_json(SIGNED).unwrap();
        let method = unsigned.try_resolve("#key-1").unwrap();
        let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());

        unsigned.sign(&JcsEd25519Signature2020, options, &secret).unwrap();

        assert!(unsigned.verify(&JcsEd25519Signature2020).is_ok());
        assert_eq!(
            unsigned.properties().proof().unwrap().data().try_signature().unwrap(),
            SIGNATURE_DOCUMENT
        );

        assert_eq!(
            serde_jcs::to_vec(&unsigned).unwrap(),
            serde_jcs::to_vec(&signed).unwrap(),
        );
    }

    #[test]
    fn test_jcsed25519signature2020_fails_when_key_is_mutated() {
        let secret = decode_b58(SECRET_B58).unwrap();
        let mut document: VerifiableDocument = VerifiableDocument::from_json(UNSIGNED).unwrap();
        let method = document.try_resolve("#key-1").unwrap();
        let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());

        document.sign(&JcsEd25519Signature2020, options, &secret).unwrap();

        assert!(document.verify(&JcsEd25519Signature2020).is_ok());
        assert_eq!(
            document.properties().proof().unwrap().data().try_signature().unwrap(),
            SIGNATURE_DOCUMENT
        );

        document.proof_mut().unwrap().verification_method = "#key-2".into();

        assert!(document.verify(&JcsEd25519Signature2020).is_err());
    }

    #[test]
    fn test_jcsed25519signature2020_fails_when_signature_is_mutated() {
        let secret = decode_b58(SECRET_B58).unwrap();
        let mut document: VerifiableDocument = VerifiableDocument::from_json(UNSIGNED).unwrap();
        let method = document.try_resolve("#key-1").unwrap();
        let options: SignatureOptions = SignatureOptions::new(method.try_into_fragment().unwrap());

        document.sign(&JcsEd25519Signature2020, options, &secret).unwrap();

        assert!(document.verify(&JcsEd25519Signature2020).is_ok());
        assert_eq!(
            document.properties().proof().unwrap().data().try_signature().unwrap(),
            SIGNATURE_DOCUMENT
        );

        document
            .proof_mut()
            .unwrap()
            .data_mut()
            .set(SignatureData::Signature("foo".into()));

        assert!(document.verify(&JcsEd25519Signature2020).is_err());
    }
}
