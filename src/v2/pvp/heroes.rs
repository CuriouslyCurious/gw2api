use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{ids_to_string, parse_response};

const ENDPOINT_URL: &str = "/v2/pvp/heroes";

/// A hero used in the Stronghold game structured PvP game type.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Hero {
    /// id of the hero.
    id: String,
    /// Name of the hero.
    name: String,
    /// Flavor text describing the lore behind the hero.
    description: String,
    /// Flavor type describing the hero.
    #[serde(rename = "type")]
    flavor_type: String,
    /// A struct containing the champion's stats: offense, defense and speed.
    stats: Stats,
    /// Url to the overlay art for the champion.
    #[serde(rename = "overlay")]
    overlay_url: String,
    /// Url to the underlay art for the champion.
    #[serde(rename = "underlay")]
    underlay_url: String,
    /// A `Vec` of the skins available to the given hero.
    skins: Vec<Skin>,
}

/// Struct that contains the offense, defense and speed stats for a given hero.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Stats {
    offense: u32,
    defense: u32,
    speed: u32,
}

/// Cosmetic skin information of a hero.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Skin {
    /// Skin id
    id: u32,
    /// Name of the skin
    name: String,
    /// Url to the icon
    #[serde(rename = "icon")]
    icon_url: String,
    /// Whether the skin is the default for that hero or not.
    default: bool,
    /// Item ids which unlock the skin.
    #[serde(default)]
    unlock_items: Vec<u32>,
}

impl Hero {
    /// Retrieve a hero by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Hero, ApiError> {
        let url = format!("{}?id={}", ENDPOINT_URL, id);
        parse_response(&mut client.request(&url)?)
    }

    /// Retrieve all heroes that are available.
    pub fn get_all_heroes(client: &Client) -> Result<Vec<Hero>, ApiError> {
        let url = format!("{}?ids=all", ENDPOINT_URL);
        parse_response(&mut client.request(&url)?)
    }

    /// Retrive heroes by their ids.
    pub fn get_heroes_by_ids(client: &Client, ids: Vec<String>) -> Result<Vec<Hero>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        parse_response(&mut client.authenticated_request(&url)?)
    }

    /// Returns the requested id of the hero.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the hero.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the flavor description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the flavor type, e.g "Specialist Hero".
    pub fn flavor_type(&self) -> &str {
        &self.flavor_type
    }

    /// Returns the stats of the given hero.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Returns the url to the overlay art for the hero.
    pub fn overlay_url(&self) -> &str {
        &self.overlay_url
    }

    /// Returns the url to the underlay art for the hero.
    pub fn underlay_url(&self) -> &str {
        &self.underlay_url
    }

    /// Returns a `Vec` containing the available skins for the given hero.
    pub fn skins(&self) -> &Vec<Skin> {
        &self.skins
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::pvp::heroes::{Hero, Stats, Skin};
    use crate::client::Client;
    use crate::error::ApiError;

    #[test]
    fn create_hero() {
        let json_hero = r#"
        {
            "id": "115C140F-C2F5-40EB-8EA2-C3773F2AE468",
            "name": "Nika",
            "description": "Nika was a proficient assassin schooled in her youth at Shing Jea Monastery. She served Cantha as a member of the Obsidian Flame.",
            "type": "Specialist Hero",
            "stats": {
                "offense": 3,
                "defense": 2,
                "speed": 4
            },
            "overlay": "https://render.guildwars2.com/file/2CACF4120E370D1997A4C3D69BF592D7CC1870C8/993693.png",
            "underlay": "https://render.guildwars2.com/file/103108E0D8EDD22C577FA4171618D004A82AD955/993694.png",
            "skins": [
                {
                    "id": 1,
                    "name": "Nika",
                    "icon": "https://render.guildwars2.com/file/4602BDC15B73422011AC664425D93750707F04F3/1058576.png",
                    "default": true
                },
                {
                    "id": 7,
                    "name": "Shadow Assassin Nika",
                    "icon": "https://render.guildwars2.com/file/01643F1BD1202007BEE8E37F7DA3EA31AEE9536C/1322841.png",
                    "default": false
                }
            ]
        }"#;

        match serde_json::from_str::<Hero>(json_hero) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }
}

