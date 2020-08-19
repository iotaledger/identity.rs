use std::str::FromStr;

use serde::{
    ser::{Serialize, SerializeSeq, Serializer},
    Deserialize,
};

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(transparent)]
pub struct Context(Vec<String>);

impl Context {
    pub fn new(inner: Vec<String>) -> Self {
        Self(inner)
    }
    pub fn as_inner(&self) -> &Vec<String> {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add_context(&mut self, s: String) -> crate::Result<Self> {
        self.0.push(s);

        Ok(self.clone())
    }
}

/// Default context to "https://www.w3.org/ns/did/v1" as per the standard.
impl Default for Context {
    fn default() -> Self {
        Context(vec!["https://www.w3.org/ns/did/v1".into()])
    }
}

impl FromStr for Context {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Context(vec![s.into()]))
    }
}

impl Serialize for Context {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0.len() {
            0 => serializer.serialize_none(),
            1 => serializer.serialize_str(&self.0[0]),
            _ => {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for ch in &self.0 {
                    seq.serialize_element(&ch)?;
                }
                seq.end()
            }
        }
    }
}
