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
    name: String,
    value: Option<String>,
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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use totems::assert_err;

    #[test]
    fn test_create_did() {
        let did = DID::new("iota".into(), vec!["123456".into()], None, None, None, None).unwrap();

        assert_eq!(did.id_segments, vec!["123456"]);
        assert_eq!(did.method_name, "iota");
        assert_eq!(format!("{}", did), "did:iota:123456");
    }

    #[test]
    fn test_multiple_ids() {
        let did = DID::new(
            "iota".into(),
            vec!["123456".into(), "789011".into()],
            Some(vec![("name".into(), Some("value".into()))]),
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(format!("{}", did), "did:iota:123456:789011;name=value");
    }

    #[test]
    fn test_param() {
        let param = Param::new(("name".into(), Some("value".into()))).unwrap();

        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some(String::from("value")));
        assert_eq!(format!("{}", param), "name=value");
    }

    #[test]
    fn test_frag() {
        let mut did = DID::new("iota".into(), vec!["123456".into()], None, None, None, None).unwrap();

        did.add_fragment("a-fragment".into());

        assert_eq!(did.fragment, Some(String::from("a-fragment")));
        assert_eq!(format!("{}", did), "did:iota:123456#a-fragment");
    }

    #[test]
    fn test_params() {
        let param_a = Param::new(("param".into(), Some("a".into()))).unwrap();
        let param_b = Param::new(("param".into(), Some("b".into()))).unwrap();
        let params = Some(vec![param_a.clone(), param_b.clone()]);
        let mut did = DID::new(
            "iota".into(),
            vec!["123456".into()],
            Some(vec![
                ("param".into(), Some("a".into())),
                ("param".into(), Some("b".into())),
            ]),
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(format!("{}", did), "did:iota:123456;param=a;param=b");
        assert_eq!(did.params, params);

        let param_c = Param::new(("param".into(), Some("c".into()))).unwrap();
        let params = vec![param_c.clone()];
        did.add_params(params);

        assert_eq!(did.params, Some(vec![param_a, param_b, param_c]));
    }
    #[test]
    fn test_full_did() {
        let did = DID::new(
            "iota".into(),
            vec!["123456".into()],
            Some(vec![
                ("param".into(), Some("a".into())),
                ("param".into(), Some("b".into())),
            ]),
            Some(vec!["some_path".into()]),
            Some("some_query".into()),
            Some("a_fragment".into()),
        )
        .unwrap();

        assert_eq!(
            format!("{}", did),
            "did:iota:123456;param=a;param=b/some_path?some_query#a_fragment"
        );
    }

    #[test]
    fn test_parser() {
        let did = DID::parse_from_str("did:iota:123456;param=a;param=b/some_path?some_query#a_fragment").unwrap();
        let param_a = Param::new(("param".into(), Some("a".into()))).unwrap();
        let param_b = Param::new(("param".into(), Some("b".into()))).unwrap();

        assert_eq!(
            format!("{}", did),
            "did:iota:123456;param=a;param=b/some_path?some_query#a_fragment"
        );
        assert_eq!(
            did,
            DID {
                method_name: "iota".into(),
                id_segments: vec!["123456".into()],
                params: Some(vec![param_a, param_b]),
                path_segments: Some(vec!["some_path".into()]),
                query: Some("some_query".into()),
                fragment: Some("a_fragment".into())
            }
        );
    }

    #[test]
    fn test_multiple_paths() {
        let did = DID::parse_from_str("did:iota:123456/some_path_a/some_path_b").unwrap();

        assert_eq!(format!("{}", did), "did:iota:123456/some_path_a/some_path_b");
        assert_eq!(
            did,
            DID {
                method_name: "iota".into(),
                id_segments: vec!["123456".into()],
                params: None,
                path_segments: Some(vec!["some_path_a".into(), "some_path_b".into()]),
                query: None,
                fragment: None,
            }
        )
    }

    #[test]
    fn test_parsing_contraints() {
        let did = DID::parse_from_str("did:IOTA:12345");

        assert_err!(did);

        let did = DID::parse_from_str("did:iota:%$^@1234");

        assert_err!(did);

        let did = DID::parse_from_str("x:iota:123456");

        assert_err!(did);
    }

    fn wrapper_did_id_seg(s: &str) -> Option<DID> {
        let did_str = format!("did:iota:{}", s);

        DID::parse_from_str(&did_str).unwrap();

        Some(DID::new("iota".into(), vec![s.into()], None, None, None, None).unwrap())
    }

    fn wrapper_did_name(s: &str) -> Option<DID> {
        let did_str = format!("did:{}:12345678", s);

        DID::parse_from_str(&did_str).unwrap();

        Some(DID::new(s.into(), vec!["12345678".into()], None, None, None, None).unwrap())
    }

    fn wrapper_did_params(n: &str, v: &str) -> Option<DID> {
        let did_str = format!("did:iota:12345678;{}={}", n, v);

        DID::parse_from_str(did_str).unwrap();

        Some(
            DID::new(
                "iota".into(),
                vec!["12345678".into()],
                Some(vec![(n.into(), Some(v.into()))]),
                None,
                None,
                None,
            )
            .unwrap(),
        )
    }

    fn wrapper_did_query(q: &str) -> Option<DID> {
        let did_str = format!("did:iota:12345678?{}", q);

        DID::parse_from_str(did_str).unwrap();

        Some(DID::new("iota".into(), vec!["12345678".into()], None, None, Some(q.into()), None).unwrap())
    }

    proptest! {
        #[test]
        fn prop_parse_did_id_seg(s in "[a-z0-9A-Z._-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
            wrapper_did_id_seg(&s);
        }

        #[test]
        fn prop_parse_did_name(s in "[a-z0-9]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
            wrapper_did_name(&s);
        }

        #[test]
        fn prop_parse_did_params(n in "[a-zA-Z0-9.=:-]+", v in "[a-zA-Z0-9.=:-]*".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
            wrapper_did_params(&n, &v);
        }

        #[test]
        fn prop_parse_did_query(q in "[a-zA-Z0-9._!~$&'()*+;,=:@/?-]+".prop_filter("Values must be Ascii", |v| v.is_ascii())) {
            wrapper_did_query(&q);
        }
    }
}
