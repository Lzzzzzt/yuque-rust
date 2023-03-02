pub mod time_serde {
    use chrono::{offset::Local as offset_local, DateTime, Local};
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };

    pub fn serialize<S: Serializer>(
        time: DateTime<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(time.timestamp_millis())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Local>, D::Error> {
        let time: String = deserializer.deserialize_string(StrVisitor)?;
        let time = DateTime::parse_from_rfc3339(&time).map_err(de::Error::custom)?;
        let now = *Local::now().offset();
        Ok(DateTime::<offset_local>::from_local(
            time.naive_local(),
            now,
        ))
    }

    struct StrVisitor;

    impl<'de> Visitor<'de> for StrVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "is a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v.to_string())
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
}

pub mod option_time_serde {
    #![allow(unused)]

    use chrono::{offset::Local as offset_local, DateTime, Local};
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };

    pub fn serialize<S: Serializer>(
        time: Option<DateTime<Local>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(time) = time {
            serializer.serialize_i64(time.timestamp_millis())
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<DateTime<Local>>, D::Error> {
        let time: Option<String> = deserializer.deserialize_str(StrVisitor).ok().flatten();
        if let Some(time) = time {
            let time = DateTime::parse_from_rfc3339(&time).map_err(de::Error::custom)?;
            let now = *Local::now().offset();
            Ok(Some(DateTime::<offset_local>::from_local(
                time.naive_local(),
                now,
            )))
        } else {
            Ok(None)
        }
    }

    struct StrVisitor;

    impl<'de> Visitor<'de> for StrVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "is a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
}

pub mod number_to_bool {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(value: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        if *value {
            serializer.serialize_u8(1)
        } else {
            serializer.serialize_u8(0)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        let value: u8 = Deserialize::deserialize(deserializer)?;
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(D::Error::custom("invalid value")),
        }
    }
}
