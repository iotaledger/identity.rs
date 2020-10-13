use derive_builder::Builder;
use identity_diff::{self as diff, Diff};
use serde::{Deserialize, Serialize};

use crate::{
    did::{KeyData, KeyType, DID},
    utils::HasId,
};

/// Public key struct.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Builder)]
#[builder(pattern = "owned")]
pub struct PublicKey {
    #[builder(try_setter)]
    id: DID,
    #[builder(try_setter)]
    controller: DID,
    #[serde(rename = "type")]
    #[builder(try_setter)]
    key_type: KeyType,
    #[serde(flatten)]
    key_data: KeyData,
}

impl PublicKey {
    pub fn default_key_type() -> KeyType {
        KeyType::Ed25519VerificationKey2018
    }

    pub fn default_key_data() -> KeyData {
        KeyData::PublicKeyHex("".into())
    }

    pub fn id(&self) -> &DID {
        &self.id
    }

    pub fn controller(&self) -> &DID {
        &self.controller
    }

    pub fn key_type(&self) -> KeyType {
        self.key_type
    }

    pub fn key_data(&self) -> &KeyData {
        &self.key_data
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        Self {
            id: Default::default(),
            controller: Default::default(),
            key_type: Self::default_key_type(),
            key_data: Self::default_key_data(),
        }
    }
}

impl HasId for PublicKey {
    type Id = DID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "PublicKey", into = "PublicKey")]
pub struct DiffPublicKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<<DID as Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controller: Option<<DID as Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_type: Option<<KeyType as Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_data: Option<<KeyData as Diff>::Type>,
}

impl Diff for PublicKey {
    type Type = DiffPublicKey;

    fn merge(&self, diff: Self::Type) -> Result<Self, diff::Error> {
        Ok(Self {
            id: diff
                .id
                .map(|value| self.id.merge(value))
                .transpose()?
                .unwrap_or_else(|| self.id.clone()),
            controller: diff
                .controller
                .map(|value| self.controller.merge(value))
                .transpose()?
                .unwrap_or_else(|| self.controller.clone()),
            key_type: diff
                .key_type
                .map(|value| self.key_type.merge(value))
                .transpose()?
                .unwrap_or_else(|| self.key_type),
            key_data: diff
                .key_data
                .map(|value| self.key_data.merge(value))
                .transpose()?
                .unwrap_or_else(|| self.key_data.clone()),
        })
    }

    fn diff(&self, other: &Self) -> Result<Self::Type, diff::Error> {
        Ok(DiffPublicKey {
            id: if self.id == other.id {
                None
            } else {
                Some(self.id.diff(&other.id)?)
            },
            controller: if self.controller == other.controller {
                None
            } else {
                Some(self.controller.diff(&other.controller)?)
            },
            key_type: if self.key_type == other.key_type {
                None
            } else {
                Some(self.key_type.diff(&other.key_type)?)
            },
            key_data: if self.key_data == other.key_data {
                None
            } else {
                Some(self.key_data.diff(&other.key_data)?)
            },
        })
    }

    fn from_diff(diff: Self::Type) -> Result<Self, diff::Error> {
        Ok(Self {
            id: diff.id.map(<DID>::from_diff).transpose()?.unwrap_or_default(),
            controller: diff.controller.map(<DID>::from_diff).transpose()?.unwrap_or_default(),
            key_type: diff
                .key_type
                .map(<KeyType>::from_diff)
                .transpose()?
                .unwrap_or_else(Self::default_key_type),
            key_data: diff
                .key_data
                .map(<KeyData>::from_diff)
                .transpose()?
                .unwrap_or_else(Self::default_key_data),
        })
    }

    fn into_diff(self) -> Result<Self::Type, diff::Error> {
        Ok(DiffPublicKey {
            id: Some(self.id.into_diff()?),
            controller: Some(self.controller.into_diff()?),
            key_type: Some(self.key_type.into_diff()?),
            key_data: Some(self.key_data.into_diff()?),
        })
    }
}

impl From<PublicKey> for DiffPublicKey {
    fn from(other: PublicKey) -> Self {
        other.into_diff().expect("Unable to convert to diff")
    }
}

impl From<DiffPublicKey> for PublicKey {
    fn from(other: DiffPublicKey) -> Self {
        Self::from_diff(other).expect("Unable to convert from diff")
    }
}
