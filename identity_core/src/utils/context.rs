use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Context(Vec<String>);

impl Context {
    pub fn as_inner(&self) -> &Vec<String> {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for Context {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Context(vec![s.to_owned()]))
    }
}
