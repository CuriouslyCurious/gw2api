use reqwest::{StatusCode, Response};
use serde::de::DeserializeOwned;
use crate::error::ApiError;

/// Convert a `Vec<u32>` to a comma-separated `String`
pub fn ids_to_string(ids: Vec<u32>) -> String {
    let mut ids: String = ids.iter().map(|id| format!("{},", id.to_string())).collect();
    ids.pop(); // Remove the last comma
    ids
}

/// Parse a response from the API into the appropriate type. Returns an `ApiError` upon failed
/// deserialization to the new type.
pub fn parse_response<T>(response: &mut Response) -> Result<T, ApiError>
where T: DeserializeOwned {
    if response.status() == StatusCode::OK {
        Ok(response.json::<T>().unwrap())
    } else {
        Err(response.json::<ApiError>().unwrap())
    }
}

