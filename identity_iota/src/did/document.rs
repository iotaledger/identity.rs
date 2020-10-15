use identity_core::{
    common::{AsJson as _, Object, SerdeInto as _, Value},
    did::DIDDocument,
    key::{KeyData, KeyRelation, KeyType, PublicKey},
    utils::{decode_b58, decode_hex, encode_b58},
};
use identity_crypto::{Ed25519, Secp256k1, SecretKey, Sign as _, Verify as _};
use serde::{Deserialize, Serialize};

use crate::{
    did::{DIDDiff, DIDProof, TangleDocument},
    error::{Error, Result},
};

impl TangleDocument for DIDDocument {
    fn sign_diff_unchecked(&self, diff: &mut DIDDiff, secret: &SecretKey) -> Result<()> {
        // Get the first authentication key from the document.
        let key: &PublicKey = self
            .resolve_key(0, KeyRelation::Authentication)
            .ok_or(Error::InvalidAuthenticationKey)?;

        // Reset the proof object in the diff.
        diff.proof = DIDProof::new(key.id().clone());

        // Create a signature from the diff JSON.
        let signature: String = sign_canonicalized(diff, key.key_type(), secret)?;

        // Update the diff proof with the encoded signature.
        diff.proof.signature = signature;

        Ok(())
    }

    fn verify_diff_unchecked(&self, diff: &DIDDiff) -> Result<()> {
        // TODO: Validate diff.id
        // TODO: Validate diff.prevMsg

        // Get the first authentication key from the document.
        let key: &PublicKey = self
            .resolve_key(0, KeyRelation::Authentication)
            .ok_or(Error::InvalidAuthenticationKey)?;

        verify_canonicalized(diff, key)
    }

    fn sign_unchecked(&mut self, secret: &SecretKey) -> Result<()> {
        // Get the first authentication key from the document.
        let key: &PublicKey = self
            .resolve_key(0, KeyRelation::Authentication)
            .ok_or(Error::InvalidAuthenticationKey)?;

        let key_type: KeyType = key.key_type();
        let proof: DIDProof = DIDProof::new(key.id().clone());
        let proof: Object = proof.serde_into()?;

        // Reset the proof object in the document.
        self.set_metadata("proof", proof);

        // Create a signature from the document JSON.
        let signature: String = sign_canonicalized(self, key_type, secret)?;

        // Update the document proof with the encoded signature.
        //
        // Note: This access should not panic since we already set the "proof" object.
        self.metadata_mut()["proof"]["signature"] = signature.into();

        Ok(())
    }

    fn verify_unchecked(&self) -> Result<()> {
        // Get the first authentication key from the document.
        let key: &PublicKey = self
            .resolve_key(0, KeyRelation::Authentication)
            .ok_or(Error::InvalidAuthenticationKey)?;

        // TODO: Validate self.id == DID::parse(key.key_data())

        verify_canonicalized(self, key)
    }
}

fn sign_canonicalized<T>(data: &T, key_type: KeyType, secret: &SecretKey) -> Result<String>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    // Serialize as canonicalized JSON.
    // TODO: Canonicalize
    let data: Vec<u8> = data.to_json_vec()?;

    // Create a signature from the canonicalized JSON.
    let signature: Vec<u8> = sign(&data, key_type, secret)?;

    Ok(encode_b58(&signature))
}

fn sign(data: &[u8], key_type: KeyType, secret: &SecretKey) -> Result<Vec<u8>> {
    match key_type {
        KeyType::JsonWebKey2020 => todo!("Not Supported: JsonWebKey2020"),
        KeyType::EcdsaSecp256k1VerificationKey2019 => Secp256k1.sign(&data, secret).map_err(Into::into),
        KeyType::Ed25519VerificationKey2018 => Ed25519.sign(&data, secret).map_err(Into::into),
        KeyType::GpgVerificationKey2020 => todo!("Not Supported: GpgVerificationKey2020"),
        KeyType::RsaVerificationKey2018 => todo!("Not Supported: RsaVerificationKey2018"),
        KeyType::X25519KeyAgreementKey2019 => todo!("Not Supported: X25519KeyAgreementKey2019"),
        KeyType::SchnorrSecp256k1VerificationKey2019 => todo!("Not Supported: SchnorrSecp256k1VerificationKey2019"),
        KeyType::EcdsaSecp256k1RecoveryMethod2020 => todo!("Not Supported: EcdsaSecp256k1RecoveryMethod2020"),
    }
}

fn verify_canonicalized<T>(data: &T, key: &PublicKey) -> Result<()>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    // Convert the diff to a JSON object.
    let mut data: Object = data.serde_into()?;

    // Remove the signature from the proof.
    let signature: Vec<u8> = data
        .get_mut("proof")
        .ok_or(Error::InvalidProof)?
        .as_object_mut()
        .ok_or(Error::InvalidProof)?
        .remove("signature")
        .and_then(|value| match value {
            Value::String(value) => decode_b58(&value).ok(),
            _ => None,
        })
        .ok_or(Error::InvalidProof)?;

    // Serialize as canonicalized JSON.
    // TODO: Canonicalize
    let data: Vec<u8> = data.to_json_vec()?;

    if verify(&data, &signature, key)? {
        Ok(())
    } else {
        Err(Error::InvalidProof)
    }
}

fn verify(data: &[u8], signature: &[u8], key: &PublicKey) -> Result<bool> {
    match (key.key_type(), key.key_data()) {
        (KeyType::JsonWebKey2020, KeyData::PublicKeyJwk(_)) => todo!("Not Supported: JsonWebKey2020/PublicKeyJwk"),

        (KeyType::EcdsaSecp256k1VerificationKey2019, KeyData::PublicKeyHex(inner)) => {
            let key: Vec<u8> = decode_hex(inner)?;
            let valid: bool = Secp256k1.verify(data, signature, &key.into())?;

            Ok(valid)
        }
        (KeyType::EcdsaSecp256k1VerificationKey2019, KeyData::PublicKeyJwk(_)) => {
            todo!("Not Supported: EcdsaSecp256k1VerificationKey2019/PublicKeyJwk")
        }

        (KeyType::Ed25519VerificationKey2018, KeyData::PublicKeyJwk(_)) => {
            todo!("Not Supported: Ed25519VerificationKey2018/PublicKeyJwk")
        }
        (KeyType::Ed25519VerificationKey2018, KeyData::PublicKeyBase58(inner)) => {
            let key: Vec<u8> = decode_b58(inner)?;
            let valid: bool = Ed25519.verify(data, signature, &key.into())?;

            Ok(valid)
        }

        // (KeyType::GpgVerificationKey2020, KeyData::PublicKeyGpg(_)) => {}
        (KeyType::RsaVerificationKey2018, KeyData::PublicKeyJwk(_)) => {
            todo!("Not Supported: RsaVerificationKey2018/PublicKeyJwk")
        }
        (KeyType::RsaVerificationKey2018, KeyData::PublicKeyPem(_)) => {
            todo!("Not Supported: RsaVerificationKey2018/PublicKeyPem")
        }

        (KeyType::X25519KeyAgreementKey2019, KeyData::PublicKeyJwk(_)) => {
            todo!("Not Supported: X25519KeyAgreementKey2019/PublicKeyJwk")
        }
        (KeyType::X25519KeyAgreementKey2019, KeyData::PublicKeyBase58(_)) => {
            todo!("Not Supported: X25519KeyAgreementKey2019/PublicKeyBase58")
        }

        (KeyType::SchnorrSecp256k1VerificationKey2019, KeyData::PublicKeyJwk(_)) => {
            todo!("Not Supported: SchnorrSecp256k1VerificationKey2019/PublicKeyJwk")
        }
        (KeyType::SchnorrSecp256k1VerificationKey2019, KeyData::PublicKeyBase58(_)) => {
            todo!("Not Supported: SchnorrSecp256k1VerificationKey2019/PublicKeyBase58")
        }

        (KeyType::EcdsaSecp256k1RecoveryMethod2020, KeyData::EthereumAddress(_)) => {
            todo!("Not Supported: EcdsaSecp256k1RecoveryMethod2020/EthereumAddress")
        }
        (KeyType::EcdsaSecp256k1RecoveryMethod2020, KeyData::PublicKeyHex(_)) => {
            todo!("Not Supported: EcdsaSecp256k1RecoveryMethod2020/PublicKeyHex")
        }
        (KeyType::EcdsaSecp256k1RecoveryMethod2020, KeyData::PublicKeyJwk(_)) => {
            todo!("Not Supported: EcdsaSecp256k1RecoveryMethod2020/PublicKeyJwk")
        }
        (_, _) => todo!("Invalid KeyType/KeyData Configuration"),
    }
}
