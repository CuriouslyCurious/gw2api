use serde::Deserialize;

use std::collections::HashMap;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/colors";

/// Struct containing a hashmap of all dyes in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Colors {
    pub colors: HashMap<u32, Dye>,
}

/// Contains information about a dye, including localised names and their colour component
/// information.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Dye {
    /// Name of the dye.
    pub name: String,
    /// The base RGB values.
    #[serde(rename = "base_rgb")]
    pub rgb: Vec<u8>,
    /// Information on the dye's appearance when applied on *cloth* armor.
    pub cloth: Material,
    /// Information on the dye's appearance when applied on *leather* armor.
    pub leather: Material,
    /// Information on the dye's appearance when applied on *metal* armor.
    pub metal: Material,
    /// (Optional) Information on the dye's appearance when applied on *fur* armor.
    #[serde(default)]
    pub fur: Material,
    /// (Optional) Item ID of the dye.
    #[serde(default, rename = "item")]
    pub item_id: u32,
    /// The potential categories the dye belongs to, including colour-family, type of material and
    /// rarity
    #[serde(default)]
    pub categories: Vec<String>,
}

/// Struct that contains the offense, defense and speed stats for a given hero.
#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Material {
    /// The brightness.
    pub brightness: i32,
    /// The contrast.
    pub contrast: f32,
    /// The hue in HSL (hue, saturation, lightness) colourspace.
    pub hue: u32,
    /// The saturation.
    pub saturation: f32,
    /// The lightness.
    pub lightness: f32,
    /// A list of precalculated RGB values.
    pub rgb: Vec<u8>,
}

impl Colors {
    /// Retrieve all dyes that are in the game.
    pub fn get_all_dyes(client: &Client) -> Result<Colors, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

impl Dye {}

impl Material {}

#[cfg(test)]
mod tests {
    use crate::v1::colors::*;
    use crate::client::Client;

    const JSON_DYE: &str = r#"
    {
      "name": "Black",
      "base_rgb": [
        128,
        26,
        26
      ],
      "cloth": {
        "brightness": -13,
        "contrast": 1,
        "hue": 275,
        "saturation": 0.0234375,
        "lightness": 1.09375,
        "rgb": [
          37,
          35,
          38
        ]
      },
      "leather": {
        "brightness": -13,
        "contrast": 1,
        "hue": 275,
        "saturation": 0.0234375,
        "lightness": 1.09375,
        "rgb": [
          37,
          35,
          38
        ]
      },
      "metal": {
        "brightness": -13,
        "contrast": 1,
        "hue": 275,
        "saturation": 0.0234375,
        "lightness": 1.09375,
        "rgb": [
          37,
          35,
          38
        ]
      },
      "fur": {
        "brightness": -13,
        "contrast": 1,
        "hue": 275,
        "saturation": 0.0234375,
        "lightness": 1.09375,
        "rgb": [
          37,
          35,
          38
        ]
      },
      "item": 20358,
      "categories": [
        "Gray",
        "Metal",
        "Rare"
      ]
    }"#;

    #[test]
    fn create_dye() {
        serde_json::from_str::<Dye>(JSON_DYE).unwrap();
    }

    #[test]
    fn get_all_dyes() {
        let client = Client::new();
        let dye_num = 2; // Black dye
        let dye = serde_json::from_str::<Dye>(JSON_DYE).unwrap();
        let colors = Colors::get_all_dyes(&client).unwrap();
        assert_eq!(colors.colors.get(&dye_num).unwrap(), &dye)
    }
}
