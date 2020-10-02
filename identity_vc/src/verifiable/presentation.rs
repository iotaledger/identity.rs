use identity_common::{Object, OneOrMany};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::presentation::Presentation;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiablePresentation {
    #[serde(flatten)]
    presentation: Presentation,
    proof: OneOrMany<Object>,
}

impl VerifiablePresentation {
    pub fn new(presentation: Presentation, proof: impl Into<OneOrMany<Object>>) -> Self {
        Self {
            presentation,
            proof: proof.into(),
        }
    }

    pub fn presentation(&self) -> &Presentation {
        &self.presentation
    }

    pub fn presentation_mut(&mut self) -> &mut Presentation {
        &mut self.presentation
    }

    pub fn proof(&self) -> &OneOrMany<Object> {
        &self.proof
    }

    pub fn proof_mut(&mut self) -> &mut OneOrMany<Object> {
        &mut self.proof
    }
}

impl Deref for VerifiablePresentation {
    type Target = Presentation;

    fn deref(&self) -> &Self::Target {
        &self.presentation
    }
}
