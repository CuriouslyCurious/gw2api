use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{parse_response, Team};

const ENDPOINT_URL: &str = "/v1/wvw/match_details";

/// Contains information about a specified WvW match, including the total score and further details
/// for each map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Match {
    /// id of the WvW match.
    #[serde(rename = "match_id")]
    id: String,
    /// List of the three total scores (order: red, blue, green).
    scores: Vec<u32>,
    /// List of objects containing information about each of the four WvW maps.
    maps: Vec<Map>,
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
    id: u32,
    /// Current owner of the objective.
    owner: Team,
    /// The guild id of the guild currently claiming the objective. If the objective is not claimed
    /// this property is missing.
    #[serde(default)]
    owner_guild: String,
}

/// Describes a bonus given by a map and its current owner.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Bonus {
    /// Shorthand name for the bonus.
    #[serde(rename = "type")]
    bonus_type: String,
    /// Current owner of the bonus. Neutral-owned bonuses are not listed.
    owner: Team,
}

/// Struct containing information about a WvW map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
    /// Identifier for the map.
    #[serde(rename = "type")]
    map_type: MapType,
    /// List of the three scores for this map (order: red, blue, green).
    scores: Vec<u32>,
    /// List of objectives for this map.
    #[serde(default)]
    objectives: Vec<Objective>,
    /// List of bonuses granted by this map. If no team owns a bonus from this map this is empty.
    #[serde(default)]
    bonuses: Vec<Bonus>,
}

impl Match {
    /// Retrieve a match by its id.
    pub fn get_by_id(client: &Client, id: String) -> Result<Match, ApiError> {
        let url = format!("{}?match_id={}", ENDPOINT_URL, id);
        parse_response(&mut client.request(&url)?)
    }

    /// Returns the id of the match.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns a list of the scores of the map.
    pub fn scores(&self) -> &Vec<u32> {
        &self.scores
    }

    /// Returns a list of objects describing the maps of the match.
    pub fn maps(&self) -> &Vec<Map> {
        &self.maps
    }
}

impl Objective {
    /// Returns the id of the objective.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the current owner of the objective.
    pub fn owner(&self) -> &Team {
        &self.owner
    }

    /// Returns the guild id of the guild currently claiming the objective.
    pub fn owner_guild(&self) -> &str {
        &self.owner_guild
    }
}

impl Bonus {
    /// Returns the shorthand for the bonus type.
    pub fn bonus_type(&self) -> &str {
        &self.bonus_type
    }

    /// Returns the current owner of the bonus. Neutral-owned bonuses are not listsed.
    pub fn owner(&self) -> &Team {
        &self.owner
    }
}

impl Map {
    /// Returns the identifier of the map.
    pub fn map_type(&self) -> &MapType {
        &self.map_type
    }

    /// Returns a list of scores for this map (order: red, blue, green).
    pub fn scores(&self) -> &Vec<u32> {
        &self.scores
    }

    /// Returns a list of objects with information about ownership of objectives on a map.
    pub fn objectives(&self) -> &Vec<Objective> {
        &self.objectives
    }

    /// Returns a list of bonuses granted by the map. If no team owns a bonus from this map this is
    /// empty.
    pub fn bonuses(&self) -> &Vec<Bonus> {
        &self.bonuses
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::wvw::match_details::*;
    use crate::client::Client;

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
        match serde_json::from_str::<Match>(JSON_MATCH) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_match_by_id() {
        let client = Client::new();
        let id = "1-4";
        assert_eq!(Match::get_by_id(&client, id.to_string()).unwrap().id(), id)
    }
}
