use crate::client::Client;
use crate::error::ApiError;

use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v1/maps";

/// Struct contains a map of map objects.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Maps {
    /// HashMap of map objects.
    maps: HashMap<u32, Map>,
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
    name: String,
    /// Minimum level (height) of the map.
    min_level: i32,
    /// Maximum level of the map.
    max_level: i32,
    /// Default floor for the map.
    default_floor: i32,
    /// List of available floors.
    #[serde(default)]
    floors: Vec<i32>,
    /// The type of map.
    #[serde(default, rename = "type")]
    map_type: Option<MapType>,
    /// id of the region this map belongs to.
    region_id: Option<u32>,
    /// Name of the region this map belongs to.
    region_name: Option<String>,
    /// id of the continent this map belongs to.
    continent_id: Option<u32>,
    /// Name of the continent this map belongs to.
    continent_name: Option<String>,
    /// Dimensions of the map, given as the coordinates of the lower-left (SW) and upper-right (NE)
    /// corners.
    map_rect: Vec<(i32, i32)>,
    /// Dimensions of the map within the continent coordinate system,
    /// given as the coordinates of the lower-left (SW) and upper-right (NE) corners.
    continent_rect: Vec<(i32, i32)>,
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

    /// Returns the map of matched map objects.
    pub fn maps(&self) -> &HashMap<u32, Map> {
        &self.maps
    }
}

impl Map {
    /// Returns the name of the map.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the minimum level of the map.
    pub fn min_level(&self) -> i32 {
        self.min_level
    }

    /// Returns the maximum level of the map.
    pub fn max_level(&self) -> i32 {
        self.max_level
    }

    /// Returns the default level of the map.
    pub fn default_floor(&self) -> i32 {
        self.default_floor
    }

    /// Returns the list of available floors.
    pub fn floors(&self) -> &Vec<i32> {
        &self.floors
    }

    /// Returns the type of the map.
    pub fn map_type(&self) -> Option<&MapType> {
        self.map_type.as_ref()
    }

    /// Returns the id of the region this map belongs to.
    pub fn region_id(&self) -> Option<u32> {
        self.region_id
    }

    /// Returns the name of the region this map belongs to.
    pub fn region_name(&self) -> Option<&String> {
        self.region_name.as_ref()
    }

    /// Returns the id of the continent this map belongs to.
    pub fn continent_id(&self) -> Option<u32> {
        self.continent_id
    }

    /// Returns the name of the continent this map belongs to.
    pub fn continent_name(&self) -> Option<&String> {
        self.continent_name.as_ref()
    }

    /// Returns the dimensions of the map, given as the coordinates of the lower-left (SW)
    /// and upper-right (NE) corners.
    pub fn map_rect(&self) -> &Vec<(i32, i32)> {
        &self.map_rect
    }

    /// Returns the dimensions of the map within the continent coordinate system,
    /// given as the coordinates of the lower-left (SW) and upper-right (NE) corners.
    pub fn continent_rect(&self) -> &Vec<(i32, i32)> {
        &self.continent_rect
    }
}

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
        match serde_json::from_str::<Map>(JSON_MAP) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_map() {
        let client = Client::new();
        let id = 15;
        let map = serde_json::from_str::<Map>(JSON_MAP).unwrap();
        assert_eq!(Maps::get_id(&client, id.to_string()).unwrap().maps()[&id], map)
    }

    #[test]
    fn get_all_maps() {
        let client = Client::new();
        assert!(Maps::get_all(&client).unwrap().maps().len() >= 50)
    }
}
