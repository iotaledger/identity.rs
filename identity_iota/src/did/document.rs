use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Deref,
};
use identity_core::{
    common::{Object, Timestamp},
    convert::{FromJson as _, SerdeInto as _},
    crypto::{KeyPair, SecretKey},
    did_doc::{
        Document, DocumentBuilder, Method, MethodBuilder, MethodData, MethodScope, MethodType, MethodWrap,
        SetSignature, Signature, SignatureDocument, SignatureOptions, TrySignature, VerifiableDocument,
    },
    did_url::DID,
    identity_diff::{did_doc::DiffDocument, Diff},
    proof::JcsEd25519Signature2020,
};
use iota::transaction::bundled::Address;
use serde::Serialize;

use crate::{
    client::{Client, ClientBuilder, Network},
    did::{DIDDiff, IotaDID},
    error::{Error, Result},
};

const AUTH_QUERY: (usize, MethodScope) = (0, MethodScope::Authentication);

const ERR_AMNS: &str = "Authentication Method Not Supported";
const ERR_AMMF: &str = "Authentication Method Missing Fragment";
const ERR_AMIM: &str = "Authentication Method Id Mismatch";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Properties {
    pub created: Timestamp,
    pub updated: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_chain: Option<String>,
    #[serde(skip)]
    pub message_id: Option<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            created: Timestamp::now(),
            updated: Timestamp::now(),
            prev_msg: None,
            diff_chain: None,
            message_id: None,
            properties: Object::new(),
        }
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "Document", into = "VerifiableDocument<Properties>")]
pub struct IotaDocument(VerifiableDocument<Properties>);

impl IotaDocument {
    pub fn generate_ed25519<'a, 'b, T, U>(tag: &str, network: T, shard: U) -> Result<(Self, KeyPair)>
    where
        T: Into<Option<&'a str>>,
        U: Into<Option<&'b str>>,
    {
        let (did, keypair): (IotaDID, KeyPair) = IotaDID::generate_ed25519(network, shard)?;
        let key: DID = (*did).join(format!("#{}", tag))?;

        let authentication: Method = MethodBuilder::default()
            .id(key.clone())
            .controller(did.clone().into())
            .key_type(MethodType::Ed25519VerificationKey2018)
            .key_data(MethodData::new_b58(keypair.public()))
            .build()?;

        let this: Self = DocumentBuilder::new(Properties::new())
            .id(did.into())
            // Note: We use a reference to the verification method due to
            // limitations in the did_doc crate.
            .authentication(key)
            .verification_method(authentication)
            .build()
            .map(VerifiableDocument::new)
            .map(Self)?;

        Ok((this, keypair))
    }

    /// Converts a generic DID `Document` to an `IotaDocument`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the document is not a valid `IotaDocument`.
    pub fn try_from_document(mut document: Document) -> Result<Self> {
        let did: &IotaDID = IotaDID::try_from_borrowed(document.id())?;
        let key: &DID = document.try_resolve(AUTH_QUERY)?.into_method().id();

        if key.fragment().is_none() {
            return Err(Error::InvalidDocument { error: ERR_AMMF });
        }

        if key.authority() != did.authority() {
            return Err(Error::InvalidDocument { error: ERR_AMIM });
        }

        if let Some(proof) = document.properties_mut().remove("proof") {
            let proof: Signature = Signature::from_json_value(proof)?;
            let root: Document<Properties> = document.try_map(|old| old.serde_into())?;

            Ok(Self(VerifiableDocument::with_proof(root, proof)))
        } else {
            let root: Document<Properties> = document.try_map(|old| old.serde_into())?;

            Ok(Self(VerifiableDocument::new(root)))
        }
    }

    /// Returns the DID document `id`.
    pub fn id(&self) -> &IotaDID {
        // SAFETY: We checked the validity of the DID Document ID in the
        // IotaDocument constructors; we don't provide mutable references so
        // the value cannot change with typical "safe" Rust.
        unsafe { IotaDID::new_unchecked_ref(self.0.id()) }
    }

    /// Returns the Tangle message id of the published DID document, if any.
    pub fn message_id(&self) -> Option<&str> {
        self.0.properties().message_id.as_deref()
    }

    // Sets the Tangle message id the published DID document.
    pub fn set_message_id<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.0.properties_mut().message_id = Some(value.into());
    }

    /// Returns the Tangle message id of the previous DID document, if any.
    pub fn prev_msg(&self) -> Option<&str> {
        self.0.properties().prev_msg.as_deref()
    }

    /// Sets the Tangle message id the previous DID document.
    pub fn set_prev_msg<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.0.properties_mut().prev_msg = Some(value.into());
    }

    /// Returns the Tangle address of the DID document diff chain, if any.
    pub fn diff_chain(&self) -> Option<&str> {
        self.0.properties().diff_chain.as_deref()
    }

    /// Sets the Tangle address_hash of the DID document diff chain.
    pub fn set_diff_chain<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.0.properties_mut().diff_chain = Some(value.into());
    }

    /// Returns a reference to the custom `IotaDocument` properties.
    pub fn properties(&self) -> &Object {
        &self.0.properties().properties
    }

    /// Returns a mutable reference to the custom `IotaDocument` properties.
    pub fn properties_mut(&mut self) -> &mut Object {
        &mut self.0.properties_mut().properties
    }

    /// Returns the timestamp of when the DID document was created.
    pub fn created(&self) -> Timestamp {
        self.0.properties().created
    }

    /// Sets the timestamp of when the DID document was created.
    pub fn set_created(&mut self, value: Timestamp) {
        self.0.properties_mut().created = value;
    }

    /// Sets the DID document "created" timestamp to `Timestamp::now`.
    pub fn set_created_now(&mut self) {
        self.set_created(Timestamp::now());
    }

    /// Returns the timestamp of the last DID document update.
    pub fn updated(&self) -> Timestamp {
        self.0.properties().updated
    }

    /// Sets the timestamp of the last DID document update.
    pub fn set_updated(&mut self, value: Timestamp) {
        self.0.properties_mut().updated = value;
    }

    /// Sets the DID document "updated" timestamp to `Timestamp::now`.
    pub fn set_updated_now(&mut self) {
        self.set_updated(Timestamp::now());
    }

    /// Returns the default authentication method of the DID document.
    pub fn authentication(&self) -> MethodWrap {
        self.resolve(AUTH_QUERY).unwrap()
    }

    /// Returns the key bytes of the default DID document authentication method.
    pub fn authentication_bytes(&self) -> Result<Vec<u8>> {
        self.try_resolve_bytes(AUTH_QUERY).map_err(Into::into)
    }

    /// Returns the method type of the default DID document authentication method.
    pub fn authentication_type(&self) -> MethodType {
        self.authentication().key_type()
    }

    /// Returns a reference to the `VerifiableDocument`.
    pub fn as_document(&self) -> &VerifiableDocument<Properties> {
        &self.0
    }

    /// Returns a mutable reference to the `VerifiableDocument`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that modifications
    /// made to the `VerifiableDocument` maintain a valid `IotaDocument`.
    ///
    /// If this constraint is violated, it may cause issues with future uses of
    /// the `IotaDocument`.
    pub unsafe fn as_document_mut(&mut self) -> &mut VerifiableDocument<Properties> {
        &mut self.0
    }

    pub async fn publish(&mut self) -> Result<()> {
        let network: Network = Network::from_str(self.id().network());

        let client: Client = ClientBuilder::new()
            .node(network.node_url().as_str())
            .network(network)
            .build()?;

        self.publish_with_client(&client).await
    }

    pub async fn publish_with_client(&mut self, client: &Client) -> Result<()> {
        let transaction: _ = client.publish_document(&*self).await?;
        let message_id: String = client.transaction_hash(&transaction);

        self.set_message_id(message_id);

        Ok(())
    }

    /// Signs the DID document with the default authentication method.
    ///
    /// # Errors
    ///
    /// Fails if an unsupported verification method is used, document
    /// serialization fails, or the signature operation fails.
    pub fn sign(&mut self, secret: &SecretKey) -> Result<()> {
        match self.authentication_type() {
            MethodType::Ed25519VerificationKey2018 => {
                let mut opts: SignatureOptions = self.0.resolve_options(AUTH_QUERY)?;
                opts.created = Some(Timestamp::now().to_string());
                self.0.sign(JcsEd25519Signature2020, opts, secret.as_ref())?;
            }
            _ => {
                return Err(Error::InvalidDocument { error: ERR_AMNS });
            }
        }

        Ok(())
    }

    /// Verifies the signature of the DID document.
    ///
    /// Note: It is assumed that the signature was created using the default
    /// authentication method.
    ///
    /// # Errors
    ///
    /// Fails if an unsupported verification method is used, document
    /// serialization fails, or the verification operation fails.
    pub fn verify(&self) -> Result<()> {
        match self.authentication_type() {
            MethodType::Ed25519VerificationKey2018 => {
                self.0.verify(JcsEd25519Signature2020)?;
            }
            _ => {
                return Err(Error::InvalidDocument { error: ERR_AMNS });
            }
        }

        Ok(())
    }

    /// Signs the provided data with the default authentication method.
    ///
    /// # Errors
    ///
    /// Fails if an unsupported verification method is used, document
    /// serialization fails, or the signature operation fails.
    pub fn sign_data<T>(&self, data: &mut T, secret: &SecretKey) -> Result<()>
    where
        T: Serialize + SetSignature,
    {
        match self.authentication_type() {
            MethodType::Ed25519VerificationKey2018 => {
                let mut opts: SignatureOptions = self.0.resolve_options(AUTH_QUERY)?;
                opts.created = Some(Timestamp::now().to_string());
                self.0.sign_data(data, JcsEd25519Signature2020, opts, secret.as_ref())?;
            }
            _ => {
                return Err(Error::InvalidDocument { error: ERR_AMNS });
            }
        }

        Ok(())
    }

    /// Verfies the signature of the provided data.
    ///
    /// Note: It is assumed that the signature was created using the default
    /// authentication method.
    ///
    /// # Errors
    ///
    /// Fails if an unsupported verification method is used, document
    /// serialization fails, or the verification operation fails.
    pub fn verify_data<T>(&self, data: &T) -> Result<()>
    where
        T: Serialize + TrySignature,
    {
        match self.authentication_type() {
            MethodType::Ed25519VerificationKey2018 => {
                self.0.verify_data(data, JcsEd25519Signature2020)?;
            }
            _ => {
                return Err(Error::InvalidDocument { error: ERR_AMNS });
            }
        }

        Ok(())
    }

    /// Creates a `DIDDiff` representing the changes between `self` and `other`.
    ///
    /// The returned `DIDDiff` will have a digital signature created using the
    /// default authentication method and `secret`.
    ///
    /// # Errors
    ///
    /// Fails if the diff operation or signature operation fails.
    pub fn diff(&self, other: &Self, secret: &SecretKey, prev_msg: String) -> Result<DIDDiff> {
        let mut diff: DIDDiff = DIDDiff::new(self, other, prev_msg)?;

        self.sign_data(&mut diff, secret)?;

        Ok(diff)
    }

    /// Verifies a `DIDDiff` signature and merges the changes into `self`.
    ///
    /// If merging fails `self` remains unmodified, otherwise `self` represents
    /// the merged document state.
    ///
    /// # Errors
    ///
    /// Fails if the merge operation or signature operation fails.
    pub fn merge(&mut self, diff: &DIDDiff) -> Result<()> {
        self.verify_data(diff)?;

        let this: Document = self.serde_into()?;
        let data: DiffDocument = DiffDocument::from_json(&diff.diff)?;

        *self = Diff::merge(&this, data)?.serde_into()?;

        Ok(())
    }

    /// Verifies the `DIDDiff` proof using the default authentication method.
    ///
    /// # Errors
    ///
    /// Fails if the signature operation fails.
    pub fn verify_diff(&self, diff: &DIDDiff) -> Result<()> {
        self.verify_data(diff)
    }

    /// Returns the Tangle address of the DID document auth chain as a
    /// tryte-encoded String.
    pub fn auth_address_hash(&self) -> String {
        self.id().address_hash()
    }

    /// Returns the Tangle address of the DID document auth chain.
    pub fn auth_address(&self) -> Result<Address> {
        self.id().address()
    }
}

impl Display for IotaDocument {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Debug for IotaDocument {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Deref for IotaDocument {
    type Target = VerifiableDocument<Properties>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<VerifiableDocument<Properties>> for IotaDocument {
    fn eq(&self, other: &VerifiableDocument<Properties>) -> bool {
        &self.0 == other
    }
}

impl From<IotaDocument> for VerifiableDocument<Properties> {
    fn from(other: IotaDocument) -> Self {
        other.0
    }
}

impl TryFrom<Document> for IotaDocument {
    type Error = Error;

    fn try_from(other: Document) -> Result<Self, Self::Error> {
        Self::try_from_document(other)
    }
}

impl SignatureDocument for IotaDocument {
    fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
        SignatureDocument::resolve_method(&self.0, method)
    }

    fn try_signature(&self) -> Option<&Signature> {
        SignatureDocument::try_signature(&self.0)
    }

    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        SignatureDocument::try_signature_mut(&mut self.0)
    }

    fn set_signature(&mut self, signature: Signature) {
        SignatureDocument::set_signature(&mut self.0, signature)
    }
}
