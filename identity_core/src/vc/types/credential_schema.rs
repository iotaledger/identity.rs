use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    common::{Object, Uri},
    error::Error,
};

/// Information used to validate the structure of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#data-schemas)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CredentialSchema {
    pub id: Uri,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for CredentialSchema {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.try_take_object_id()?.into(),
            type_: other.try_take_object_type()?,
            properties: other,
        })
    }
}
