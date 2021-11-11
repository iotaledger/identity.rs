use std::fmt::Display;

#[derive(Debug)]
/// Error indicating that a fundamental assumption or invariant has been broken.  
pub struct FatalError{
    inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    kind: ErrorKind,
}


impl FatalError {
    /// Consumes the error returning its inner error (if any).
    pub fn into_inner(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        self.inner
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
/// A list specifying categories of fatal errors from identity-core 
pub enum ErrorKind {
    /// The number of public keys does not match the number of private keys when this was expected
    KeyPairImbalance, 
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            Self::KeyPairImbalance => "the number of public keys does not match the number of private keys",
        }
    }
}

impl Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.as_str())
    }
}

impl std::error::Error for FatalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.and_then(|error| error.source())
    }
}

impl From<ErrorKind> for FatalError {
    fn from(kind: ErrorKind) -> Self {
        Self {inner: None, kind}
    }
}