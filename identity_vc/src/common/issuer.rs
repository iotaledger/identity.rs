use identity_core::common::Object;

use crate::common::URI;

/// TODO:
///   - Deserialize single URI into object-style layout
///   - Replace Enum with plain struct
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Issuer {
    URI(URI),
    OBJ {
        id: URI,
        #[serde(flatten)]
        object: Object,
    },
}

impl Issuer {
    pub fn uri(&self) -> &URI {
        match self {
            Self::URI(uri) => uri,
            Self::OBJ { id, .. } => id,
        }
    }
}

impl<T> From<T> for Issuer
where
    T: Into<URI>,
{
    fn from(other: T) -> Self {
        Self::URI(other.into())
    }
}
