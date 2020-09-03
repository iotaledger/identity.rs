use identity_core::common::{Object, Uri};
use std::convert::TryFrom;

use crate::{common::take_object_id, error::Error};

/// An entity who is the target of a set of claims.
///
/// Ref: https://www.w3.org/TR/vc-data-model/#credential-subject
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
        let mut this: Self = Default::default();

        this.id = take_object_id(&mut other).map(Into::into);
        this.properties = other;

        Ok(this)
    }
}
