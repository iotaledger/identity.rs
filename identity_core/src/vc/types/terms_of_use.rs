use core::convert::TryFrom;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{Object, OneOrMany, Uri},
    error::Error,
};

/// Information used to express obligations, prohibitions, and permissions about
/// a `Credential` or `Presentation`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#terms-of-use)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, Builder)]
pub struct TermsOfUse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<Uri>,
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    #[builder(default, setter(into))]
    pub properties: Object,
}

impl TryFrom<Object> for TermsOfUse {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.take_object_id().map(Into::into),
            types: other.try_take_object_types()?,
            properties: other,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "`types` must be initialized"]
    fn test_builder_missing_types() {
        TermsOfUseBuilder::default().id("my-id").build().unwrap();
    }
}
