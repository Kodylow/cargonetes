use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::time::Instant;

pub fn serialize_instant<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration_since_epoch =
        instant.duration_since(Instant::now() - Duration::from_secs(5 * 60 * 60)); // Adjust epoch as needed
    let secs = duration_since_epoch.as_secs();
    serializer.serialize_u64(secs)
}

pub fn deserialize_instant<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(Instant::now() - Duration::from_secs(5 * 60 * 60) + Duration::from_secs(secs))
    // Adjust epoch as needed
}

// Custom serialize function for `Option<Instant>`
pub fn serialize_option_instant<S>(
    instant: &Option<Instant>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    instant
        .map(|i| i.elapsed().as_secs_f64())
        .serialize(serializer)
}

// Custom deserialize function for `Option<Instant>`
pub fn deserialize_option_instant<'de, D>(deserializer: D) -> Result<Option<Instant>, D::Error>
where
    D: Deserializer<'de>,
{
    let elapsed_secs: Option<f64> = Option::deserialize(deserializer)?;
    Ok(elapsed_secs.map(|secs| Instant::now() - tokio::time::Duration::from_secs_f64(secs)))
}
