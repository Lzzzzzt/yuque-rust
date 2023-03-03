pub(crate) mod time_serde {
    use chrono::{offset::Local as offset_local, DateTime, Local};
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };

    #[allow(unused)]
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

pub(crate) mod option_time_serde {
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

pub(crate) mod number_to_bool {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    #[allow(unused)]
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

pub(crate) mod toc_serde {
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };
    use serde_yaml::to_string;

    use crate::{Toc, TocItem, TocMeta};

    #[allow(unused)]
    pub fn serialize<S: Serializer>(value: Option<Toc>, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(value) = value {
            let Toc { meta, toc } = value;

            let meta = vec![meta];
            let meta = to_string(&meta).map_err(|e| serde::ser::Error::custom(e.to_string()))?;
            let toc = to_string(&toc).map_err(|e| serde::ser::Error::custom(e.to_string()))?;

            let result = vec![meta, toc].join("\n");

            serializer.serialize_str(&result)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Toc>, D::Error> {
        let value: String = deserializer.deserialize_string(StrVisitor)?.unwrap();

        let meta = value
            .lines()
            .take(9)
            .map(|s| format!("{}\n", s))
            .collect::<String>();
        let toc = value
            .lines()
            .skip(9)
            .map(|s| format!("{}\n", s))
            .collect::<String>();

        let meta = serde_yaml::from_str::<Vec<TocMeta>>(&meta)
            .map_err(|e| de::Error::custom(e.to_string()))?
            .pop()
            .expect("Can not fine Metadata.");

        let toc = serde_yaml::from_str::<Vec<TocItem>>(&toc)
            .map_err(|e| de::Error::custom(e.to_string()))?;

        Ok(Some(Toc { meta, toc }))
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
