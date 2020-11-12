#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyIndex<'i> {
    Index(usize),
    Ident(&'i str),
}

impl<'i> KeyIndex<'i> {
    pub fn normalize(&self) -> Option<&str> {
        match self {
            Self::Ident(ident) if ident.starts_with("did:") => {
                if let Some(index) = ident.rfind('#') {
                    Some(&ident[index + 1..])
                } else {
                    None
                }
            }
            Self::Ident(ident) => Some(ident.trim_start_matches('#')),
            Self::Index(_) => None,
        }
    }
}

impl<'i> From<&'i str> for KeyIndex<'i> {
    fn from(other: &'i str) -> Self {
        Self::Ident(other)
    }
}

impl From<usize> for KeyIndex<'_> {
    fn from(other: usize) -> Self {
        Self::Index(other)
    }
}

impl PartialEq<usize> for KeyIndex<'_> {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Self::Index(index) => index == other,
            Self::Ident(_) => false,
        }
    }
}

impl PartialEq<&'_ str> for KeyIndex<'_> {
    fn eq(&self, other: &&'_ str) -> bool {
        match self {
            Self::Index(_) => false,
            Self::Ident(ident) => ident == other,
        }
    }
}
