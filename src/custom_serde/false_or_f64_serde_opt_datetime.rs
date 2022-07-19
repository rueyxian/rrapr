use serde::Deserialize;
use serde::Serialize;

///
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FalseOrF64 {
    False(bool),
    F64(f64),
}

///
// unused
// pub fn serialize<S>(
//     datetime: &Option<chrono::DateTime<chrono::Utc>>,
//     serializer: S,
// ) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let timestamp = datetime
//         .map(|dt| FalseOrF64::F64(dt.timestamp() as f64))
//         .unwrap_or(FalseOrF64::False(false));
//     serde::Serialize::serialize(&timestamp, serializer)
// }

///
/// reference: https://stackoverflow.com/questions/56384447/how-do-i-transform-special-values-into-optionnone-when-using-serde-to-deserial
pub(crate) fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp: FalseOrF64 = serde::Deserialize::deserialize(deserializer)?;
    let datetime = match timestamp {
        FalseOrF64::False(must_false) => {
            assert_eq!(must_false, false, "should always be false if it is a bool");
            None
        }
        FalseOrF64::F64(timestamp) => {
            let naive = chrono::naive::NaiveDateTime::from_timestamp(timestamp as i64, 0);
            let datetime = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
            Some(datetime)
        }
    };
    Ok(datetime)
}
