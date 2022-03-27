use serde::Deserialize;

use std::collections::HashMap;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/maps";

/// Struct contains a map of map objects.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Maps {
    /// HashMap of map objects.
    pub maps: HashMap<u32, Map>,
}

/// Different types of maps.
#[derive(Debug, Deserialize, PartialEq, Hash)]
pub enum MapType {
    Center,
    Instance,
    Public,
    GreenHome,
    BlueHome,
    RedHome,
    Tutorial,
    Pvp,
    JumpPuzzle,
    EdgeOfTheMists,
    Unknown,
}

/// Struct containing information about a maps in the game, including information about floor and
/// translation data on how to translate between world coordinates and map coordinates.
#[derive(Debug, Deserialize, PartialEq, Hash)]
pub struct Map {
    /// Name of the map.
    #[serde(rename = "map_name")]
    pub name: String,
    /// Minimum level (height) of the map.
    pub min_level: i32,
    /// Maximum level of the map.
    pub max_level: i32,
    /// Default floor for the map.
    pub default_floor: i32,
    /// List of available floors.
    #[serde(default)]
    pub floors: Vec<i32>,
    /// The type of map.
    #[serde(default, rename = "type")]
    pub map_type: Option<MapType>,
    /// id of the region this map belongs to.
    pub region_id: Option<u32>,
    /// Name of the region this map belongs to.
    pub region_name: Option<String>,
    /// id of the continent this map belongs to.
    pub continent_id: Option<u32>,
    /// Name of the continent this map belongs to.
    pub continent_name: Option<String>,
    /// Dimensions of the map, given as the coordinates of the lower-left (SW) and upper-right (NE)
    /// corners.
    pub map_rect: Vec<(i32, i32)>,
    /// Dimensions of the map within the continent coordinate system,
    /// given as the coordinates of the lower-left (SW) and upper-right (NE) corners.
    pub continent_rect: Vec<(i32, i32)>,
}

impl Maps {
    /// Retrieve a map by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Maps, ApiError> {
        let url = format!("{}?map_id={}", ENDPOINT_URL, id);
        client.request(&url)
    }

    /// Retrieve a map by its id.
    pub fn get_all(client: &Client) -> Result<Maps, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

impl Map {}

#[cfg(test)]
mod tests {
    use crate::v1::maps::*;
    use crate::client::Client;

    const JSON_MAP: &str = r#"
    {
      "map_name": "Queensdale",
      "min_level": 1,
      "max_level": 15,
      "default_floor": 1,
      "floors": [ 1, 3, 2, 0 ],
      "type": "Public",
      "region_id": 4,
      "region_name": "Kryta",
      "continent_id": 1,
      "continent_name": "Tyria",
      "map_rect": [
        [ -43008, -27648 ],
        [ 43008, 30720 ]
      ],
      "continent_rect": [
        [ 9856, 11648 ],
        [ 13440, 14080 ]
      ]
    }"#;

    #[test]
    fn create_map() {
        serde_json::from_str::<Map>(JSON_MAP).unwrap();
    }

    #[test]
    fn get_map() {
        let client = Client::new();
        let id = 15;
        let map = serde_json::from_str::<Map>(JSON_MAP).unwrap();
        assert_eq!(Maps::get_id(&client, id.to_string()).unwrap().maps[&id], map)
    }

    #[test]
    fn get_all_maps() {
        let client = Client::new();
        assert!(Maps::get_all(&client).unwrap().maps.len() >= 50)
    }
}
