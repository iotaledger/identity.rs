#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct MessageId(Option<String>);

impl MessageId {
    pub const NONE: Self = Self(None);

    pub fn new(value: String) -> Self {
        if value.is_empty() {
            Self(None)
        } else {
            Self(Some(value))
        }
    }

    pub const fn is_none(&self) -> bool {
        matches!(self, Self(None))
    }

    pub const fn is_some(&self) -> bool {
        matches!(self, Self(Some(_)))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_deref().unwrap_or_default()
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<String> for MessageId {
    fn from(other: String) -> Self {
        Self::new(other)
    }
}

impl<T> PartialEq<T> for MessageId
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        match self.0.as_deref() {
            Some(inner) => inner == other.as_ref(),
            None => false,
        }
    }
}
