use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;
use crate::utils::Race;

const ENDPOINT_URL: &str = "/v1/skin_details";

/// Struct containing information about a specified skin.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Skin {
    /// id of the skin.
    #[serde(rename = "skin_id")]
    pub id: String,
    /// Name of the skin.
    pub name: String,
    /// Type of the skin.
    #[serde(rename = "type")]
    pub skin_type: SkinType,
    /// Additional flags for a skin.
    #[serde(default)]
    pub flags: Vec<SkinFlags>,
    /// Race restrictions that apply to a skin.
    #[serde(default)]
    pub restrictions: Vec<Race>,
    /// File icon id to be used with the render service.
    pub icon_file_id: String,
    /// File signature to be used with the render service.
    pub icon_file_signature: String,
    /// Optional skin description.
    #[serde(default)]
    pub description: String,
}

/// Possible skin types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum SkinType {
    Armor,
    Weapon,
    Back,
}

/// Flags for additional information about a skin.
#[derive(Debug, Deserialize, PartialEq)]
pub enum SkinFlags {
    ShowInWardrobe,
    NoCost,
    HideIfLocked,
}

impl Skin {
    /// Retrieve a skin by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Skin, ApiError> {
        let url = format!("{}?skin_id={}", ENDPOINT_URL, id);
        client.request(&url)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::skin_details::*;
    use std::fs::read_to_string;

    const JSON_SKIN_PATH: &str = "./tests/json/v1/skin_details.json";

    #[test]
    fn create_skin() {
        let json = read_to_string(JSON_SKIN_PATH).unwrap();
        serde_json::from_str::<Skin>(&json).unwrap();
    }
}
