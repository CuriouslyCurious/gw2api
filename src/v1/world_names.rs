use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/world_names";

/// Struct containing an unorded list of (localized) world names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct World {
    /// World id. First digit indicates the world's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the world.
    pub name: String,
}

impl World {
    /// Retrieve all world names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<World>, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::world_names::*;

    const JSON_WORLD: &str = r#"
    {
      "id": "2014",
      "name": "Gunnar's Hold"
    }"#;

    #[test]
    fn create_world() {
        serde_json::from_str::<World>(JSON_WORLD).unwrap();
    }
}
