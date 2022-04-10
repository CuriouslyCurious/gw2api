use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/map_names";

/// Struct containing an unorded list of (localized) map names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct MapName {
    /// Map id. First digit indicates the map's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the map.
    pub name: String,
}

impl MapName {
    /// Retrieve all map names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<MapName>, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::map_names::*;

    const JSON_MAP: &str = r#"
    {
      "id": "15",
      "name": "Queensdale"
    }"#;

    #[test]
    fn create_map() {
        serde_json::from_str::<MapName>(JSON_MAP).unwrap();
    }
}
