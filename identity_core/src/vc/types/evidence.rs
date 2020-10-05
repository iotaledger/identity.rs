use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    common::{Object, OneOrMany},
    error::Error,
};

/// Information used to increase confidence in the claims of a `Credential`
///
/// Ref: https://www.w3.org/TR/vc-data-model/#evidence
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for Evidence {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.take_object_id(),
            types: other.try_take_object_types()?,
            properties: other,
        })
    }
}
