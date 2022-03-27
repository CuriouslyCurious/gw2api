use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/map_names";

/// Struct containing an unorded list of (localized) map names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
    /// Map id. First digit indicates the map's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the map.
    pub name: String,
}

impl Map {
    /// Retrieve all map names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<Map>, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::map_names::*;
    use crate::client::Client;

    const JSON_MAP: &str = r#"
    {
      "id": "15",
      "name": "Queensdale"
    }"#;

    #[test]
    fn create_map() {
        serde_json::from_str::<Map>(JSON_MAP).unwrap();
    }

    #[test]
    fn get_all_maps() {
        let client = Client::new();
        let map = serde_json::from_str::<Map>(JSON_MAP).unwrap();
        assert!(Map::get_all(&client).unwrap().contains(&map))
    }
}
