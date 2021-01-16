use core::{
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{Object, OneOrMany},
    convert::ToJson as _,
    did_doc::{SetSignature, Signature, TrySignature, TrySignatureMut},
    vc::Credential,
};

/// A `VerifiableCredential` represents a `Credential` with an associated
/// digital proof.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiableCredential<T = Object> {
    #[serde(flatten)]
    credential: Credential<T>,
    #[serde(skip_serializing_if = "OneOrMany::is_empty")]
    proof: OneOrMany<Signature>,
}

impl<T> VerifiableCredential<T> {
    /// Creates a new `VerifiableCredential`.
    pub fn new<P>(credential: Credential<T>, proof: P) -> Self
    where
        P: Into<OneOrMany<Signature>>,
    {
        Self {
            credential,
            proof: proof.into(),
        }
    }

    /// Returns a reference to the `VerifiableCredential` proof.
    pub fn proof(&self) -> &OneOrMany<Signature> {
        &self.proof
    }

    /// Returns a mutable reference to the `VerifiableCredential` proof.
    pub fn proof_mut(&mut self) -> &mut OneOrMany<Signature> {
        &mut self.proof
    }
}

impl<T> Deref for VerifiableCredential<T> {
    type Target = Credential<T>;

    fn deref(&self) -> &Self::Target {
        &self.credential
    }
}

impl<T> Display for VerifiableCredential<T>
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

impl<T> DerefMut for VerifiableCredential<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.credential
    }
}

impl<T> TrySignature for VerifiableCredential<T> {
    fn try_signature(&self) -> Option<&Signature> {
        self.proof.get(0)
    }
}

impl<T> TrySignatureMut for VerifiableCredential<T> {
    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.get_mut(0)
    }
}

impl<T> SetSignature for VerifiableCredential<T> {
    fn set_signature(&mut self, value: Signature) {
        self.proof = OneOrMany::One(value);
    }
}
