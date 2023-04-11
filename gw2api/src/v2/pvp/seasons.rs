//use crate::client::Client;
//use crate::error::ApiError;
//use crate::utils::ids_to_string;
//use std::collections::HashMap;
//
//const ENDPOINT_URL: &str = "/v2/pvp/seasons";
//
///// Information about a structured PvP season.
//#[derive(Debug, Deserialize, PartialEq)]
//pub struct Season {
//    /// UUID of the season.
//    id: String,
//    /// Given name of the PvP season.
//    name: String,
//    /// Timestamp of when the season started.
//    #[serde(rename = "start")]
//    start_time: String,
//    /// Timestamp of when the season ends.
//    #[serde(rename = "end")]
//    end_time: String,
//    /// Whether or not the season is currently active.
//    active: bool,
//    /// A `Vec` of the divisions in the season.
//    divisions: Vec<Division>,
//    /// A `Vec` containing the leaderboard(s) of the season.
//    leaderboards: HashMap<LeaderboardType, Leaderboard>,
//}
//
///// A division in a structured PvP season.
//#[derive(Debug, Deserialize, PartialEq)]
//pub struct Division {
//    /// Name of the division.
//    name: String,
//    /// Flags applying to the division.
//    #[serde(default)]
//    flags: Vec<Flag>,
//    /// Potential url to a large version of the division's icon.
//    #[serde(rename = "large_icon")]
//    large_icon_url: Option<String>,
//    /// Potential url to a small version of the division's icon.
//    #[serde(rename = "small_icon")]
//    small_icon_url: Option<String>,
//    /// Url to the pip icon used for the division.
//    #[serde(rename = "pip_icon")]
//    pip_icon_url: String,
//    /// A `Vec` of tiers in the division, containing how many pips are required for each tier.
//    /// Number of tiers in the `Vec` indicates how many tiers there are.
//    tiers: Vec<Tier>,
//}
//
///// Flags that describe what can happen to a division.
//#[derive(Debug, Deserialize, PartialEq)]
//pub enum Flag {
//    CanLosePoints,
//    CanLoseTiers,
//    Repeatable,
//}
//
//#[derive(Debug, Deserialize, PartialEq)]
//pub struct Tier {
//    #[serde(alias = "rating")]
//    points: u32,
//}
//
///// The different leaderboard types that can be returned from the API.
///// Legendary and Guild were used during season 1-4, then replaced by Ladder.
//#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
//pub enum LeaderboardType {
//    #[serde(rename = "legendary")]
//    Legendary(Leaderboard),
//    #[serde(rename = "guild")]
//    Guild(Leaderboard),
//    #[serde(rename = "ladder")]
//    Ladder(Leaderboard),
//}
//
///// Contains details on an SPvP leaderboard.
//#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
//pub struct Leaderboard {
//    settings: Settings,
//    scorings: Vec<Scoring>,
//}
//
//#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
//pub struct Settings {
//    /// Name of something, that is supposedly always empty.
//    #[serde(default)]
//    name: String,
//    /// Duration of something, that seems to always be null.
//    #[serde(default)]
//    duration: Option<u32>,
//    /// A UUID that indicates the primary scoring component? XXX: Needs verification.
//    scoring: String,
//    /// Ranges of leaderboard ranks which gives out certain rewards, like titles.
//    tiers: Vec<Tiers>,
//}
//
///// Reference to select player/guild scoring methods.
//#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
//pub struct Scoring {
//    /// UUID for the scoring method.
//    id: String,
//    /// Type which the content is saved as.
//    #[serde(rename = "type")]
//    content_type: String,
//    /// Description of the scoring method.
//    description: String,
//    /// Name of the scoring method, e.g "wins", "losses", or "skill_rating".
//    #[serde(default)]
//    name: String,
//    /// How the scoring is ordered.
//    ordering: String,
//}
//
///// Contains ranges of leaderboard ranks, where the first value is the lowest rank and the second
///// is the highest rank of the range.
//#[derive(Debug, Deserialize, PartialEq)]
//pub struct Tiers {
//    range: Vec<f32>,
//}
////
//impl Season {
//    /// Retrieves all available season ids.
//    pub fn get_all_ids(client: &Client) -> Result<u32, ApiError> {
//        client.request(ENDPOINT_URL)
//    }
//
//    /// Retrieves a certain season's information by its id.
//    pub fn get_id(client: &Client, id: u32) -> Result<Season, ApiError> {
//        let url = format!("{}?ids={}", ENDPOINT_URL, id);
//        client.request(&url)
//    }
//
//    /// Retrives seasons' information by their ids.
//    pub fn get_seasons_by_ids(client: &Client, ids: Vec<String>) -> Result<Vec<Season>, ApiError> {
//        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
//        client.request(&url)
//    }
//
//    /// Retrives all seasons.
//    pub fn get_all_seasons(client: &Client) -> Result<Vec<Season>, ApiError> {
//        let url = format!("{}?ids=all", ENDPOINT_URL);
//        client.request(&url)
//    }
//
//    /// Returns the requested id of the hero.
//    pub fn id(&self) -> &str {
//        &self.id
//    }
//
//    /// Returns the name of the season.
//    pub fn name(&self) -> &str {
//        &self.name
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use crate::v2::pvp::seasons::*;
//    use crate::client::Client;
//
//    const JSON_SEASON: &str = r#"
//    {
//      "id": "44B85826-B5ED-4890-8C77-82DDF9F2CF2B",
//      "name": "PvP League Season One",
//      "start": "2015-12-01T20:00:00.000Z",
//      "end": "2016-01-28T01:00:00.000Z",
//      "active": false,
//      "divisions": [
//        {
//          "name": "Division 5: Diamond",
//          "flags": [
//            "CanLosePoints",
//            "CanLoseTiers"
//          ],
//          "large_icon": "https://render.guildwars2.com/file/B59BEBA950BA90083D409DE42BA8789F300F305D/1313338.png",
//          "small_icon": "https://render.guildwars2.com/file/F40A6880FB80C53F39D32B021737256371BE26B6/1313344.png",
//          "pip_icon": "https://render.guildwars2.com/file/F8E6757D23AA4495CDC9AE28E1B1FEB844BD2B1A/1313350.png",
//          "tiers": [
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            }
//          ]
//        },
//        {
//          "name": "Division 6: Legendary",
//          "flags": [
//            "CanLosePoints",
//            "CanLoseTiers",
//            "Repeatable"
//          ],
//          "large_icon": "https://render.guildwars2.com/file/97E44C1BB3B7434639D470E9F25DD9C601ACEDD9/1313339.png",
//          "small_icon": "https://render.guildwars2.com/file/540530F225A8B39990DBB165227A0624F10DFF9A/1313345.png",
//          "pip_icon": "https://render.guildwars2.com/file/7BD522452275401F6C9EE2C29F08DAEC0C52F144/1313351.png",
//          "tiers": [
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            },
//            {
//              "points": 5
//            }
//          ]
//        }
//      ],
//      "leaderboards": {
//        "guild": {
//          "settings": {
//            "name": "",
//            "duration": null,
//            "scoring": "16F74226-5DDF-4FE7-ADC5-72A49DA30572",
//            "tiers": [
//              {
//                "color": "\#E5E4E2",
//                "type": "Rank",
//                "name": "Platinum",
//                "range": [
//                  1,
//                  1
//                ]
//              },
//              {
//                "color": "\#EAC117",
//                "type": "Rank",
//                "name": "Gold",
//                "range": [
//                  25,
//                  2
//                ]
//              },
//              {
//                "color": "\#C0C0C0",
//                "type": "Rank",
//                "name": "Silver",
//                "range": [
//                  100,
//                  26
//                ]
//              },
//              {
//                "color": "\#D2691E",
//                "type": "Rank",
//                "name": "Bronze",
//                "range": [
//                  250,
//                  101
//                ]
//              },
//              {
//                "color": "\#C87533",
//                "type": "Rank",
//                "name": "Copper",
//                "range": [
//                  1000,
//                  251
//                ]
//              }
//            ]
//          },
//          "scorings": [
//            {
//              "id": "16F74226-5DDF-4FE7-ADC5-72A49DA30572",
//              "type": "Integer",
//              "description": "Team rating represents your team's skill level.",
//              "name": "Skill Rating",
//              "ordering": "MoreIsBetter"
//            },
//            {
//              "id": "9A9CB2FD-7D73-4DFE-8FDD-A97A9C7C0B0C",
//              "type": "Integer",
//              "description": "",
//              "name": "Wins",
//              "ordering": "MoreIsBetter"
//            },
//            {
//              "id": "9064DD25-2C75-48D6-88C9-7FAD18DC784D",
//              "type": "Integer",
//              "description": "",
//              "name": "Losses",
//              "ordering": "LessIsBetter"
//            }
//          ]
//        },
//        "legendary": {
//          "settings": {
//            "name": "",
//            "duration": null,
//            "scoring": "E6487336-4B5B-4BFA-9CFA-9FF232CAEF85",
//            "tiers": [
//              {
//                "range": [
//                  1,
//                  0.95
//                ]
//              },
//              {
//                "range": [
//                  0.95,
//                  0.9
//                ]
//              },
//              {
//                "range": [
//                  0.9,
//                  0.75
//                ]
//              }
//            ]
//          },
//          "scorings": [
//            {
//              "id": "E6487336-4B5B-4BFA-9CFA-9FF232CAEF85",
//              "type": "Integer",
//              "description": "Current prestige rank. Prestige rank can be gained or lost by winning or losing ranked matches in the legendary division.",
//              "name": "Prestige",
//              "ordering": "MoreIsBetter"
//            }
//          ]
//        }
//      }
//    }"#;
//
//    const JSON_DIVISION: &str = r#"
//    {
//      "name": "Division 5: Diamond",
//      "flags": [
//        "CanLosePoints",
//        "CanLoseTiers"
//      ],
//      "large_icon": "https://render.guildwars2.com/file/B59BEBA950BA90083D409DE42BA8789F300F305D/1313338.png",
//      "small_icon": "https://render.guildwars2.com/file/F40A6880FB80C53F39D32B021737256371BE26B6/1313344.png",
//      "pip_icon": "https://render.guildwars2.com/file/F8E6757D23AA4495CDC9AE28E1B1FEB844BD2B1A/1313350.png",
//      "tiers": [
//        {
//          "points": 5
//        },
//        {
//          "points": 5
//        },
//        {
//          "points": 5
//        },
//        {
//          "points": 5
//        },
//        {
//          "points": 5
//        },
//        {
//          "points": 5
//        },
//        {
//          "points": 5
//        }
//      ]
//    }"#;
//
//    const JSON_SCORING: &str = r#"
//    {
//      "id": "E6487336-4B5B-4BFA-9CFA-9FF232CAEF85",
//      "type": "Integer",
//      "description": "Current prestige rank. Prestige rank can be gained or lost by winning or losing ranked matches in the legendary division.",
//      "name": "Prestige",
//      "ordering": "MoreIsBetter"
//    }"#;
//
//    const JSON_SETTING: &str = r#"
//    {
//      "name": "",
//      "duration": null,
//      "scoring": "E6487336-4B5B-4BFA-9CFA-9FF232CAEF85",
//      "tiers": [
//        {
//          "range": [
//            1,
//            0.95
//          ]
//        },
//        {
//          "range": [
//            0.95,
//            0.9
//          ]
//        },
//        {
//          "range": [
//            0.9,
//            0.75
//          ]
//        }
//      ]
//    }
//    "#;
//
//    #[test]
//    fn create_division() {
//        match serde_json::from_str::<Division>(JSON_DIVISION) {
//            Ok(_) => assert!(true),
//            Err(e) => panic!(e.to_string()),
//        }
//    }
//
//    #[test]
//    fn create_scoring() {
//        match serde_json::from_str::<Scoring>(JSON_SCORING) {
//            Ok(_) => assert!(true),
//            Err(e) => panic!(e.to_string()),
//        }
//    }
//
//    #[test]
//    fn create_setting() {
//        match serde_json::from_str::<Settings>(JSON_SETTING) {
//            Ok(_) => assert!(true),
//            Err(e) => panic!(e.to_string()),
//        }
//    }
//
//    #[test]
//    fn create_season() {
//        match serde_json::from_str::<Season>(JSON_SEASON) {
//            Ok(_) => assert!(true),
//            Err(e) => panic!(e.to_string()),
//        }
//    }
//}
//
