use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Information about a structured PvP rank.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v2/pvp/ranks"]
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
