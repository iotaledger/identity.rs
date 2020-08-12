use crate::did_parser::parse;
use std::fmt::{self, Display, Formatter};

const LEADING_TOKENS: &'static str = "did";

type DIDTuple = (String, Option<String>);

/// Decentralized identity structure.  
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DID {
    pub method_name: String,
    pub id_segments: Vec<String>,
    pub params: Option<Vec<Param>>,
    pub path_segments: Option<Vec<String>>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// DID Params struct.
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
            id_segments: id_segments,
            ..Default::default()
        };

        if let Some(prms) = params {
            let ps: Vec<Param> = prms
                .into_iter()
                .map(|pms| Param::new(pms).expect("Format of Param is incorrect"))
                .collect();

            did.params = Some(ps);
        };

        if let Some(_) = fragment {
            did.fragment = fragment;
        };

        if let Some(_) = path_segments {
            did.path_segments = path_segments;
        }

        if let Some(_) = query {
            did.query = query;
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
            "{}",
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
            "{}:{}:{}{}{}{}{}",
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
