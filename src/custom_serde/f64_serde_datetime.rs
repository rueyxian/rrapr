///
// unused
// pub fn serialize<S>(date: &chrono::DateTime<chrono::Utc>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let timestamp = date.timestamp() as f64;
//     serializer.serialize_f64(timestamp)
// }

///
/// reference: https://serde.rs/custom-date-format.html
pub (crate) fn deserialize<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp: f64 = serde::Deserialize::deserialize(deserializer)?;
    let naive = chrono::naive::NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let datetime: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(naive, chrono::Utc);
    Ok(datetime)
}
