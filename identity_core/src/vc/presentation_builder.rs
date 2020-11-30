use crate::{
    common::{Context, Object, Url, Value},
    error::Result,
    vc::{Presentation, RefreshService, TermsOfUse, VerifiableCredential},
};

/// A `PresentationBuilder` is used to create a customized `Presentation`.
#[derive(Clone, Debug)]
pub struct PresentationBuilder<T = Object, U = Object> {
    pub(crate) context: Vec<Context>,
    pub(crate) id: Option<Url>,
    pub(crate) types: Vec<String>,
    pub(crate) verifiable_credential: Vec<VerifiableCredential<U>>,
    pub(crate) holder: Option<Url>,
    pub(crate) refresh_service: Vec<RefreshService>,
    pub(crate) terms_of_use: Vec<TermsOfUse>,
    pub(crate) properties: T,
}

impl<T, U> PresentationBuilder<T, U> {
    /// Creates a new `PresentationBuilder`.
    pub fn new(properties: T) -> Self {
        Self {
            context: vec![Presentation::<T, U>::base_context().clone()],
            id: None,
            types: vec![Presentation::<T, U>::base_type().into()],
            verifiable_credential: Vec::new(),
            holder: None,
            refresh_service: Vec::new(),
            terms_of_use: Vec::new(),
            properties,
        }
    }

    /// Adds a value to the `Presentation` context set.
    #[must_use]
    pub fn context(mut self, value: impl Into<Context>) -> Self {
        self.context.push(value.into());
        self
    }

    /// Sets the value of the `Presentation` `id`.
    #[must_use]
    pub fn id(mut self, value: Url) -> Self {
        self.id = Some(value);
        self
    }

    /// Adds a value to the `Presentation` type set.
    #[must_use]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.types.push(value.into());
        self
    }

    /// Adds a value to the `verifiableCredential` set.
    #[must_use]
    pub fn verifiable_credential(mut self, value: VerifiableCredential<U>) -> Self {
        self.verifiable_credential.push(value);
        self
    }

    /// Sets the value of the `Credential` `holder`.
    #[must_use]
    pub fn holder(mut self, value: Url) -> Self {
        self.holder = Some(value);
        self
    }

    /// Adds a value to the `refreshService` set.
    #[must_use]
    pub fn refresh_service(mut self, value: RefreshService) -> Self {
        self.refresh_service.push(value);
        self
    }

    /// Adds a value to the `termsOfUse` set.
    #[must_use]
    pub fn terms_of_use(mut self, value: TermsOfUse) -> Self {
        self.terms_of_use.push(value);
        self
    }

    /// Returns a new `Presentation` based on the `PresentationBuilder` configuration.
    pub fn build(self) -> Result<Presentation<T, U>> {
        Presentation::from_builder(self)
    }
}

impl<T> PresentationBuilder<Object, T> {
    /// Adds a new custom property to the `Presentation`.
    #[must_use]
    pub fn property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Adds a series of custom properties to the `Presentation`.
    #[must_use]
    pub fn properties<K, V, I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<Value>,
    {
        self.properties
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl<T, U> Default for PresentationBuilder<T, U>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use did_doc::{Document, DocumentBuilder, Method, MethodBuilder, MethodData, MethodType};
    use did_url::DID;
    use serde_json::{json, Value};

    use crate::{
        common::{Object, Url},
        convert::FromJson as _,
        crypto::KeyPair,
        proof::JcsEd25519Signature2020,
        utils::encode_b58,
        vc::{
            Credential as Credential_, CredentialBuilder, CredentialSubject, Presentation as Presentation_,
            PresentationBuilder, VerifiableCredential,
        },
    };

    type Credential = Credential_<Object>;
    type Presentation = Presentation_<Object, Object>;

    fn subject() -> CredentialSubject {
        let json: Value = json!({
            "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
            "degree": {
                "type": "BachelorDegree",
                "name": "Bachelor of Science and Arts"
            }
        });

        CredentialSubject::from_json_value(json).unwrap()
    }

    fn issuer() -> Url {
        Url::parse("did:example:issuer").unwrap()
    }

    #[test]
    #[rustfmt::skip]
    fn test_presentation_builder_valid() {
        let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();
        let controller: DID = "did:example:1234".parse().unwrap();

        let method: Method = MethodBuilder::default()
            .id(controller.join("#key-1").unwrap())
            .controller(controller.clone())
            .key_type(MethodType::Ed25519VerificationKey2018)
            .key_data(MethodData::PublicKeyBase58(encode_b58(keypair.public())))
            .build()
            .unwrap();

        let document: Document = DocumentBuilder::default()
            .id(controller)
            .verification_method(method)
            .build()
            .unwrap();

        let credential: VerifiableCredential = CredentialBuilder::default()
            .type_("ExampleCredential")
            .credential_subject(subject())
            .issuer(issuer())
            .build()
            .unwrap()
            .sign(&document, 0, keypair.secret())
            .unwrap();

        let presentation: Presentation = PresentationBuilder::default()
            .type_("ExamplePresentation")
            .verifiable_credential(credential)
            .build()
            .unwrap();

        assert_eq!(presentation.context.len(), 1);
        assert_eq!(presentation.context.get(0).unwrap(), Presentation::base_context());
        assert_eq!(presentation.types.len(), 2);
        assert_eq!(presentation.types.get(0).unwrap(), Presentation::base_type());
        assert_eq!(presentation.types.get(1).unwrap(), "ExamplePresentation");
        assert_eq!(presentation.verifiable_credential.len(), 1);
        assert_eq!(presentation.verifiable_credential.get(0).unwrap().types.get(0).unwrap(), Credential::base_type());
        assert_eq!(presentation.verifiable_credential.get(0).unwrap().types.get(1).unwrap(), "ExampleCredential");
    }
}
