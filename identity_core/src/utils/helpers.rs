use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use std::{fmt, marker::PhantomData, str::FromStr};

pub fn string_or_list<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = crate::Error>,
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrList(PhantomData))
}

pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = crate::Error>,
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

struct StringOrList<T>(PhantomData<fn() -> T>);

struct StringOrStruct<T>(PhantomData<fn() -> T>);

impl<'de, T> Visitor<'de> for StringOrList<T>
where
    T: Deserialize<'de> + FromStr<Err = crate::Error>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or list")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
        E: de::Error,
    {
        Ok(FromStr::from_str(value).unwrap())
    }

    fn visit_seq<S>(self, seq: S) -> Result<T, S::Error>
    where
        S: SeqAccess<'de>,
    {
        Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
    }
}

impl<'de, T> Visitor<'de> for StringOrStruct<T>
where
    T: Deserialize<'de> + FromStr<Err = crate::Error>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
        E: de::Error,
    {
        Ok(FromStr::from_str(value).unwrap())
    }

    fn visit_map<M>(self, map: M) -> Result<T, M::Error>
    where
        M: MapAccess<'de>,
    {
        Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
    }
}
