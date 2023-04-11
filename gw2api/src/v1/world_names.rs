use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Struct containing an unorded list of (localized) world names.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/world_names"]
#[localised = true]
pub struct World {
    /// World id. First digit indicates the world's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the world.
    pub name: String,
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
