use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{parse_response, Race};

const ENDPOINT_URL: &str = "/v1/skin_details";

/// Struct containing information about a specified skin.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Skin {
    /// id of the skin.
    #[serde(rename = "skin_id")]
    id: String,
    /// Name of the skin.
    name: String,
    /// Type of the skin.
    #[serde(rename = "type")]
    skin_type: SkinType,
    /// Additional flags for a skin.
    #[serde(default)]
    flags: Vec<SkinFlags>,
    /// Race restrictions that apply to a skin.
    #[serde(default)]
    restrictions: Vec<Race>,
    /// File icon id to be used with the render service.
    icon_file_id: String,
    /// File signature to be used with the render service.
    icon_file_signature: String,
    /// Optional skin description.
    #[serde(default)]
    description: String,
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
        parse_response(&mut client.request(&url)?)
    }

    /// Returns the id of the skin.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the skin.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the skin.
    pub fn skin_type(&self) -> &SkinType {
        &self.skin_type
    }

    /// Returns additional flags for the skin.
    pub fn flags(&self) -> &Vec<SkinFlags> {
        &self.flags
    }

    /// Returns potential race restrictions for the skin.
    pub fn restrictions(&self) -> &Vec<Race> {
        &self.restrictions
    }

    /// Returns icon file id used with the render service.
    pub fn icon_file_id(&self) -> &str {
        &self.icon_file_id
    }

    /// Returns icon file signature used with the render service.
    pub fn icon_file_signature(&self) -> &str {
        &self.icon_file_signature
    }

    /// Returns the potential description for the skin.
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::skin_details::*;
    use crate::client::Client;

    const JSON_SKIN: &str = r#"
    {
      "skin_id": "1350",
      "name": "Zodiac Light Vest",
      "type": "Armor",
      "flags": [
        "ShowInWardrobe"
      ],
      "restrictions": [],
      "icon_file_id": "740312",
      "icon_file_signature": "021048C317DFFFB6727E0955A2D6C7EFFBE9425B",
      "armor": {
        "type": "Coat",
        "weight_class": "Light"
      }
    }"#;

    #[test]
    fn create_skin() {
        match serde_json::from_str::<Skin>(JSON_SKIN) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_skin() {
        let client = Client::new();
        let skin = serde_json::from_str::<Skin>(JSON_SKIN).unwrap();
        assert_eq!(Skin::get_id(&client, "1350".to_string()).unwrap(), skin)
    }
}
