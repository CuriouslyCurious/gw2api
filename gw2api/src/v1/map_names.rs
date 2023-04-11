use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Struct containing an unorded list of (localized) map names.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/map_names"]
pub struct MapName {
    /// Map id. First digit indicates the map's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the map.
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::v1::map_names::*;

    const JSON_MAP: &str = r#"
    {
      "id": "15",
      "name": "Queensdale"
    }"#;

    #[test]
    fn create_map() {
        serde_json::from_str::<MapName>(JSON_MAP).unwrap();
    }
}
