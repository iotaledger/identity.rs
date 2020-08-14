use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
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
        Ok(Context(vec![s.to_owned()]))
    }
}
