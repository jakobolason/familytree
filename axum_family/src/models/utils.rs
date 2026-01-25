use serde::Deserialize;

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn date_time_from_str<'de, D>(deserializer: D) -> Result<chrono::naive::NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date_time = chrono::naive::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f")
        .map_err(|_| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"Chrono Naive Date Time",
            )
        })?;
    Ok(date_time)
}

pub fn opt_date_time_from_str<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::naive::NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(s) => {
            let date_time =
                chrono::naive::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f").map_err(
                    |_| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&s),
                            &"Chrono Naive Date Time",
                        )
                    },
                )?;
            Ok(Some(date_time))
        }
        None => Ok(None),
    }
}
