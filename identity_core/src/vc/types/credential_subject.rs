use crate::common::{Object, Url};

/// An entity who is the target of a set of claims.
///
/// [More Info](https://www.w3.org/TR/vc-data-model/#credential-subject)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CredentialSubject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Url>,
    #[serde(flatten)]
    pub properties: Object,
}

impl CredentialSubject {
    pub fn new() -> Self {
        Self::with_properties(Object::new())
    }

    pub fn with_id(id: Url) -> Self {
        Self::with_id_and_properties(id, Object::new())
    }

    pub fn with_properties(properties: Object) -> Self {
        Self { id: None, properties }
    }

    pub fn with_id_and_properties(id: Url, properties: Object) -> Self {
        Self {
            id: Some(id),
            properties,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert::FromJson as _, vc::CredentialSubject};

    const JSON1: &str = include_str!("../../../tests/fixtures/vc/credential-subject-1.json");
    const JSON2: &str = include_str!("../../../tests/fixtures/vc/credential-subject-2.json");
    const JSON3: &str = include_str!("../../../tests/fixtures/vc/credential-subject-3.json");
    const JSON4: &str = include_str!("../../../tests/fixtures/vc/credential-subject-4.json");
    const JSON5: &str = include_str!("../../../tests/fixtures/vc/credential-subject-5.json");
    const JSON6: &str = include_str!("../../../tests/fixtures/vc/credential-subject-6.json");
    const JSON7: &str = include_str!("../../../tests/fixtures/vc/credential-subject-7.json");
    const JSON8: &str = include_str!("../../../tests/fixtures/vc/credential-subject-8.json");
    const JSON9: &str = include_str!("../../../tests/fixtures/vc/credential-subject-9.json");
    const JSON10: &str = include_str!("../../../tests/fixtures/vc/credential-subject-10.json");

    #[test]
    #[rustfmt::skip]
    fn test_from_json() {
        let subject: CredentialSubject = CredentialSubject::from_json(JSON1).unwrap();
        assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
        assert_eq!(subject.properties["alumniOf"]["id"], "did:example:c276e12ec21ebfeb1f712ebc6f1");
        assert_eq!(subject.properties["alumniOf"]["name"][0]["value"], "Example University");
        assert_eq!(subject.properties["alumniOf"]["name"][0]["lang"], "en");
        assert_eq!(subject.properties["alumniOf"]["name"][1]["value"], "Exemple d'Université");
        assert_eq!(subject.properties["alumniOf"]["name"][1]["lang"], "fr");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON2).unwrap();
        assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
        assert_eq!(subject.properties["degree"]["type"], "BachelorDegree");
        assert_eq!(subject.properties["degree"]["name"], "Bachelor of Science and Arts");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON3).unwrap();
        assert_eq!(subject.id.unwrap(), "did:example:abcdef1234567");
        assert_eq!(subject.properties["name"], "Jane Doe");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON4).unwrap();
        assert_eq!(subject.id.unwrap(), "did:example:abcdef1234567");
        assert_eq!(subject.properties["name"], "Jane Doe");
        assert_eq!(subject.properties["favoriteFood"], "Papaya");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON5).unwrap();
        assert_eq!(subject.properties["givenName"], "Jane");
        assert_eq!(subject.properties["familyName"], "Doe");
        assert_eq!(subject.properties["degree"]["type"], "BachelorDegree");
        assert_eq!(subject.properties["degree"]["name"], "Bachelor of Science and Arts");
        assert_eq!(subject.properties["degree"]["college"], "College of Engineering");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON6).unwrap();
        assert_eq!(subject.properties["degreeType"], "BachelorDegree");
        assert_eq!(subject.properties["degreeSchool"], "College of Engineering");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON7).unwrap();
        assert_eq!(subject.id.unwrap(), "http://example.com/credentials/245");
        assert_eq!(subject.properties["currentStatus"], "Disputed");
        assert_eq!(subject.properties["statusReason"]["value"], "Address is out of date.");
        assert_eq!(subject.properties["statusReason"]["lang"], "en");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON8).unwrap();
        assert_eq!(subject.properties["degree"]["type"], "BachelorDegree");
        assert_eq!(subject.properties["degree"]["name"], "Bachelor of Science and Arts");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON9).unwrap();
        assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
        assert_eq!(subject.properties["image"], "https://example.edu/images/58473");
        assert_eq!(subject.properties["alumniOf"]["id"], "did:example:c276e12ec21ebfeb1f712ebc6f1");
        assert_eq!(subject.properties["alumniOf"]["name"][0]["value"], "Example University");
        assert_eq!(subject.properties["alumniOf"]["name"][0]["lang"], "en");
        assert_eq!(subject.properties["alumniOf"]["name"][1]["value"], "Exemple d'Université");
        assert_eq!(subject.properties["alumniOf"]["name"][1]["lang"], "fr");

        let subject: CredentialSubject = CredentialSubject::from_json(JSON10).unwrap();
        assert_eq!(subject.id.unwrap(), "did:example:ebfeb1f712ebc6f1c276e12ec21");
        assert_eq!(subject.properties["image"], "ipfs:/ipfs/QmXfrS3pHerg44zzK6QKQj6JDk8H6cMtQS7pdXbohwNQfK/image");
        assert_eq!(subject.properties["alumniOf"]["id"], "did:example:c276e12ec21ebfeb1f712ebc6f1");
        assert_eq!(subject.properties["alumniOf"]["name"][0]["value"], "Example University");
        assert_eq!(subject.properties["alumniOf"]["name"][0]["lang"], "en");
        assert_eq!(subject.properties["alumniOf"]["name"][1]["value"], "Exemple d'Université");
        assert_eq!(subject.properties["alumniOf"]["name"][1]["lang"], "fr");
    }
}
