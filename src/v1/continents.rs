use crate::client::Client;
use crate::error::ApiError;

use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v1/continents";

/// Struct containing a hashmap of all continents in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Continents {
    continents: HashMap<u32, Continent>,
}

/// Contains information about a continent, including the localised name, dimensions, minimum and
/// maximal zoom levels, and a list of floors.
/// Note: there are only two continents, Tyria and Mists.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Continent {
    /// Name of the continent.
    name: String,
    /// Dimensions of the continent.
    #[serde(rename = "continent_dims")]
    dimensions: Vec<u32>,
    /// Minimum zoom level.
    min_zoom: u8,
    /// Maximum zoom level.
    max_zoom: u8,
    /// A list of floors available.
    floors: Vec<i8>
}

impl Continents {
    /// Retrieve all continents that are in the game.
    pub fn get_all_continents(client: &Client) -> Result<Continents, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Returns the hashmap containing all the continents.
    pub fn continents(&self) -> &HashMap<u32, Continent> {
        &self.continents
    }
}

impl Continent {
    /// Returns the name of the continent.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the dimensions of the continent.
    pub fn dimensions(&self) -> &Vec<u32> {
        &self.dimensions
    }

    /// Returns the minimum zoom level.
    pub fn min_zoom(&self) -> u8 {
        self.min_zoom
    }

    /// Returns the maximum zoom level.
    pub fn max_zoom(&self) -> u8 {
        self.max_zoom
    }

    /// Returns a list of the floors of the continent.
    pub fn metal(&self) -> &Vec<i8> {
        &self.floors
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::continents::*;
    use crate::client::Client;

    const JSON_CONTINENT: &str = r#"
    {
      "name": "Tyria",
      "continent_dims": [ 32768, 32768 ],
      "min_zoom": 0,
      "max_zoom": 7,
      "floors": [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
          18, 19, 30, -2, -3, -4, -5, -6, -7, -8, -10, -11, -15, -16, -17, 38,
          20, 21, 22, 23, 24, 25, 26, 27, 34, 36, 37 ]
    }"#;

    #[test]
    fn create_continent() {
        match serde_json::from_str::<Continent>(JSON_CONTINENT) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_continents() {
        let client = Client::new();
        // Currently only 2 continents in the game
        assert!(Continents::get_all_continents(&client).unwrap().continents().len() >= 2)
    }
}
