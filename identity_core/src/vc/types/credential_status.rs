use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    common::{Object, OneOrMany, Uri},
    error::Error,
};

/// Information used to determine the current status of a `Credential`.
///
/// Ref: https://www.w3.org/TR/vc-data-model/#status
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CredentialStatus {
    pub id: Uri,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for CredentialStatus {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.try_take_object_id()?.into(),
            types: other.try_take_object_types()?,
            properties: other,
        })
    }
}
