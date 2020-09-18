use identity_core::common::{Object, Uri};
use serde::{Deserialize, Serialize};

/// TODO:
///   - Deserialize single Uri into object-style layout
///   - Replace Enum with plain struct
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
