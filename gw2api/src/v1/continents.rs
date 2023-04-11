use serde::Deserialize;

use gw2api_derive::Endpoint;

use std::collections::HashMap;

/// Struct containing a hashmap of all continents in the game.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/continents"]
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
    pub floors: Vec<i8>,
}

impl Continent {}

#[cfg(test)]
mod tests {
    use crate::v1::continents::*;

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
}
