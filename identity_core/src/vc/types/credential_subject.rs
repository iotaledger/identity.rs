use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    common::{Object, Uri},
    error::Error,
};

/// An entity who is the target of a set of claims.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#credential-subject)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CredentialSubject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uri>,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for CredentialSubject {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.take_object_id().map(Into::into),
            properties: other,
        })
    }
}
