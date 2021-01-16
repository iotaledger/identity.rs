use crate::{
    common::{Context, Object, Timestamp, Url, Value},
    error::Result,
    vc::{
        Credential, CredentialSchema, CredentialStatus, CredentialSubject, Evidence, Issuer, RefreshService, TermsOfUse,
    },
};

/// A `CredentialBuilder` is used to create a customized `Credential`.
#[derive(Clone, Debug)]
pub struct CredentialBuilder<T = Object> {
    pub(crate) context: Vec<Context>,
    pub(crate) id: Option<Url>,
    pub(crate) types: Vec<String>,
    pub(crate) credential_subject: Vec<CredentialSubject>,
    pub(crate) issuer: Option<Issuer>,
    pub(crate) issuance_date: Option<Timestamp>,
    pub(crate) expiration_date: Option<Timestamp>,
    pub(crate) credential_status: Vec<CredentialStatus>,
    pub(crate) credential_schema: Vec<CredentialSchema>,
    pub(crate) refresh_service: Vec<RefreshService>,
    pub(crate) terms_of_use: Vec<TermsOfUse>,
    pub(crate) evidence: Vec<Evidence>,
    pub(crate) non_transferable: Option<bool>,
    pub(crate) properties: T,
}

impl<T> CredentialBuilder<T> {
    /// Creates a new `CredentialBuilder`.
    pub fn new(properties: T) -> Self {
        Self {
            context: vec![Credential::<T>::base_context().clone()],
            id: None,
            types: vec![Credential::<T>::base_type().into()],
            credential_subject: Vec::new(),
            issuer: None,
            issuance_date: None,
            expiration_date: None,
            credential_status: Vec::new(),
            credential_schema: Vec::new(),
            refresh_service: Vec::new(),
            terms_of_use: Vec::new(),
            evidence: Vec::new(),
            non_transferable: None,
            properties,
        }
    }

    /// Adds a value to the `Credential` context set.
    #[must_use]
    pub fn context(mut self, value: impl Into<Context>) -> Self {
        self.context.push(value.into());
        self
    }

    /// Sets the value of the `Credential` `id`.
    #[must_use]
    pub fn id(mut self, value: Url) -> Self {
        self.id = Some(value);
        self
    }

    /// Adds a value to the `Credential` type set.
    #[must_use]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.types.push(value.into());
        self
    }

    /// Adds a value to the `credentialSubject` set.
    #[must_use]
    pub fn credential_subject(mut self, value: CredentialSubject) -> Self {
        self.credential_subject.push(value);
        self
    }

    /// Sets the value of the `Credential` `issuer`.
    #[must_use]
    pub fn issuer(mut self, value: impl Into<Issuer>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    /// Sets the value of the `Credential` `issuanceDate`.
    #[must_use]
    pub fn issuance_date(mut self, value: Timestamp) -> Self {
        self.issuance_date = Some(value);
        self
    }

    /// Sets the value of the `Credential` `expirationDate`.
    #[must_use]
    pub fn expiration_date(mut self, value: Timestamp) -> Self {
        self.expiration_date = Some(value);
        self
    }

    /// Adds a value to the `credentialStatus` set.
    #[must_use]
    pub fn credential_status(mut self, value: CredentialStatus) -> Self {
        self.credential_status.push(value);
        self
    }

    /// Adds a value to the `credentialSchema` set.
    #[must_use]
    pub fn credential_schema(mut self, value: CredentialSchema) -> Self {
        self.credential_schema.push(value);
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

    /// Adds a value to the `evidence` set.
    #[must_use]
    pub fn evidence(mut self, value: Evidence) -> Self {
        self.evidence.push(value);
        self
    }

    /// Sets the value of the `Credential` `nonTransferable` property.
    #[must_use]
    pub fn non_transferable(mut self, value: bool) -> Self {
        self.non_transferable = Some(value);
        self
    }

    /// Returns a new `Credential` based on the `CredentialBuilder` configuration.
    pub fn build(self) -> Result<Credential<T>> {
        Credential::from_builder(self)
    }
}

impl CredentialBuilder {
    /// Adds a new custom property to the `Credential`.
    #[must_use]
    pub fn property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Adds a series of custom properties to the `Credential`.
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

impl<T> Default for CredentialBuilder<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::{
        common::{Object, Timestamp, Url},
        convert::FromJson as _,
        vc::{Credential as Credential_, CredentialBuilder, CredentialSubject},
    };

    type Credential = Credential_<Object>;

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
    fn test_credential_builder_valid() {
        let credential: Credential = CredentialBuilder::default()
            .context(Url::parse("https://www.w3.org/2018/credentials/examples/v1").unwrap())
            .id(Url::parse("http://example.edu/credentials/3732").unwrap())
            .type_("UniversityDegreeCredential")
            .credential_subject(subject())
            .issuer(issuer())
            .issuance_date(Timestamp::parse("2010-01-01T00:00:00Z").unwrap())
            .build()
            .unwrap();

        assert_eq!(credential.context.len(), 2);
        assert_eq!(credential.context.get(0).unwrap(), Credential::base_context());
        assert_eq!(credential.context.get(1).unwrap(), "https://www.w3.org/2018/credentials/examples/v1");
        assert_eq!(credential.id.unwrap(), "http://example.edu/credentials/3732");
        assert_eq!(credential.types.len(), 2);
        assert_eq!(credential.types.get(0).unwrap(), Credential::base_type());
        assert_eq!(credential.types.get(1).unwrap(), "UniversityDegreeCredential");
        assert_eq!(credential.credential_subject.len(), 1);
        assert_eq!(credential.issuer.url(), "did:example:issuer");
        assert_eq!(credential.issuance_date.to_string(), "2010-01-01T00:00:00Z");
        assert_eq!(credential.credential_subject.get(0).unwrap().id.as_ref().unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
        assert_eq!(credential.credential_subject.get(0).unwrap().properties["degree"]["type"], "BachelorDegree");
        assert_eq!(credential.credential_subject.get(0).unwrap().properties["degree"]["name"], "Bachelor of Science and Arts");
    }

    #[test]
    #[should_panic = "Missing Subject"]
    fn test_builder_missing_subjects() {
        let _: Credential = CredentialBuilder::default().issuer(issuer()).build().unwrap();
    }

    #[test]
    #[should_panic = "Missing Credential Issuer"]
    fn test_builder_missing_issuer() {
        let _: Credential = CredentialBuilder::default()
            .credential_subject(subject())
            .build()
            .unwrap();
    }
}
