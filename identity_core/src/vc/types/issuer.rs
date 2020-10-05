use serde::{Deserialize, Serialize};

use crate::common::{Object, Uri};

/// An identifier representing the issuer of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#issuer)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Issuer {
    Uri(Uri),
    Obj {
        id: Uri,
        #[serde(flatten)]
        object: Object,
    },
}

impl Issuer {
    pub fn uri(&self) -> &Uri {
        match self {
            Self::Uri(uri) => uri,
            Self::Obj { id, .. } => id,
        }
    }
}

impl<T> From<T> for Issuer
where
    T: Into<Uri>,
{
    fn from(other: T) -> Self {
        Self::Uri(other.into())
    }
}
