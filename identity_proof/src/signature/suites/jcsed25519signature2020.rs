use identity_core::{
    common::{SerdeInto as _, ToJson as _, Value},
    did::DIDDocument as Document,
    key::KeyRelation,
    utils::{decode_b58, encode_b58},
};
use identity_crypto::{
    sha2::{self, Digest as _},
    KeyPair,
};
use serde::Serialize;
use sodiumoxide::crypto::sign::ed25519;

use crate::{
    error::{Error, Result},
    signature::{LdSignature, SignatureOptions},
};

const SIGNATURE_TYPE: &str = "JcsEd25519Signature2020";

pub fn new_keypair() -> KeyPair {
    let (public, secret): (ed25519::PublicKey, ed25519::SecretKey) = ed25519::gen_keypair();

    KeyPair::new(public.as_ref().to_vec().into(), secret.as_ref().to_vec().into())
}

fn pubkey(slice: &[u8]) -> Result<ed25519::PublicKey> {
    ed25519::PublicKey::from_slice(slice).ok_or(Error::InvalidKeyFormat)
}

fn seckey(slice: &[u8]) -> Result<ed25519::SecretKey> {
    ed25519::SecretKey::from_slice(slice).ok_or(Error::InvalidKeyFormat)
}

// output = <SIGNATURE><MESSAGE>
fn ed25519_sign(message: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
    seckey(secret).map(|secret| ed25519::sign(message, &secret))
}

// signature = <SIGNATURE><MESSAGE>
fn ed25519_verify(signature: &[u8], public: &[u8]) -> Result<Vec<u8>> {
    pubkey(public).and_then(|public| ed25519::verify(signature, &public).map_err(|_| Error::InvalidSignature))
}

pub fn jcs_sign<T>(document: &T, secret: &[u8]) -> Result<String>
where
    T: Serialize,
{
    serde_jcs::to_vec(document)
        .map_err(|_| Error::InvalidDocument)
        .map(|canon| sha2::Sha256::digest(&canon))
        .and_then(|digest| ed25519_sign(&digest, secret))
        .map(|signature| encode_b58(&signature))
}

pub fn jcs_verify<T>(document: &T, public: &[u8]) -> Result<()>
where
    T: Serialize,
{
    let mut json: Value = document.to_json_value()?;

    let proof: LdSignature = json
        .get("proof")
        .map(|data| data.serde_into())
        .transpose()?
        .ok_or(Error::InvalidSignature)?;

    if proof.type_ != SIGNATURE_TYPE {
        return Err(Error::InvalidSignature);
    }

    let signature: Vec<u8> = decode_b58(proof.proof())?;
    let verified: Vec<u8> = ed25519_verify(&signature, public)?;

    // Remove the signature from the JSON object
    json["proof"]
        .as_object_mut()
        .ok_or(Error::InvalidSignature)?
        .remove(proof.data.key());

    let digest = serde_jcs::to_vec(&json)
        .map_err(|_| Error::InvalidDocument)
        .map(|canon| sha2::Sha256::digest(&canon))?;

    if digest[..] == verified[..] {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

pub fn sign_lds(document: &mut Document, options: SignatureOptions, secret: &[u8]) -> Result<()> {
    let fragment: &str = extract_verification(&options.verification_method)?;
    let keydata: Vec<u8> = resolve_key(document, fragment)?;

    // The verification method key data MUST be equal to the derived public key data.
    if pubkey(&keydata)? != seckey(secret)?.public_key() {
        return Err(Error::InvalidDocument);
    }

    // Create and serialize a proof with a blank signature
    let proof: Value = LdSignature::new(SIGNATURE_TYPE, options).to_json_value()?;

    // Add the proof to the DID document.
    document.metadata_mut().insert("proof".into(), proof);

    // Create an encoded signature
    let signature: String = jcs_sign(&document, secret)?;

    // Add the signature to the proof object within the DID document.
    document
        .metadata_mut()
        .get_mut("proof")
        .ok_or(Error::InvalidSignature)?
        .as_object_mut()
        .ok_or(Error::InvalidSignature)?
        .insert("signatureValue".into(), signature.into());

    Ok(())
}

pub fn verify_lds(document: &Document) -> Result<()> {
    // Extract the verification method from the proof
    let method: &str = document
        .metadata()
        .get("proof")
        .and_then(|proof| proof.as_object())
        .and_then(|proof| proof.get("verificationMethod"))
        .and_then(|method| method.as_str())
        .ok_or(Error::InvalidDocument)?;

    let fragment: &str = extract_verification(method)?;
    let keydata: Vec<u8> = resolve_key(document, fragment)?;

    jcs_verify(document, &keydata)
}

fn extract_verification(method: &str) -> Result<&str> {
    // "Parse" the verification method identifier.
    let fragment: &str = method.trim_start_matches('#');

    // The verification method identifier MUST NOT be empty.
    if fragment.is_empty() {
        Err(Error::InvalidDocument)
    } else {
        Ok(fragment)
    }
}

fn resolve_key(document: &Document, fragment: &str) -> Result<Vec<u8>> {
    // The DID document MUST have a verification method with the specified fragment.
    document
        .resolve_key(fragment, KeyRelation::Authentication)
        .ok_or(Error::InvalidDocument)?
        .key_data()
        .try_decode()
        .ok_or(Error::InvalidDocument)?
        .map_err(|_| Error::InvalidDocument)
}

#[cfg(test)]
mod tests {
    const UNSIGNED: &str = r#"
    {
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
    }
  "#;

    const SIGNED: &str = r#"
    {
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
        "type": "JcsEd25519Signature2020",
        "signatureValue": "5TcawVLuoqRjCuu4jAmRqBcKoab1YVqxG8RXnQwvQBHNwP7RhPwXhzhTLVu3dKGposo2mmtfx9AwcqB2Mwnagup1JT5Yr9u3SjzLCc6kx4wW6HG5SKcra4SauhutN94s8Eo"
      }
    }
  "#;

    use super::{ed25519_sign, ed25519_verify, jcs_sign, jcs_verify};

    use identity_core::{
        common::Value,
        utils::{decode_b58, decode_hex, encode_hex},
    };

    const PUBLIC_B58: &str = "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c";
    const SECRET_B58: &str = "3qsrFcQqVuPpuGrRkU4wkQRvw1tc1C5EmEDPioS1GzQ2pLoThy5TYS2BsrwuzHYDnVqcYhMSpDhTXGst6H5ttFkG";
    const EXPECTED: &str = "0ccbeb905006a327b5112c7bfaa2a5918784209818a83750548b9965661b9d1d467c4078faacbaa36c1bd0f88673039adea51f5d216cd45cbf0e1528fb67f10a68656c6c6f";

    #[test]
    fn test_ed25519_can_sign_and_verify() {
        let public: Vec<u8> = decode_b58(PUBLIC_B58).unwrap();
        let secret: Vec<u8> = decode_b58(SECRET_B58).unwrap();
        let expected: Vec<u8> = decode_hex(EXPECTED).unwrap();

        let signature = ed25519_sign(b"hello", &secret).unwrap();
        let verified = ed25519_verify(&expected, &public).unwrap();

        assert_eq!(encode_hex(&signature), EXPECTED);
        assert_eq!(&verified, b"hello");
    }

    #[test]
    fn test_jcsed25519signature2020_can_sign_and_verify() {
        let public = decode_b58(PUBLIC_B58).unwrap();
        let secret = decode_b58(SECRET_B58).unwrap();
        let expected = "5TcawVLuoqRjCuu4jAmRqBcKoab1YVqxG8RXnQwvQBHNwP7RhPwXhzhTLVu3dKGposo2mmtfx9AwcqB2Mwnagup1JT5Yr9u3SjzLCc6kx4wW6HG5SKcra4SauhutN94s8Eo";

        let mut unsigned = serde_json::from_str::<Value>(UNSIGNED).unwrap();
        let signed = serde_json::from_str::<Value>(SIGNED).unwrap();

        let signature = jcs_sign(&unsigned, &secret).unwrap();

        assert_eq!(signature, expected);

        unsigned["proof"]["signatureValue"] = signature.into();

        println!("U > {:#}", unsigned);

        assert_eq!(
            serde_jcs::to_vec(&unsigned).unwrap(),
            serde_jcs::to_vec(&signed).unwrap(),
        );

        let verified = jcs_verify(&unsigned, &public);

        assert!(verified.is_ok());
    }

    #[test]
    fn test_jcsed25519signature2020_fails_when_key_is_mutated() {
        let public = decode_hex("00015daa95f69cbd3f431ff5a3b2eefe2bb5d9ea0d296607446aab7b7106f3ed").unwrap();
        let secret = decode_b58(SECRET_B58).unwrap();
        let expected = "5TcawVLuoqRjCuu4jAmRqBcKoab1YVqxG8RXnQwvQBHNwP7RhPwXhzhTLVu3dKGposo2mmtfx9AwcqB2Mwnagup1JT5Yr9u3SjzLCc6kx4wW6HG5SKcra4SauhutN94s8Eo";

        let mut document = serde_json::from_str::<Value>(UNSIGNED).unwrap();
        let signature = jcs_sign(&document, &secret).unwrap();

        assert_eq!(signature, expected);

        document["proof"]["signatureValue"] = signature.into();

        let verified = jcs_verify(&document, &public);

        assert!(verified.is_err());
    }

    #[test]
    fn test_jcsed25519signature2020_fails_when_signature_is_mutated() {
        let public = decode_b58(PUBLIC_B58).unwrap();
        let secret = decode_b58(SECRET_B58).unwrap();
        let expected = "5TcawVLuoqRjCuu4jAmRqBcKoab1YVqxG8RXnQwvQBHNwP7RhPwXhzhTLVu3dKGposo2mmtfx9AwcqB2Mwnagup1JT5Yr9u3SjzLCc6kx4wW6HG5SKcra4SauhutN94s8Eo";

        let mut document = serde_json::from_str::<Value>(UNSIGNED).unwrap();
        let mut signature = jcs_sign(&document, &secret).unwrap();

        assert_eq!(signature, expected);

        signature.push('0');

        document["proof"]["signatureValue"] = signature.into();

        let verified = jcs_verify(&document, &public);

        assert!(verified.is_err());
    }
}
