use serde::{Deserialize, Serialize};

use crate::common::{Object, Url};

/// An identifier representing the issuer of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#issuer)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Issuer {
    Url(Url),
    Obj {
        id: Url,
        #[serde(flatten)]
        object: Object,
    },
}

impl Issuer {
    pub fn url(&self) -> &Url {
        match self {
            Self::Url(url) => url,
            Self::Obj { id, .. } => id,
        }
    }
}

impl<T> From<T> for Issuer
where
    T: Into<Url>,
{
    fn from(other: T) -> Self {
        Self::Url(other.into())
    }
}
