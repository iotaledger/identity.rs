use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Error as FmtError, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};
use identity_core::{
    common::{OneOrMany, ToJson as _},
    did::{DIDDocument as Document, DIDDocumentBuilder as DocumentBuilder, DID},
    diff::Diff as _,
    key::{KeyRelation, KeyType, PublicKey},
};
use identity_crypto::{KeyPair, SecretKey};
use identity_proof::{signature::jcsed25519signature2020, LdDocument, LdSignature, SignatureOptions};
use serde::{Deserialize, Serialize};

use crate::{
    did::{DIDDiff, IotaDID, LdDiffRead, LdDiffWrite},
    error::{Error, Result},
};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "Document", into = "Document")]
pub struct IotaDocument {
    document: Document,
}

impl IotaDocument {
    pub fn generate_ed25519_keypair() -> KeyPair {
        jcsed25519signature2020::new_keypair()
    }

    pub fn try_from_document(document: Document) -> Result<Self> {
        // The DID document MUST have a well-formed IOTA DID
        let did: IotaDID = IotaDID::try_from_did(document.did().clone())?;

        // The DID document MUST have an authentication key that matches the DID
        let key: Vec<u8> = LdDocument::resolve_key(&document, 0.into())?;
        let tag: String = IotaDID::encode_key(&key);

        // The DID tag MUST equal Base58( Blake2b-256( authentication-key ) )
        if did.method_id() != tag {
            return Err(Error::InvalidAuthenticationKey);
        }

        Ok(Self { document })
    }

    pub fn new(did: IotaDID, authentication: PublicKey) -> Result<Self> {
        // TODO: Validate `authentication`; ensure the DIDs match

        let mut document: Document = DocumentBuilder::default()
            .context(OneOrMany::One(DID::BASE_CONTEXT.into()))
            .id(did.into())
            .auth(vec![authentication.id().clone().into()])
            .public_keys(vec![authentication])
            .build()
            .expect("FIXME");

        document.init_timestamps();

        Ok(Self { document })
    }

    pub fn authentication_key(&self) -> &PublicKey {
        self.resolve_key(0, KeyRelation::Authentication).expect("infallible")
    }

    pub fn sign(&mut self, secret: &SecretKey) -> Result<()> {
        let key: &PublicKey = self.authentication_key();

        let fragment: String = format!("{}", key.id());
        let options: SignatureOptions = SignatureOptions::new(fragment);

        match key.key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::sign_lds(&mut self.document, options, secret)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(())
    }

    pub fn verify(&self) -> Result<()> {
        let key: &PublicKey = self.authentication_key();

        match key.key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::verify_lds(&self.document)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(())
    }

    pub fn diff(&self, mut other: Document, secret: &SecretKey) -> Result<DIDDiff> {
        // Update the `updated` timestamp of the new document
        other.update_time();

        // Get the first authentication key from the document.
        let key: &PublicKey = self.authentication_key();

        let fragment: String = format!("{}", key.id());
        let options: SignatureOptions = SignatureOptions::new(fragment);

        // Create a diff of changes between the two documents.
        let mut diff: DIDDiff = DIDDiff {
            id: self.document.did().clone(),
            diff: self.document.diff(&other)?,
            proof: LdSignature::new("", options.clone()),
        };

        // Wrap the diff/document in a signable type.
        let mut target: LdDiffWrite = LdDiffWrite::new(&mut diff, &self.document);

        // Create and apply the signature
        match key.key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::sign_lds(&mut target, options, secret)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(diff)
    }

    pub fn verify_diff(&self, diff: &DIDDiff) -> Result<()> {
        // Wrap the diff/document in a verifiable type.
        let target: LdDiffRead = LdDiffRead::new(diff, &self.document);

        match self.authentication_key().key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::verify_lds(&target)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(())
    }
}

impl Display for IotaDocument {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
        } else {
            f.write_str(&self.to_json().map_err(|_| FmtError)?)
        }
    }
}

impl Debug for IotaDocument {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(&self.document, f)
    }
}

impl Deref for IotaDocument {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl DerefMut for IotaDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}

impl From<IotaDocument> for Document {
    fn from(other: IotaDocument) -> Self {
        other.document
    }
}

impl TryFrom<Document> for IotaDocument {
    type Error = Error;

    fn try_from(other: Document) -> Result<Self, Self::Error> {
        Self::try_from_document(other)
    }
}
