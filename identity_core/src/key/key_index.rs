#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyIndex<'i> {
    Index(usize),
    Ident(&'i str),
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
