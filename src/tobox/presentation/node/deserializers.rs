use std::fmt;

use serde::de;
use uuid::Uuid;

pub fn deserialize_uuid_list<'de, D>(deserializer: D) -> Result<Option<Vec<Uuid>>, D::Error>
    where
        D: de::Deserializer<'de>,
{
    struct StringVecVisitor;

    impl<'de> de::Visitor<'de> for StringVecVisitor {
        type Value = Option<Vec<Uuid>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a list of UUIDs")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(";") {
                let id = Uuid::parse_str(id).map_err(E::custom)?;
                ids.push(id);
            }
            Ok(Option::from(ids))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringVecVisitor)
}