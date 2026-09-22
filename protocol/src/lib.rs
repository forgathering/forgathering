use std::fmt;

use base64::{Engine, engine::general_purpose};
use serde::{
    Deserializer, Serializer,
    de::{self, Unexpected, Visitor},
};

pub mod v0;

pub fn serialize_image<S>(image: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match image {
        Some(image) => serializer.serialize_some(&general_purpose::STANDARD.encode(image)),
        None => serializer.serialize_none(),
    }
}

struct ImageStringVisitor;

impl Visitor<'_> for ImageStringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an image string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }
}

struct ImageVisitor;

impl<'de> Visitor<'de> for ImageVisitor {
    type Value = Option<Vec<u8>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an optional base64'd image")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let image_string = deserializer.deserialize_string(ImageStringVisitor)?;

        Ok(Some(
            general_purpose::STANDARD
                .decode(&image_string)
                .map_err(|_| de::Error::invalid_value(Unexpected::Str(&image_string), &self))?,
        ))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn deserialize_image<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(ImageVisitor)
}
