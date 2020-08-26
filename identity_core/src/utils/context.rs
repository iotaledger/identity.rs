use std::str::FromStr;

use serde::{
    ser::{Serialize, SerializeSeq, Serializer},
    Deserialize,
};
use serde_diff::SerdeDiff;

/// A context type.  Contains a Vector of Strings which describe the DID context.
#[derive(Debug, PartialEq, Eq, Deserialize, Clone, SerdeDiff)]
#[serde(transparent)]
pub struct Context(Vec<String>);

impl Context {
    /// creates a new context.  requires an `inner` value.
    pub fn new(inner: Vec<String>) -> Self {
        Self(inner)
    }

    /// gets the inner value of the context.
    pub fn as_inner(&self) -> &Vec<String> {
        &self.0
    }

    /// checks if the context is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// add a context to an existing context.
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

impl From<&str> for Context {
    fn from(s: &str) -> Context {
        Context(vec![s.into()])
    }
}
