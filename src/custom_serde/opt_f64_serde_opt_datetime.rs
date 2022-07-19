///
// unused
// pub fn serialize<S>(
//     datetime: &Option<chrono::DateTime<chrono::Utc>>,
//     serializer: S,
// ) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let timestamp = datetime.map(|datetime| datetime.timestamp() as f64);
//     serde::Serialize::serialize(&timestamp, serializer)
// }

///
pub (crate) fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp: Option<f64> = serde::Deserialize::deserialize(deserializer)?;
    let datetime = timestamp.map(|timestamp| {
        let naive = chrono::naive::NaiveDateTime::from_timestamp(timestamp as i64, 0);
        chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc)
    });
    Ok(datetime)
}
