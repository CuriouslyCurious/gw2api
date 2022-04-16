use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;
use crate::utils::Team;

const ENDPOINT_URL: &str = "/v1/wvw/match_details";

/// Contains information about a specified WvW match, including the total score and further details
/// for each map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Match {
    /// id of the WvW match.
    #[serde(rename = "match_id")]
    pub id: String,
    /// List of the three total scores (order: red, blue, green).
    pub scores: Vec<u32>,
    /// List of objects containing information about each of the four WvW maps.
    pub maps: Vec<Map>,
}

/// Possible map types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum MapType {
    /// Borderlands
    RedHome,
    GreenHome,
    BlueHome,
    /// Eternal Battlegrounds
    Center,
}

/// Contains information about who owns an objective.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Objective {
    /// Objective id.
    pub id: u32,
    /// Current owner of the objective.
    pub owner: Team,
    /// The guild id of the guild currently claiming the objective. If the objective is not claimed
    /// this property is missing.
    #[serde(default)]
    pub owner_guild: String,
}

/// Describes a bonus given by a map and its current owner.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Bonus {
    /// Shorthand name for the bonus.
    #[serde(rename = "type")]
    pub bonus_type: String,
    /// Current owner of the bonus. Neutral-owned bonuses are not listed.
    pub owner: Team,
}

/// Struct containing information about a WvW map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
    /// Identifier for the map.
    #[serde(rename = "type")]
    pub map_type: MapType,
    /// List of the three scores for this map (order: red, blue, green).
    pub scores: Vec<u32>,
    /// List of objectives for this map.
    #[serde(default)]
    pub objectives: Vec<Objective>,
    /// List of bonuses granted by this map. If no team owns a bonus from this map this is empty.
    #[serde(default)]
    pub bonuses: Vec<Bonus>,
}

impl Match {
    /// Retrieve a match by its id.
    pub fn get_by_id(client: &Client, id: String) -> Result<Match, ApiError> {
        let url = format!("{}?match_id={}", ENDPOINT_URL, id);
        client.request(&url)
    }
}

impl Objective {}

impl Bonus {}

impl Map {}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::v1::wvw::match_details::*;

    const JSON_MATCH: &str = r#"
    {
      "match_id": "1-4",
      "scores": [ 155502, 137176, 189824 ],
      "maps": [
        {
          "type": "RedHome",
          "scores": [ 80148, 7022, 18582 ],
          "objectives": [
            { "id": 32, "owner": "Red", "owner_guild": "277CCE76-6254-4CF2-8A2D-15A30B7110BD" },
            { "id": 35, "owner": "Green" }
          ],
          "bonuses": [
            { "type": "bloodlust", "owner": "Blue" }
          ]
        },
        {
          "type": "GreenHome",
          "scores": [ 13285, 7465, 82817 ],
          "objectives": [
          ],
          "bonuses": [
            { "type": "bloodlust", "owner": "Green" }
          ]
        },
        {
          "type": "BlueHome",
          "scores": [ 18592, 54837, 29013 ],
          "objectives": [
          ],
          "bonuses": [
          ]
        },
        {
          "type": "Center",
          "scores": [ 43477, 67852, 59412 ],
          "objectives": [
          ],
          "bonuses": [
          ]
        }
      ]
    }"#;

    #[test]
    fn create_match() {
        serde_json::from_str::<Match>(JSON_MATCH).unwrap();
    }

    #[test]
    fn get_match_by_id() {
        let client = Client::new();
        let id = "1-4";
        assert_eq!(Match::get_by_id(&client, id.to_string()).unwrap().id, id)
    }
}
