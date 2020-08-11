use std::fmt::{self, Display, Formatter};

const LEADING_TOKENS: &'static str = "did";

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
        params: Option<Vec<(String, Option<String>)>>,
        path_segments: Option<Vec<String>>,
        query: Option<String>,
        fragment: Option<String>,
    ) -> Self {
        let mut did = DID {
            method_name: name,
            id_segments: id_segments,
            ..Default::default()
        };

        if let Some(prms) = params {
            let ps: Vec<Param> = prms.into_iter().map(|pms| Param::new(pms)).collect();

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

        did
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

    /// Method to add a fragment to the DID.  
    pub fn add_fragment(&mut self, fragment: String) {
        self.fragment = Some(fragment);
    }
}

impl Param {
    /// Creates a new Param struct.
    fn new(params: (String, Option<String>)) -> Self {
        let (name, value) = params;

        Param { name, value }
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
            LEADING_TOKENS, self.method_name, formatted_ids, prms, frag, path_segs, query
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

    #[test]
    fn test_create_did() {
        let did = DID::new("iota".into(), vec!["123456".into()], None, None, None, None);

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
        );

        assert_eq!(format!("{}", did), "did:iota:123456:789011;name=value");
    }

    #[test]
    fn test_param() {
        let param = Param::new(("name".into(), Some("value".into())));

        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some(String::from("value")));
        assert_eq!(format!("{}", param), "name=value");
    }

    #[test]
    fn test_frag() {
        let mut did = DID::new("iota".into(), vec!["123456".into()], None, None, None, None);

        did.add_fragment("a-fragment".into());

        assert_eq!(did.fragment, Some(String::from("a-fragment")));
        assert_eq!(format!("{}", did), "did:iota:123456#a-fragment");
    }

    #[test]
    fn test_params() {
        let param_a = Param::new(("param".into(), Some("a".into())));
        let param_b = Param::new(("param".into(), Some("b".into())));
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
        );

        assert_eq!(format!("{}", did), "did:iota:123456;param=a;param=b");
        assert_eq!(did.params, params);

        let param_c = Param::new(("param".into(), Some("c".into())));
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
            Some("a_fragement".into()),
        );

        assert_eq!(
            format!("{}", did),
            "did:iota:123456;param=a;param=b#a_fragement/some_path?some_query"
        );
    }
}
