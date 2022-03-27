use serde::Deserialize;

use std::collections::HashMap;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/continents";

/// Struct containing a hashmap of all continents in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Continents {
    pub continents: HashMap<u32, Continent>,
}

/// Contains information about a continent, including the localised name, dimensions, minimum and
/// maximal zoom levels, and a list of floors.
/// Note: there are only two continents, Tyria and Mists.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Continent {
    /// Name of the continent.
    pub name: String,
    /// Dimensions of the continent.
    #[serde(rename = "continent_dims")]
    pub dimensions: Vec<u32>,
    /// Minimum zoom level.
    pub min_zoom: u8,
    /// Maximum zoom level.
    pub max_zoom: u8,
    /// A list of floors available.
    pub floors: Vec<i8>
}

impl Continents {
    /// Retrieve all continents that are in the game.
    pub fn get_all_continents(client: &Client) -> Result<Continents, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

impl Continent {}

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
        serde_json::from_str::<Continent>(JSON_CONTINENT).unwrap();
    }

    #[test]
    fn get_all_continents() {
        let client = Client::new();
        // Currently only 2 continents in the game
        assert!(Continents::get_all_continents(&client).unwrap().continents.len() >= 2)
    }
}
