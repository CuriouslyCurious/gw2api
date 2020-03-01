use reqwest::{StatusCode, Response};
use serde::de::DeserializeOwned;
use crate::error::ApiError;

/// Possible teams used in WvW or SPvP.
#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
pub enum Team {
    #[serde(alias = "red")]
    Red,
    #[serde(alias = "green")]
    Green,
    #[serde(alias = "blue")]
    Blue,
    #[serde(alias = "neutral")]
    Neutral,
}

/// All the professions currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Profession {
    Revenant,
    Warrior,
    Guardian,
    Thief,
    Ranger,
    Engineer,
    Necromancer,
    Mesmer,
    Elementalist,
}

/// Convert a `Vec<T>` to a comma-separated `String`
pub fn ids_to_string(ids: Vec<impl ToString>) -> String {
    let mut ids: String = ids.iter().map(|id| format!("{},", id.to_string())).collect();
    ids.pop(); // Remove the last comma
    ids
}

/// Parse a response from the API into the appropriate type. Returns an `ApiError` upon failed
/// deserialization to the new type.
pub fn parse_response<T>(response: &mut Response) -> Result<T, ApiError>
where T: DeserializeOwned {
    match response.status() {
        // When everything is a-ok.
        StatusCode::OK => Ok(response.json::<T>().unwrap()),
        // Request timed out (for example when requesting a lot of data)
        StatusCode::REQUEST_TIMEOUT => Err(response.json::<ApiError>().unwrap()),
        // The endpoint might be down or disabled.
        StatusCode::NOT_FOUND => Err(response.json::<ApiError>().unwrap()),
        // Occurs when only some of the content requested exists.
        StatusCode::PARTIAL_CONTENT => Ok(response.json::<T>().unwrap()),
        _ => Err(response.json::<ApiError>().unwrap()),
    }
}

