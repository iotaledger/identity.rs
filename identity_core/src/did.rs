use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt::{self, Display, Formatter};

use crate::did_parser::parse;

const LEADING_TOKENS: &str = "did";

/// An aliased tuple the converts into a `Param` type.
type DIDTuple = (String, Option<String>);

/// a Decentralized identity structure.  
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DID {
    pub method_name: String,
    pub id_segments: Vec<String>,
    pub params: Option<Vec<Param>>,
    pub path_segments: Option<Vec<String>>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// a DID Params struct.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub name: String,
    pub value: Option<String>,
}

impl DID {
    /// Creates a new DID. `params` and `fragment` are both optional.
    pub fn new(
        name: String,
        id_segments: Vec<String>,
        params: Option<Vec<DIDTuple>>,
        path_segments: Option<Vec<String>>,
        query: Option<String>,
        fragment: Option<String>,
    ) -> crate::Result<Self> {
        let mut did = DID {
            method_name: name,
            id_segments,
            ..Default::default()
        };

        if let Some(prms) = params {
            let ps: Vec<Param> = prms
                .into_iter()
                .map(|pms| Param::new(pms).expect("Format of Param is incorrect"))
                .collect();

            did.params = Some(ps);
        };

        if let Some(frag) = fragment {
            did.add_fragment(frag);
        };

        if let Some(path) = path_segments {
            did.add_path_segments(path);
        }

        if let Some(qry) = query {
            did.add_query(qry);
        }

        // constrain DID parameters with parser.
        DID::parse_from_str(format!("{}", did))?;

        Ok(did)
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

impl Param {
    /// Creates a new Param struct.
    pub fn new(params: DIDTuple) -> crate::Result<Self> {
        let (name, value) = params;

        if name == String::new() {
            Err(crate::Error::FormatError(
                "Format of the params is incorrect or empty".into(),
            ))
        } else {
            Ok(Param { name, value })
        }
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
