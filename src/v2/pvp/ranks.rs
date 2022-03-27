use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;
use crate::utils::ids_to_string;

const ENDPOINT_URL: &str = "/v2/pvp/ranks";

/// Information about a structured PvP rank.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Rank {
    /// id of the PvP rank.
    pub id: u32,
    /// id of the unlocked finisher corresponding to the rank.
    pub finisher_id: u32,
    /// Given name of the PvP rank.
    pub name: String,
    /// Icon url for the PvP rank.
    #[serde(rename = "icon")]
    pub icon_url: String,
    /// The minimum PvP level required to be this rank.
    pub min_rank: u32,
    /// The maximum PvP level required to be this rank.
    pub max_rank: u32,
    /// Span of levels which the rank covers, also contains the PvP experience points needed to go
    /// from the minimum rank to the maximum rank.
    /// TODO: Make this not be an unnecessary Vec.
    #[serde(flatten)]
    pub levels: Levels,
}

/// Contains the span of PvP levels a certain rank covers, as well as the required amount of PvP
/// experience needed to go from the minimum rank to the maximum rank in that PvP rank.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Levels {
    /// The minimum PvP level required to be this rank.
    min_rank: u32,
    /// The maximum PvP level required to be this rank.
    max_rank: u32,
    /// Points needed to go from the given minimum rank to maximum rank.
    points: u32,
}

impl Rank {
    /// Retrieve all available rank ids.
    pub fn get_all_ids(client: &Client) -> Result<u32, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Retrieve a certain rank's information by its id.
    pub fn get_id(client: &Client, id: u32) -> Result<Rank, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, id);
        client.request(&url)
    }

    /// Retrieve all ranks.
    pub fn get_all_ranks(client: &Client) -> Result<Vec<Rank>, ApiError> {
        let url = format!("{}?ids=all", ENDPOINT_URL);
        client.request(&url)
    }

    /// Retrive ranks' information by their ids.
    pub fn get_ranks_by_ids(client: &Client, ids: Vec<String>) -> Result<Vec<Rank>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        client.request(&url)
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::pvp::ranks::Rank;
    //use crate::client::Client;

    #[test]
    fn create_rank() {
        let json_rank = r#"
        {
            "id": 2,
            "finisher_id": 2,
            "name": "Deer",
            "icon": "https://render.guildwars2.com/file/DECD0D647C9433CC2128BF2F6FE5A5185513EE59/347223.png",
            "min_rank": 10,
            "max_rank": 19,
            "levels": [
                {
                    "min_rank": 10,
                    "max_rank": 19,
                    "points": 4000
                }
            ]
        }"#;

        serde_json::from_str::<Rank>(json_rank).unwrap();
    }
}

