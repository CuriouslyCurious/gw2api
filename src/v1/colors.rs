use crate::client::Client;
use crate::error::ApiError;

use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v1/colors";

/// Struct containing a hashmap of all dyes in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Colors {
    colors: HashMap<u32, Dye>,
}

/// Contains information about a dye, including localised names and their colour component
/// information.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Dye {
    /// Name of the dye.
    name: String,
    /// The base RGB values.
    #[serde(rename = "base_rgb")]
    rgb: Vec<u8>,
    /// Information on the dye's appearance when applied on *cloth* armor.
    cloth: Material,
    /// Information on the dye's appearance when applied on *leather* armor.
    leather: Material,
    /// Information on the dye's appearance when applied on *metal* armor.
    metal: Material,
    /// (Optional) Information on the dye's appearance when applied on *fur* armor.
    #[serde(default)]
    fur: Material,
    /// (Optional) Item ID of the dye.
    #[serde(default, rename = "item")]
    item_id: u32,
    /// The potential categories the dye belongs to, including colour-family, type of material and
    /// rarity
    #[serde(default)]
    categories: Vec<String>,
}

/// Struct that contains the offense, defense and speed stats for a given hero.
#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Material {
    /// The brightness.
    brightness: i32,
    /// The contrast.
    contrast: f32,
    /// The hue in HSL (hue, saturation, lightness) colourspace.
    hue: u32,
    /// The saturation.
    saturation: f32,
    /// The lightness.
    lightness: f32,
    /// A list of precalculated RGB values.
    rgb: Vec<u8>,
}

impl Colors {
    /// Retrieve all dyes that are in the game.
    pub fn get_all_dyes(client: &Client) -> Result<Colors, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Returns the hashmap containing all the dyes.
    pub fn colors(&self) -> &HashMap<u32, Dye> {
        &self.colors
    }
}

impl Dye {
    /// Returns the name of the dye.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the base RGB values for the dye.
    pub fn rgb(&self) -> &Vec<u8> {
        &self.rgb
    }

    /// Returns the struct containing the information on the dye's appearance on *cloth* armor.
    pub fn cloth(&self) -> &Material {
        &self.cloth
    }

    /// Returns the struct containing the information on the dye's appearance on *leather* armor.
    pub fn leather(&self) -> &Material {
        &self.leather
    }

    /// Returns the struct containing the information on the dye's appearance on *metal* armor.
    pub fn metal(&self) -> &Material {
        &self.metal
    }

    /// Returns the struct containing the information on the dye's appearance on *fur* armor.
    pub fn fur(&self) -> &Material {
        &self.fur
    }

    /// Returns the item ID of the dye.
    pub fn item_id(&self) -> u32 {
        self.item_id
    }

    /// Returns the categories the dye belongs to: colour-family, type of material and
    /// rarity (can be empty).
    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }
}

impl Material {
    /// Returns the brightness of the material.
    pub fn brightness(&self) -> i32 {
        self.brightness
    }

    /// Returns the contrast of the material.
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    /// Returns the hue of the material.
    pub fn hue(&self) -> u32 {
        self.hue
    }

    /// Returns the saturation of the material.
    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    /// Returns the lightness of the material.
    pub fn lightness(&self) -> f32 {
        self.lightness
    }

    /// Returns a list of precalculated RGB values for the material.
    pub fn rgb(&self) -> &Vec<u8> {
        &self.rgb
    }
}

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
        match serde_json::from_str::<Dye>(JSON_DYE) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_dyes() {
        let client = Client::new();
        let dye_num = 2; // Black dye
        let dye = serde_json::from_str::<Dye>(JSON_DYE).unwrap();
        let colors = Colors::get_all_dyes(&client).unwrap();
        assert_eq!(colors.colors().get(&dye_num).unwrap(), &dye)
    }
}
