use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
    Deserialize as DDeserialize, Serialize as DSerialize,
};
use serde_diff::SerdeDiff;
use std::fmt::{self, Display, Formatter};

use crate::did_parser::parse;

const LEADING_TOKENS: &str = "did";

/// An aliased tuple the converts into a `Param` type.
type DIDTuple = (String, Option<String>);

/// a Decentralized identity structure.  
#[derive(Debug, PartialEq, Default, Eq, Clone, SerdeDiff)]
pub struct DID {
    pub method_name: String,
    pub id_segments: Vec<String>,
    pub params: Option<Vec<Param>>,
    pub path_segments: Option<Vec<String>>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// a DID Params struct.
#[derive(Debug, PartialEq, Eq, Clone, Default, SerdeDiff, DDeserialize, DSerialize)]
pub struct Param {
    pub name: String,
    pub value: Option<String>,
}

impl DID {
    /// Initializes the DID struct with the filled out fields. Also runs parse_from_str to validate the fields.
    pub fn init(self) -> crate::Result<DID> {
        let did = DID {
            method_name: self.method_name,
            id_segments: self.id_segments,
            params: self.params,
            fragment: self.fragment,
            path_segments: self.path_segments,
            query: self.query,
        };

        DID::parse_from_str(did)
    }

    pub fn parse_from_str<T>(input: T) -> crate::Result<Self>
    where
        T: ToString,
    {
        parse(input)
    }

    /// Method to add params to the DID.  
    pub fn add_params(&mut self, params: Vec<Param>) {
        let ps = match &mut self.params {
            Some(v) => {
                v.extend(params);

                v
            }
            None => &params,
        };

        self.params = Some(ps.clone());
    }

    /// add path segments to the current DID.
    pub fn add_path_segments(&mut self, path_segment: Vec<String>) {
        let ps = match &mut self.path_segments {
            Some(p) => {
                p.extend(path_segment);

                p
            }
            None => &path_segment,
        };

        self.path_segments = Some(ps.clone());
    }

    /// add a query to the DID.
    pub fn add_query(&mut self, query: String) {
        self.query = Some(query);
    }

    /// Method to add a fragment to the DID.  
    pub fn add_fragment(&mut self, fragment: String) {
        self.fragment = Some(fragment);
    }
}

/// Display trait for the DID struct.
impl Display for DID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let prms = match &self.params {
            Some(ps) => format!(
                ";{}",
                ps.iter().map(ToString::to_string).fold(&mut String::new(), |acc, p| {
                    if !acc.is_empty() {
                        acc.push_str(";");
                    }
                    acc.push_str(&p);

                    acc
                })
            ),
            None => String::new(),
        };

        let frag = match &self.fragment {
            Some(f) => format!("#{}", f),
            None => String::new(),
        };

        let formatted_ids = format!(
            ":{}",
            self.id_segments
                .iter()
                .map(ToString::to_string)
                .fold(&mut String::new(), |acc, p| {
                    if !acc.is_empty() {
                        acc.push_str(":");
                    }
                    acc.push_str(&p);

                    acc
                })
        );

        let path_segs = match &self.path_segments {
            Some(segs) => format!(
                "/{}",
                segs.iter().map(ToString::to_string).fold(&mut String::new(), |acc, p| {
                    if !acc.is_empty() {
                        acc.push_str("/");
                    }
                    acc.push_str(&p);

                    acc
                })
            ),
            None => String::new(),
        };

        let query = match &self.query {
            Some(q) => format!("?{}", q),
            None => String::new(),
        };

        write!(
            f,
            "{}:{}{}{}{}{}{}",
            LEADING_TOKENS, self.method_name, formatted_ids, prms, path_segs, query, frag
        )
    }
}

/// Display trait for the param struct.
impl Display for Param {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let val = match &self.value {
            Some(v) => format!("={}", v),
            None => String::new(),
        };

        write!(f, "{}{}", self.name, val)
    }
}

/// deserialize logic for the `DID` type.
impl<'de> Deserialize<'de> for DID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DIDVisitor;

        impl<'de> Visitor<'de> for DIDVisitor {
            type Value = DID;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("DID String")
            }

            fn visit_str<V>(self, value: &str) -> Result<DID, V>
            where
                V: de::Error,
            {
                match DID::parse_from_str(value) {
                    Ok(d) => Ok(d),
                    Err(e) => Err(de::Error::custom(e.to_string())),
                }
            }
        }

        deserializer.deserialize_any(DIDVisitor)
    }
}

/// serialize logic for the `DID` type.
impl Serialize for DID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self);

        serializer.serialize_str(s.as_str())
    }
}

impl From<DIDTuple> for Param {
    fn from(tuple: DIDTuple) -> Param {
        let (name, value) = tuple;

        Param { name, value }
    }
}
