use identity_core::{
    crypto::KeyPair,
    did_doc::{DocumentBuilder, Method, MethodBuilder, MethodData, MethodType, VerifiableDocument},
    proof::JcsEd25519Signature2020,
};

use crate::{
    did::{IotaDID, IotaDocument, Properties},
    error::Result,
};

#[derive(Clone, Debug)]
pub struct IotaDocumentBuilder {
    authentication_tag: String,
    authentication_type: MethodType,
    did_network: Option<String>,
    did_shard: Option<String>,
    immutable: bool,
}

impl IotaDocumentBuilder {
    pub fn new() -> Self {
        Self {
            authentication_tag: "authentication".into(),
            authentication_type: MethodType::Ed25519VerificationKey2018,
            did_network: None,
            did_shard: None,
            immutable: false,
        }
    }

    #[must_use]
    pub fn authentication_tag<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.authentication_tag = value.into();
        self
    }

    #[must_use]
    pub fn authentication_type(mut self, value: MethodType) -> Self {
        self.authentication_type = value;
        self
    }

    #[must_use]
    pub fn did_network<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.did_network = Some(value.into());
        self
    }

    #[must_use]
    pub fn did_shard<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.did_shard = Some(value.into());
        self
    }

    #[must_use]
    pub fn immutable(mut self, value: bool) -> Self {
        self.immutable = value;
        self
    }

    pub fn build(self) -> Result<(IotaDocument, KeyPair)> {
        let keypair: KeyPair = Self::default_keypair(self.authentication_type)?;
        let public: &[u8] = keypair.public().as_ref();

        let network: Option<&str> = self.did_network.as_deref();
        let shard: Option<&str> = self.did_shard.as_deref();

        let did: IotaDID = IotaDID::with_network_and_shard(public, network, shard)?;
        let key: IotaDID = did.join(format!("#{}", self.authentication_tag))?;

        let method: Method = MethodBuilder::default()
            .id(key.into())
            .controller(did.clone().into())
            .key_type(self.authentication_type)
            .key_data(MethodData::new_b58(public))
            .build()?;

        let properties: Properties = Properties {
            immutable: self.immutable,
            ..Properties::new()
        };

        let document: IotaDocument = DocumentBuilder::new(properties)
            .id(did.into())
            .authentication(method)
            .build()
            .map(VerifiableDocument::new)
            .map(Into::into)?;

        Ok((document, keypair))
    }

    fn default_keypair(method: MethodType) -> Result<KeyPair> {
        match method {
            MethodType::Ed25519VerificationKey2018 => Ok(JcsEd25519Signature2020::new_keypair()),
            _ => {
                todo!("Invalid Method Type")
            }
        }
    }
}

impl Default for IotaDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
