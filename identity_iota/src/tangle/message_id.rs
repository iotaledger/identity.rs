use core::fmt::{Debug, Formatter, Result};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct MessageId(Option<String>);

impl MessageId {
    pub const NONE: Self = Self(None);

    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        let value: String = value.into();

        if maybe_trytes(&value) {
            Self(Some(value))
        } else {
            Self(None)
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

impl Debug for MessageId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(self.0.as_deref().unwrap_or_default())
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

fn maybe_trytes(input: &str) -> bool {
    if input.len() != iota_constants::HASH_TRYTES_SIZE {
        return false;
    }

    input.chars().all(|ch| iota_constants::TRYTE_ALPHABET.contains(&ch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_new() {
        // Valid
        assert!(MessageId::new("BCZVFSZPSMLPEQUXVWQQIHHQJJXZPZWRERJVBKKXHKMJAQJUN9OIXDBKMWFSAQIGC9YNXCCFOFKBQZ999").is_some());
        assert!(MessageId::new("999999999999999999999999999999999999999999999999999999999999999999999999999999999").is_some());

        // Invalid
        assert!(MessageId::new("").is_none());
        assert!(MessageId::new("         ").is_none());
        assert!(MessageId::new("- - - - - - -").is_none());
        assert!(MessageId::new("999999999999999999-999999999999999999999999999-9999999999999999999999-99999999999").is_none());
        assert!(MessageId::new("BCZVFSZPSMLPEQUXVWQQIHHQJJXZPZWRERJVBKKXHKMJAQJUN9OIXDBKMWFSAQIGC9YNXCCFOFKBQZ").is_none());
    }
}
