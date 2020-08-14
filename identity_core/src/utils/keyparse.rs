use crate::utils::{KeyEncodingType, PublicKey, PublicKeyTypes, Subject};

use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
};

use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

enum Field {
    Subject,
    Type,
    Controller,
    Key(KeyEncodingType),
}

struct PublicKeyVisitor;
struct FieldVisitor;

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PublicKeyVisitor)
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FieldVisitor)
    }
}
impl<'de> Visitor<'de> for PublicKeyVisitor {
    type Value = PublicKey;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Expecting DID Public Key Struct")
    }

    fn visit_str<E>(self, value: &str) -> Result<PublicKey, E>
    where
        E: de::Error,
    {
        Ok(PublicKey {
            id: Subject::from_str(value).unwrap(),
            ..Default::default()
        })
    }

    fn visit_map<M>(self, mut map: M) -> Result<PublicKey, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut id = None;
        let mut key_type = None;
        let mut controller = None;
        let mut encoding_type = None;
        let mut key_data = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Subject => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
                    }
                    id = Some(map.next_value()?);
                }
                Field::Type => {
                    if key_type.is_some() {
                        return Err(de::Error::duplicate_field("type"));
                    }
                    key_type = Some(map.next_value()?);
                }
                Field::Controller => {
                    if controller.is_some() {
                        return Err(de::Error::duplicate_field("controller"));
                    }
                    controller = Some(map.next_value()?);
                }
                Field::Key(encoding) => {
                    if key_data.is_some() {
                        return Err(de::Error::duplicate_field("key data"));
                    }
                    encoding_type = Some(encoding);
                    key_data = Some(map.next_value()?);
                }
            }
        }

        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let key_type = key_type.ok_or_else(|| de::Error::missing_field("type"))?;
        let controller = controller.ok_or_else(|| de::Error::missing_field("controller"))?;
        let encoding_type = encoding_type.ok_or_else(|| de::Error::missing_field("key data"))?;
        let key_data = key_data.ok_or_else(|| de::Error::missing_field("key data"))?;

        Ok(PublicKey {
            id,
            key_type,
            controller,
            encoding_type,
            key_data,
            reference: false,
        })
    }
}

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Expected `id`, `type`, `controller` or a base key type")
    }

    fn visit_str<E>(self, value: &str) -> Result<Field, E>
    where
        E: de::Error,
    {
        match value {
            "id" => Ok(Field::Subject),
            "type" => Ok(Field::Type),
            "controller" => Ok(Field::Controller),
            _ => {
                if let Ok(encoding) = KeyEncodingType::from_str(value) {
                    Ok(Field::Key(encoding))
                } else {
                    Err(de::Error::unknown_field(value, &[]))
                }
            }
        }
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.reference {
            self.id.serialize(serializer)
        } else {
            let mut pk = serializer.serialize_struct("", 4)?;
            pk.serialize_field("id", &self.id)?;
            pk.serialize_field("type", &self.key_type)?;
            pk.serialize_field("controller", &self.controller)?;
            match self.encoding_type {
                KeyEncodingType::Unknown => pk.serialize_field("publicKeyUnknown", &self.key_data)?,
                KeyEncodingType::Pem => pk.serialize_field("publicKeyPem", &self.key_data)?,
                KeyEncodingType::Jwk => pk.serialize_field("publicKeyJwk", &self.key_data)?,
                KeyEncodingType::Hex => pk.serialize_field("publicKeyHex", &self.key_data)?,
                KeyEncodingType::Base64 => pk.serialize_field("publicKeyBase64", &self.key_data)?,
                KeyEncodingType::Base58 => pk.serialize_field("publicKeyBase58", &self.key_data)?,
                KeyEncodingType::Multibase => pk.serialize_field("publicKeyMultibase", &self.key_data)?,
                KeyEncodingType::EthereumAddress => pk.serialize_field("ethereumAddress", &self.key_data)?,
                KeyEncodingType::IotaAddress => pk.serialize_field("iotaAddress", &self.key_data)?,
            }
            pk.end()
        }
    }
}
