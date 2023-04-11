use serde::Deserialize;
use gw2api_derive::Endpoint;
use std::collections::HashMap;

/// Struct containing a hashmap of all dyes in the game.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/colors"]
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

impl Dye {}

impl Material {}

#[cfg(test)]
mod tests {
    use crate::v1::colors::*;

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
}
