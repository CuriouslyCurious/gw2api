use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/guild_details";

/// Contains information about a guild.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Guild {
    /// id of the guild.
    #[serde(rename = "guild_id")]
    pub id: String,
    /// Name of the guild.
    #[serde(rename = "guild_name")]
    pub name: String,
    /// Tag (abbreviation or shortened name) of the guild.
    pub tag: String,
    /// Potential object containing information about the emblem design of the guild.
    #[serde(default)]
    pub emblem: Option<Emblem>,
}

/// Possible flags describing the orientation of the background and/or foreground of the emblem.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Flag {
    FlipBackgroundHorizontal,
    FlipBackgroundVertical,
    FlipForegroundHorizontal,
    FlipForegroundVertical,
}

/// Struct containing information about a guild's emblem.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Emblem {
    /// id of the background image.
    pub background_id: u32,
    /// id of the foreground image.
    pub foreground_id: u32,
    /// List of flags describing orientation of background/foreground elements of the emblem.
    pub flags: Vec<Flag>,
    /// id of the background color (see v1/colors).
    pub background_color_id: u32,
    /// id of the primary foreground color (see v1/colors).
    pub foreground_primary_color_id: u32,
    /// id of the secondary foreground color (see v1/colors).
    pub foreground_secondary_color_id: u32,
}

impl Guild {
    /// Retrieve a guild by its id.
    pub fn get_by_id(client: &Client, id: String) -> Result<Guild, ApiError> {
        let url = format!("{}?guild_id={}", ENDPOINT_URL, id);
        client.request(&url)
    }

    /// Retrieve a guild by its name.
    pub fn get_by_name(client: &Client, name: String) -> Result<Guild, ApiError> {
        let url = format!("{}?guild_name={}", ENDPOINT_URL, name);
        client.request(&url)
    }
}

impl Emblem {}

#[cfg(test)]
mod tests {
    use crate::v1::guild_details::*;

    const JSON_GUILD: &str = r#"
    {
      "guild_id": "75FD83CF-0C45-4834-BC4C-097F93A487AF",
      "guild_name": "Veterans Of Lions Arch",
      "tag": "LA",
      "emblem": {
        "background_id": 27,
        "foreground_id": 114,
        "flags": [],
        "background_color_id": 11,
        "foreground_primary_color_id": 584,
        "foreground_secondary_color_id": 64
      }
    }"#;

    #[test]
    fn create_guild() {
        serde_json::from_str::<Guild>(JSON_GUILD).unwrap();
    }
}
