use crate::common::{Object, Url};

/// A `Credential` issuer in object form.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#issuer)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IssuerData {
    pub id: Url,
    #[serde(flatten)]
    pub properties: Object,
}

/// An identifier representing the issuer of a `Credential`.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#issuer)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Issuer {
    Url(Url),
    Obj(IssuerData),
}

impl Issuer {
    pub fn url(&self) -> &Url {
        match self {
            Self::Url(url) => url,
            Self::Obj(obj) => &obj.id,
        }
    }
}

impl<T> From<T> for Issuer
where
    T: Into<Url>,
{
    fn from(other: T) -> Self {
        Self::Url(other.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert::FromJson as _, vc::Issuer};

    const JSON1: &str = include_str!("../../../tests/fixtures/vc/issuer-1.json");
    const JSON2: &str = include_str!("../../../tests/fixtures/vc/issuer-2.json");

    #[test]
    #[rustfmt::skip]
    fn test_from_json() {
        let issuer: Issuer = Issuer::from_json(JSON1).unwrap();
        assert!(matches!(issuer, Issuer::Url(_)));
        assert_eq!(issuer.url(), "https://example.edu/issuers/14");

        let issuer: Issuer = Issuer::from_json(JSON2).unwrap();
        assert!(matches!(issuer, Issuer::Obj(_)));
        assert_eq!(issuer.url(), "did:example:76e12ec712ebc6f1c221ebfeb1f");
    }
}
