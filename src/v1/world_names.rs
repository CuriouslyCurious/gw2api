use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;

const ENDPOINT_URL: &str = "/v1/world_names";

/// Struct containing an unorded list of (localized) world names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct World {
    /// World id. First digit indicates the world's region: 1 = North America, 2 = Europe.
    id: String,
    /// Localized name of the world.
    name: String,
}

impl World {
    /// Retrieve all world names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<World>, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
    }

    /// Returns the id of the world.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the localized name of the world.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::world_names::*;
    use crate::client::Client;

    const JSON_WORLD: &str = r#"
    {
      "id": "2014",
      "name": "Gunnar's Hold"
    }"#;

    #[test]
    fn create_world() {
        match serde_json::from_str::<World>(JSON_WORLD) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_worlds() {
        let client = Client::new();
        let world = serde_json::from_str::<World>(JSON_WORLD).unwrap(); // The best world?
        assert!(World::get_all(&client).unwrap().contains(&world))
    }
}
