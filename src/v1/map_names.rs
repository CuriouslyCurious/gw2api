use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;

const ENDPOINT_URL: &str = "/v1/map_names";

/// Struct containing an unorded list of (localized) map names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
    /// Map id. First digit indicates the map's region: 1 = North America, 2 = Europe.
    id: String,
    /// Localized name of the map.
    name: String,
}

impl Map {
    /// Retrieve all map names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<Map>, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
    }

    /// Returns the id of the map.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the localized name of the map.
    pub fn name(&self) -> &str {
        &self.name
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
        match serde_json::from_str::<Map>(JSON_MAP) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_maps() {
        let client = Client::new();
        let map = serde_json::from_str::<Map>(JSON_MAP).unwrap();
        assert!(Map::get_all(&client).unwrap().contains(&map))
    }
}
