use identity_common::{Object, OneOrMany, Uri};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    common::{take_object_id, try_take_object_types},
    error::Error,
};

/// Information used to express obligations, prohibitions, and permissions about
/// a `Credential` or `Presentation`.
///
/// Ref: https://www.w3.org/TR/vc-data-model/#terms-of-use
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TermsOfUse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uri>,
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    #[serde(flatten)]
    pub properties: Object,
}

impl TryFrom<Object> for TermsOfUse {
    type Error = Error;

    fn try_from(mut other: Object) -> Result<Self, Self::Error> {
        let mut this: Self = Default::default();

        this.id = take_object_id(&mut other).map(Into::into);
        this.types = try_take_object_types("TermsOfUse", &mut other)?;
        this.properties = other;

        Ok(this)
    }
}
