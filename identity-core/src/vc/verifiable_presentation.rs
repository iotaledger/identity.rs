use core::{
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{Object, OneOrMany},
    convert::ToJson as _,
    did_doc::{SetSignature, Signature, TrySignature, TrySignatureMut},
    vc::Presentation,
};

/// A `VerifiablePresentation` represents a `Presentation` with an associated
/// digital proof.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiablePresentation<T = Object, U = Object> {
    #[serde(flatten)]
    presentation: Presentation<T, U>,
    #[serde(skip_serializing_if = "OneOrMany::is_empty")]
    proof: OneOrMany<Signature>,
}

impl<T, U> VerifiablePresentation<T, U> {
    /// Creates a new `VerifiablePresentation`.
    pub fn new<P>(presentation: Presentation<T, U>, proof: P) -> Self
    where
        P: Into<OneOrMany<Signature>>,
    {
        Self {
            presentation,
            proof: proof.into(),
        }
    }

    /// Returns a reference to the `VerifiablePresentation` proof.
    pub fn proof(&self) -> &OneOrMany<Signature> {
        &self.proof
    }

    /// Returns a mutable reference to the `VerifiablePresentation` proof.
    pub fn proof_mut(&mut self) -> &mut OneOrMany<Signature> {
        &mut self.proof
    }
}

impl<T, U> Deref for VerifiablePresentation<T, U> {
    type Target = Presentation<T, U>;

    fn deref(&self) -> &Self::Target {
        &self.presentation
    }
}

impl<T, U> DerefMut for VerifiablePresentation<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.presentation
    }
}

impl<T, U> Display for VerifiablePresentation<T, U>
where
    T: Serialize,
    U: Serialize,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
        } else {
            f.write_str(&self.to_json().map_err(|_| FmtError)?)
        }
    }
}

impl<T, U> TrySignature for VerifiablePresentation<T, U> {
    fn try_signature(&self) -> Option<&Signature> {
        self.proof.get(0)
    }
}

impl<T, U> TrySignatureMut for VerifiablePresentation<T, U> {
    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.get_mut(0)
    }
}

impl<T, U> SetSignature for VerifiablePresentation<T, U> {
    fn set_signature(&mut self, value: Signature) {
        self.proof = OneOrMany::One(value);
    }
}
