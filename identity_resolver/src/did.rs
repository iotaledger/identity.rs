use std::fmt::{self, Display, Formatter};

/// the leading method tokens.
const LEADING_TOKENS: &'static str = "did";

/// Decentralized identity structure.  
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DID {
    pub method_name: String,
    pub specific_id: String,
    pub params: Option<Vec<Param>>,
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
    pub fn new(name: String, id: String, params: Option<Vec<Param>>, fragment: Option<String>) -> crate::Result<Self> {
        let mut did = DID {
            method_name: name,
            specific_id: id,
            ..Default::default()
        };

        if let Some(_) = params {
            did.params = params;
        };

        if let Some(_) = fragment {
            did.fragment = fragment;
        };

        Ok(did)
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
    fn new(params: (String, String)) -> crate::Result<Self> {
        let (name, value) = params;

        Ok(Param {
            name,
            value: Some(value),
        })
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

        write!(
            f,
            "{}:{}:{}{}{}",
            LEADING_TOKENS, self.method_name, self.specific_id, prms, frag
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
        let did = DID::new("iota".into(), "123456".into(), None, None).unwrap();

        assert_eq!(did.specific_id, "123456",);
        assert_eq!(did.method_name, "iota");
        assert_eq!(format!("{}", did), "did:iota:123456");
    }

    #[test]
    fn test_param() {
        let param = Param::new(("name".into(), "value".into())).unwrap();

        assert_eq!(param.name, "name");
        assert_eq!(param.value, Some(String::from("value")));
        assert_eq!(format!("{}", param), "name=value");
    }

    #[test]
    fn test_frag() {
        let mut did = DID::new("iota".into(), "123456".into(), None, None).unwrap();

        did.add_fragment("a-fragment".into());

        assert_eq!(did.fragment, Some(String::from("a-fragment")));
        assert_eq!(format!("{}", did), "did:iota:123456#a-fragment");
    }

    #[test]
    fn test_params() {
        let param_a = Param::new(("param".into(), "a".into())).unwrap();
        let param_b = Param::new(("param".into(), "b".into())).unwrap();
        let params = Some(vec![param_a.clone(), param_b.clone()]);
        let mut did = DID::new("iota".into(), "123456".into(), params.clone(), None).unwrap();

        assert_eq!(format!("{}", did), "did:iota:123456;param=a;param=b");
        assert_eq!(did.params, params);

        let param_c = Param::new(("param".into(), "c".into())).unwrap();
        let params = vec![param_c.clone()];
        did.add_params(params);

        assert_eq!(did.params, Some(vec![param_a, param_b, param_c]));
    }
}
