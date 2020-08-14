use std::str::FromStr;

use serde::{
    ser::{Serialize, SerializeSeq, Serializer},
    Deserialize,
};

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Clone)]
#[serde(transparent)]
pub struct Context(Vec<String>);

impl Context {
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
