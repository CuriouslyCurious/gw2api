use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;

const ENDPOINT_URL: &str = "/v1/guild_details";

/// Contains information about a guild.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Guild {
    /// id of the guild.
    #[serde(rename = "guild_id")]
    id: String,
    /// Name of the guild.
    #[serde(rename = "guild_name")]
    name: String,
    /// Tag (abbreviation or shortened name) of the guild.
    tag: String,
    /// Potential object containing information about the emblem design of the guild.
    #[serde(default)]
    emblem: Option<Emblem>,
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
    background_id: u32,
    /// id of the foreground image.
    foreground_id: u32,
    /// List of flags describing orientation of background/foreground elements of the emblem.
    flags: Vec<Flag>,
    /// id of the background color (see v1/colors).
    background_color_id: u32,
    /// id of the primary foreground color (see v1/colors).
    foreground_primary_color_id: u32,
    /// id of the secondary foreground color (see v1/colors).
    foreground_secondary_color_id: u32,
}

impl Guild {
    /// Retrieve a guild by its id.
    pub fn get_by_id(client: &Client, id: String) -> Result<Guild, ApiError> {
        let url = format!("{}?guild_id={}", ENDPOINT_URL, id);
        parse_response(&mut client.request(&url)?)
    }

    /// Retrieve a guild by its name.
    pub fn get_by_name(client: &Client, name: String) -> Result<Guild, ApiError> {
        let url = format!("{}?guild_name={}", ENDPOINT_URL, name);
        parse_response(&mut client.request(&url)?)
    }

    /// Returns the id of the guild.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the guild.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the tag of the guild.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Returns the emblem object.
    pub fn emblem(&self) -> &Option<Emblem> {
        &self.emblem
    }
}

impl Emblem {
    /// Returns the id of the background image.
    pub fn background_id(&self) -> u32 {
        self.background_id
    }

    /// Returns the id of the foreground image.
    pub fn foreground_id(&self) -> u32 {
        self.background_id
    }

    /// Returns a list for flags for the emblem.
    pub fn flags(&self) -> &Vec<Flag> {
        &self.flags
    }

    /// Returns the id of the background color.
    pub fn background_color_id(&self) -> u32 {
        self.background_color_id
    }

    /// Returns the id of the primary foreground color.
    pub fn foreground_primary_color_id(&self) -> u32 {
        self.foreground_primary_color_id
    }

    /// Returns the id of the secondary foreground color.
    pub fn background_primary_color_id(&self) -> u32 {
        self.foreground_secondary_color_id
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::guild_details::*;
    use crate::client::Client;

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
        match serde_json::from_str::<Guild>(JSON_GUILD) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_guild_by_id() {
        let client = Client::new();
        let id = "1C4EE62A-E76D-48E0-A205-D56CCC4FED2D"; // id of the lovely guild TD
        assert_eq!(Guild::get_by_id(&client, id.to_string()).unwrap().id(), id)
    }

    #[test]
    fn get_guild_by_name() {
        let client = Client::new();
        let name = "The Doppelgangers";
        assert_eq!(Guild::get_by_name(&client, name.to_string()).unwrap().name(), name)
    }
}
