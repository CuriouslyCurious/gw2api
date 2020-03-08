use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;
use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v1/maps";

/// Struct contains a map of map objects.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Maps {
    /// HashMap of map objects.
    maps: HashMap<u32, Map>,
}

/// Struct containing information about a maps in the game, including information about floor and
/// translation data on how to translate between world coordinates and map coordinates.
#[derive(Debug, Deserialize, PartialEq, Hash)]
pub struct Map {
    /// Name of the map.
    #[serde(rename = "map_name")]
    name: String,
    /// Minimum level (height) of the map.
    min_level: u32,
    /// Maximum level of the map.
    max_level: u32,
    /// Default floor for the map.
    default_floor: u32,
    /// List of available floors.
    #[serde(default)]
    floors: Vec<u32>,
    /// id of the region this map belongs to.
    region_id: u32,
    /// Name of the region this map belongs to.
    region_name: String,
    /// id of the continent this map belongs to.
    continent_id: u32,
    /// Name of the continent this map belongs to.
    continent_name: String,
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
        parse_response(&mut client.request(&url)?)
    }

    /// Retrieve a map by its id.
    pub fn get_all(client: &Client) -> Result<Maps, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
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
    pub fn min_level(&self) -> u32 {
        self.min_level
    }

    /// Returns the maximum level of the map.
    pub fn max_level(&self) -> u32 {
        self.max_level
    }

    /// Returns the default level of the map.
    pub fn default_floor(&self) -> u32 {
        self.default_floor
    }

    /// Returns the list of available floors.
    pub fn floors(&self) -> &Vec<u32> {
        &self.floors
    }

    /// Returns the id of the region this map belongs to.
    pub fn region_id(&self) -> u32 {
        self.region_id
    }

    /// Returns the name of the region this map belongs to.
    pub fn region_name(&self) -> &str {
        &self.region_name
    }

    /// Returns the id of the continent this map belongs to.
    pub fn continent_id(&self) -> u32 {
        self.continent_id
    }

    /// Returns the name of the continent this map belongs to.
    pub fn continent_name(&self) -> &str {
        &self.continent_name
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
}
