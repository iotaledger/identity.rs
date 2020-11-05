use identity_core::{
    common::ToJson as _,
    did::DIDDocument,
    key::{KeyIndex, KeyRelation},
};

use crate::{
    document::LdDocument,
    error::{Error, Result},
    signature::LdSignature,
};

impl LdDocument for DIDDocument {
    fn verification_method(&self) -> Option<&str> {
        self.metadata()
            .get("proof")
            .and_then(|proof| proof.as_object())
            .and_then(|proof| proof.get("verificationMethod"))
            .and_then(|method| method.as_str())
    }

    fn resolve_key(&self, index: KeyIndex) -> Result<Vec<u8>> {
        self.resolve_key(index, KeyRelation::Authentication)
            .ok_or(Error::InvalidDocument)?
            .key_data()
            .try_decode()
            .ok_or(Error::InvalidDocument)?
            .map_err(|_| Error::InvalidDocument)
    }

    fn set_proof(&mut self, value: LdSignature) -> Result<()> {
        self.metadata_mut().insert("proof".into(), value.to_json_value()?);

        Ok(())
    }

    fn set_signature(&mut self, value: String) -> Result<()> {
        self.metadata_mut()
            .get_mut("proof")
            .ok_or(Error::InvalidSignature)?
            .as_object_mut()
            .ok_or(Error::InvalidSignature)?
            .insert("signatureValue".into(), value.into());

        Ok(())
    }
}
