use crate::did::{Param, DID};
use identity_diff::Diff;

use serde::{Deserialize as DDeserialize, Serialize as DSerialize};

#[derive(Debug, PartialEq, Default, Clone, DDeserialize, DSerialize)]
#[serde(from = "DID", into = "DID")]
pub struct DiffDID {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub method_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_segments: Option<<Vec<String> as identity_diff::Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_segments: Option<<Option<Vec<String>> as identity_diff::Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<<Option<Vec<Param>> as identity_diff::Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment: Option<<Option<String> as identity_diff::Diff>::Type>,
}

#[derive(Debug, PartialEq, Default, Clone, DSerialize, DDeserialize)]
#[serde(from = "Param", into = "Param")]
pub struct DiffParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<String as identity_diff::Diff>::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<<Option<String> as identity_diff::Diff>::Type>,
}

impl Diff for DID {
    type Type = DiffDID;

    fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
        let method_name = if self.method_name == other.method_name {
            self.method_name.clone()
        } else {
            other.method_name.clone()
        };

        Ok(DiffDID {
            method_name,
            id_segments: if self.id_segments == other.id_segments {
                None
            } else {
                Some(self.id_segments.diff(&other.id_segments)?)
            },
            path_segments: if self.path_segments == other.path_segments || other.path_segments == None {
                None
            } else {
                Some(self.path_segments.diff(&other.path_segments)?)
            },
            query: if self.query == other.query || other.query == None {
                None
            } else {
                Some(self.query.diff(&other.query)?)
            },
            fragment: if self.fragment == other.fragment || other.fragment == None {
                None
            } else {
                Some(self.fragment.diff(&other.fragment)?)
            },
        })
    }

    fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
        Ok(Self {
            method_name: if self.method_name == diff.method_name {
                self.method_name.clone()
            } else {
                diff.method_name
            },
            id_segments: if let Some(d) = diff.id_segments {
                self.id_segments.merge(d)?
            } else {
                self.id_segments.clone()
            },
            path_segments: if let Some(d) = diff.path_segments {
                self.path_segments.merge(d)?
            } else {
                self.path_segments.clone()
            },
            query: if let Some(d) = diff.query {
                self.query.merge(d)?
            } else {
                self.query.clone()
            },
            fragment: if let Some(d) = diff.fragment {
                self.fragment.merge(d)?
            } else {
                self.fragment.clone()
            },
        })
    }

    fn into_diff(self) -> identity_diff::Result<Self::Type> {
        match self {
            Self {
                method_name,
                id_segments,
                path_segments,
                query,
                fragment,
            } => Ok(DiffDID {
                method_name,
                id_segments: Some(id_segments.into_diff()?),
                path_segments: if let identity_diff::option::DiffOption::Some(_) = path_segments.clone().into_diff()? {
                    Some(path_segments.into_diff()?)
                } else {
                    None
                },
                query: if let identity_diff::option::DiffOption::Some(_) = query.clone().into_diff()? {
                    Some(query.into_diff()?)
                } else {
                    None
                },
                fragment: if let identity_diff::option::DiffOption::Some(_) = fragment.clone().into_diff()? {
                    Some(fragment.into_diff()?)
                } else {
                    None
                },
            }),
        }
    }

    fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
        match diff {
            DiffDID {
                method_name,
                id_segments,
                path_segments,
                query,
                fragment,
            } => Ok(Self {
                method_name,
                id_segments: <Vec<String>>::from_diff(match id_segments {
                    Some(v) => v,
                    None => <Vec<String>>::default().into_diff()?,
                })?,
                path_segments: <Option<Vec<String>>>::from_diff(match path_segments {
                    Some(v) => v,
                    None => <Option<Vec<String>>>::default().into_diff()?,
                })?,
                query: <Option<Vec<Param>>>::from_diff(match query {
                    Some(v) => v,
                    None => <Option<Vec<Param>>>::default().into_diff()?,
                })?,
                fragment: <Option<String>>::from_diff(match fragment {
                    Some(v) => v,
                    None => <Option<String>>::default().into_diff()?,
                })?,
            }),
        }
    }
}

impl Diff for Param {
    type Type = DiffParam;

    fn diff(&self, other: &Self) -> identity_diff::Result<Self::Type> {
        Ok(DiffParam {
            key: if self.key == other.key {
                None
            } else {
                Some(self.key.diff(&other.key)?)
            },
            value: if self.value == other.value || other.value == None {
                None
            } else {
                Some(self.value.diff(&other.value)?)
            },
        })
    }

    fn merge(&self, diff: Self::Type) -> identity_diff::Result<Self> {
        Ok(Self {
            key: if let Some(d) = diff.key {
                self.key.merge(d)?
            } else {
                self.key.clone()
            },
            value: if let Some(d) = diff.value {
                self.value.merge(d)?
            } else {
                self.value.clone()
            },
        })
    }

    fn into_diff(self) -> identity_diff::Result<Self::Type> {
        match self {
            Self { key, value } => Ok(DiffParam {
                key: Some(key.into_diff()?),
                value: if let identity_diff::option::DiffOption::Some(_) = value.clone().into_diff()? {
                    Some(value.into_diff()?)
                } else {
                    None
                },
            }),
        }
    }
    fn from_diff(diff: Self::Type) -> identity_diff::Result<Self> {
        match diff {
            DiffParam { key, value } => Ok(Self {
                key: <String>::from_diff(match key {
                    Some(v) => v,
                    None => <String>::default().into_diff()?,
                })?,
                value: <Option<String>>::from_diff(match value {
                    Some(v) => v,
                    None => <Option<String>>::default().into_diff()?,
                })?,
            }),
        }
    }
}

// impl Serialize for DiffDID {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = format!("{}", Into::<DID>::into(self.clone()));

//         serializer.serialize_str(s.as_str())
//     }
// }

impl From<DID> for DiffDID {
    fn from(did: DID) -> Self {
        did.into_diff().expect("Unable to convert to diff")
    }
}
impl From<DiffDID> for DID {
    fn from(diff: DiffDID) -> Self {
        Self::from_diff(diff).expect("Unable to convert from diff")
    }
}

impl From<Param> for DiffParam {
    fn from(param: Param) -> Self {
        param.into_diff().expect("Unable to convert to diff")
    }
}
impl From<DiffParam> for Param {
    fn from(diff: DiffParam) -> Self {
        Self::from_diff(diff).expect("Unable to convert from diff")
    }
}
