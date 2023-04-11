use serde::Deserialize;

use gw2api_core::utils::Race;
use gw2api_derive::ParamEndpoint;

/// Struct containing information about a specified skin.
#[derive(ParamEndpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/skin_details?skin_id={}"]
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
