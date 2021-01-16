use core::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use did_doc::{Document, MethodQuery, MethodType, MethodWrap, MethodWriter, SignatureOptions};
use serde::Serialize;

use crate::{
    common::{Context, Object, OneOrMany, Timestamp, Url},
    convert::ToJson as _,
    crypto::SecretKey,
    error::{Error, Result},
    proof::JcsEd25519Signature2020,
    vc::{
        CredentialBuilder, CredentialSchema, CredentialStatus, CredentialSubject, Evidence, Issuer, RefreshService,
        TermsOfUse, VerifiableCredential,
    },
};

lazy_static! {
    static ref BASE_CONTEXT: Context = Context::Url(Url::parse("https://www.w3.org/2018/credentials/v1").unwrap());
}

/// A `Credential` represents a set of claims describing an entity.
///
/// `Credential`s can be signed with `Document`s to create `VerifiableCredential`s.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Credential<T = Object> {
    /// The JSON-LD context(s) applicable to the `Credential`.
    #[serde(rename = "@context")]
    pub context: OneOrMany<Context>,
    /// A unique `URI` referencing the subject of the `Credential`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Url>,
    /// One or more URIs defining the type of the `Credential`.
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    /// One or more `Object`s representing the `Credential` subject(s).
    #[serde(rename = "credentialSubject")]
    pub credential_subject: OneOrMany<CredentialSubject>,
    /// A reference to the issuer of the `Credential`.
    pub issuer: Issuer,
    /// A timestamp of when the `Credential` becomes valid.
    #[serde(rename = "issuanceDate")]
    pub issuance_date: Timestamp,
    /// A timestamp of when the `Credential` should no longer be considered valid.
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<Timestamp>,
    /// Information used to determine the current status of the `Credential`.
    #[serde(default, rename = "credentialStatus", skip_serializing_if = "OneOrMany::is_empty")]
    pub credential_status: OneOrMany<CredentialStatus>,
    /// Information used to assist in the enforcement of a specific `Credential` structure.
    #[serde(default, rename = "credentialSchema", skip_serializing_if = "OneOrMany::is_empty")]
    pub credential_schema: OneOrMany<CredentialSchema>,
    /// Service(s) used to refresh an expired `Credential`.
    #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
    pub refresh_service: OneOrMany<RefreshService>,
    /// Terms-of-use specified by the `Credential` issuer.
    #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
    pub terms_of_use: OneOrMany<TermsOfUse>,
    /// Human-readable evidence used to support the claims within the `Credential`.
    #[serde(default, skip_serializing_if = "OneOrMany::is_empty")]
    pub evidence: OneOrMany<Evidence>,
    /// Indicates that the `Credential` must only be contained within a
    /// `Presentation` with a proof issued from the `Credential` subject.
    #[serde(rename = "nonTransferable", skip_serializing_if = "Option::is_none")]
    pub non_transferable: Option<bool>,
    /// Miscellaneous properties.
    #[serde(flatten)]
    pub properties: T,
}

impl<T> Credential<T> {
    /// Returns the base JSON-LD context for `Credential`s.
    pub fn base_context() -> &'static Context {
        &*BASE_CONTEXT
    }

    /// Returns the base type for `Credential`s.
    pub const fn base_type() -> &'static str {
        "VerifiableCredential"
    }

    /// Creates a `CredentialBuilder` to configure a new `Credential`.
    ///
    /// This is the same as `CredentialBuilder::new()`.
    pub fn builder(properties: T) -> CredentialBuilder<T> {
        CredentialBuilder::new(properties)
    }

    /// Returns a new `Credential` based on the `CredentialBuilder` configuration.
    pub fn from_builder(builder: CredentialBuilder<T>) -> Result<Self> {
        let this: Self = Self {
            context: builder.context.into(),
            id: builder.id,
            types: builder.types.into(),
            credential_subject: builder.credential_subject.into(),
            issuer: builder
                .issuer
                .ok_or_else(|| Error::InvalidCredential("Missing Credential Issuer".into()))?,
            issuance_date: builder.issuance_date.unwrap_or_default(),
            expiration_date: builder.expiration_date,
            credential_status: builder.credential_status.into(),
            credential_schema: builder.credential_schema.into(),
            refresh_service: builder.refresh_service.into(),
            terms_of_use: builder.terms_of_use.into(),
            evidence: builder.evidence.into(),
            non_transferable: builder.non_transferable,
            properties: builder.properties,
        };

        this.check_structure()?;

        Ok(this)
    }

    /// Validates the semantic structure of the `Credential`.
    pub fn check_structure(&self) -> Result<()> {
        // Ensure the base context is present and in the correct location
        match self.context.get(0) {
            Some(context) if context == Self::base_context() => {}
            Some(_) | None => return Err(Error::InvalidCredential("Missing Base Context".into())),
        }

        // The set of types MUST contain the base type
        if !self.types.iter().any(|type_| type_ == Self::base_type()) {
            return Err(Error::InvalidCredential("Missing Base Type".into()));
        }

        // Credentials MUST have at least one subject
        if self.credential_subject.is_empty() {
            return Err(Error::InvalidCredential("Missing Subject".into()));
        }

        // Each subject is defined as one or more properties - no empty objects
        for subject in self.credential_subject.iter() {
            if subject.id.is_none() && subject.properties.is_empty() {
                return Err(Error::InvalidCredential("Invalid Subject".into()));
            }
        }

        Ok(())
    }

    pub fn sign<'a, Q, D1, D2, D3>(
        self,
        document: &Document<D1, D2, D3>,
        query: Q,
        secret: &SecretKey,
    ) -> Result<VerifiableCredential<T>>
    where
        T: Serialize,
        Q: Into<MethodQuery<'a>>,
    {
        let method: MethodWrap<'_, D2> = document.try_resolve(query)?;

        let options: SignatureOptions =
            SignatureOptions::with_purpose(method.id().to_string(), method.scope().as_str().to_string());

        let mut target: VerifiableCredential<T> = VerifiableCredential::new(self, Vec::new());
        let mut writer: MethodWriter<VerifiableCredential<T>, D2> = MethodWriter::new(&mut target, &*method);

        match method.key_type() {
            MethodType::Ed25519VerificationKey2018 => {
                writer.sign(JcsEd25519Signature2020, options, secret.as_ref())?;
            }
            _ => {
                todo!("return Err(\"Verification Method Not Supported\")")
            }
        }

        Ok(target)
    }
}

impl<T> Display for Credential<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
        } else {
            f.write_str(&self.to_json().map_err(|_| FmtError)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert::FromJson as _, vc::Credential};

    const JSON1: &str = include_str!("../../tests/fixtures/vc/credential-1.json");
    const JSON2: &str = include_str!("../../tests/fixtures/vc/credential-2.json");
    const JSON3: &str = include_str!("../../tests/fixtures/vc/credential-3.json");
    const JSON4: &str = include_str!("../../tests/fixtures/vc/credential-4.json");
    const JSON5: &str = include_str!("../../tests/fixtures/vc/credential-5.json");
    const JSON6: &str = include_str!("../../tests/fixtures/vc/credential-6.json");
    const JSON7: &str = include_str!("../../tests/fixtures/vc/credential-7.json");
    const JSON8: &str = include_str!("../../tests/fixtures/vc/credential-8.json");
    const JSON9: &str = include_str!("../../tests/fixtures/vc/credential-9.json");
    const JSON10: &str = include_str!("../../tests/fixtures/vc/credential-10.json");
    const JSON11: &str = include_str!("../../tests/fixtures/vc/credential-11.json");
    const JSON12: &str = include_str!("../../tests/fixtures/vc/credential-12.json");

    #[test]
    fn test_from_json() {
        let _credential: Credential = Credential::from_json(JSON1).unwrap();
        let _credential: Credential = Credential::from_json(JSON2).unwrap();
        let _credential: Credential = Credential::from_json(JSON3).unwrap();
        let _credential: Credential = Credential::from_json(JSON4).unwrap();
        let _credential: Credential = Credential::from_json(JSON5).unwrap();
        let _credential: Credential = Credential::from_json(JSON6).unwrap();
        let _credential: Credential = Credential::from_json(JSON7).unwrap();
        let _credential: Credential = Credential::from_json(JSON8).unwrap();
        let _credential: Credential = Credential::from_json(JSON9).unwrap();
        let _credential: Credential = Credential::from_json(JSON10).unwrap();
        let _credential: Credential = Credential::from_json(JSON11).unwrap();
        let _credential: Credential = Credential::from_json(JSON12).unwrap();
    }
}
