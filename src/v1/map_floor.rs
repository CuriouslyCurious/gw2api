use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

use std::collections::BTreeMap;

const ENDPOINT_URL: &str = "/v1/map_floor";

/// Struct containing details about a specified map floor. All coordinates are map coordinates.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Floor {
    /// Tuple describing the dimensions of the texture for the map floor.
    pub texture_dims: Vec<f32>,
    /// If present, it represents a rectangle of downloadable textures. Every tile coordinate outside this rectangle is not available on the tile server.
    pub clamped_view: Vec<Vec<f32>>,
    /// A map of the regions of a map, where the region id is the key and the region object is the
    /// value.
    pub regions: BTreeMap<u32, Region>,
}

/// Struct containing information about a region of a map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Region {
    /// Region name.
    pub name: String,
    /// The coordinates of the region label.
    pub label_coord: Vec<f32>,
    /// Dimensions of the region, given as the coordinates of the lower-left (SW) and upper-right (NE)
    /// corners.
    pub continent_rect: Vec<Vec<i32>>,
    /// Mapping from the map id to an object.
    pub maps: BTreeMap<u32, Map>
}

/// Struct containing information about a maps in the game, including information about floor and
/// translation data on how to translate between world coordinates and map coordinates.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
    /// Name of the map.
    pub name: String,
    /// Minimum level (height) of the map.
    pub min_level: i32,
    /// Maximum level of the map.
    pub max_level: i32,
    /// Default floor for the map.
    pub default_floor: i32,
    /// List of available floors.
    #[serde(default)]
    pub floors: Vec<i32>,
    /// The coordinates of the map label.
    #[serde(default)]
    pub label_coord: Vec<f32>,
    /// Dimensions of the map, given as the coordinates of the lower-left (SW) and upper-right (NE)
    /// corners.
    #[serde(default)]
    pub map_rect: Vec<Vec<i32>>,
    /// Dimensions of the map within the continent coordinate system,
    /// given as the coordinates of the lower-left (SW) and upper-right (NE) corners.
    #[serde(default)]
    pub continent_rect: Vec<Vec<i32>>,
    /// List of points of interests in the map (landmarks, waypoints and vistas)
    #[serde(default)]
    pub points_of_interest: Vec<Poi>,
    /// List of god shrines (usually empty with the exception of Orr maps).
    #[serde(default)]
    pub god_shrines: Vec<GodShrine>,
    /// List of renown hearts.
    #[serde(default)]
    pub tasks: Vec<Task>,
    /// List of skill challenges.
    #[serde(default)]
    pub skill_challenges: Vec<SkillChallenge>,
    /// List of sectors/sub-regions in the map.
    #[serde(default)]
    pub sectors: Vec<Sector>,
    /// List of training points.
    #[serde(default)]
    pub training_points: Vec<TrainingPoint>,
    /// List of adventures.
    #[serde(default)]
    pub adventures: Vec<Adventure>,
}

/// Possible types a point of interest can be.
#[derive(Debug, Deserialize, PartialEq, Hash)]
pub enum PoiType {
    /// Actual points of interest (PoI).
    #[serde(rename = "landmark")]
    Landmark,
    /// A waypoint.
    #[serde(rename = "waypoint")]
    Waypoint,
    /// A vista.
    #[serde(rename = "vista")]
    Vista,
    /// An unlock.
    #[serde(rename = "unlock")]
    Unlock,
}

/// Object with information about a particular point of interest.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Poi {
    /// id of the point of interest.
    #[serde(rename = "poi_id")]
    pub id: u32,
    /// Name of the point of interest.
    pub name: String,
    /// The kind of point of interest this particular one is.
    #[serde(rename = "type")]
    pub poi_type: PoiType,
    /// The floor number of this object.
    pub floor: i32,
    /// The coordinates of this object.
    pub coord: Vec<f32>,
}

/// Struct containing information about an icon (used for the shrines of the gods).
#[derive(Debug, Deserialize, PartialEq)]
pub struct Icon {
    /// File id of the icon.
    #[serde(rename = "file_id")]
    pub id: u32,
    /// File signature of the icon.
    pub signature: String,
}

/// Struct containing information about a god shrine (found in Straits of Devestation)
#[derive(Debug, Deserialize, PartialEq)]
pub struct GodShrine {
    /// id of the god shrine.
    pub id: u32,
    /// Uncontested name of the god shrine.
    pub name: String,
    /// Contested name of the god shrine.
    pub name_contested: String,
    /// Coordinates where the god shrine is.
    pub coord: Vec<f32>,
    /// id of the point of interest for the shrine.
    pub poi_id: u32,
    /// Icon for the uncontested shrine.
    pub icon: Icon,
    /// Icon for the contested version of the shrine.
    pub icon_contested: Icon,
}

/// Struct containing information about a task (renown heart).
#[derive(Debug, Deserialize, PartialEq)]
pub struct Task {
    /// id of the renown heart.
    #[serde(rename = "task_id")]
    pub id: u32,
    /// Objective or name of the heart.
    pub objective: String,
    /// The level of the the heart.
    pub level: u16,
    /// Coordinates where the task takes place.
    pub coord: Vec<f32>,
    /// Boundaries of the task.
    pub bounds: Vec<Vec<f32>>,
}

/// Struct containing information about a skill challenge.
#[derive(Debug, Deserialize, PartialEq)]
pub struct SkillChallenge {
    // TODO: Check if these fields are accurate
    /// The expansion required for the skill challenge.
    pub expac: u32,
    /// id of the skill challenge
    #[serde(rename = "idx")]
    pub id: u32,
    /// Coordinates where the skill challenge is.
    pub coord: Vec<f32>,
}

/// Possible types of a training point, Tyrian ones give less hero points than Maguuman or ones
/// found in the Crystal Desert.
#[derive(Debug, Deserialize, PartialEq)]
pub enum PointType {
    Tyria,
    Maguuma,
    Desert,
    Tundra,
    Unknown,
}

/// Struct containing information about a training point.
#[derive(Debug, Deserialize, PartialEq)]
pub struct TrainingPoint {
    /// id of the training point.
    pub id: u32,
    /// Name of the training point.
    pub name: String,
    /// Description of the training point.
    pub description: String,
    /// Coordinates where the training point is.
    pub coord: Vec<f32>,
    /// Type of training point (either Maguuma or Tyria)
    #[serde(rename = "type")]
    pub point_type: PointType,
}

/// Struct containing information about an adventure. Seemingly only used in a few maps in the
/// Heart of Maguuma.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Adventure {
    /// id of the adventure.
    #[serde(rename = "guid")]
    pub id: String,
    /// Coordinates where the adventure is.
    pub coord: Vec<f32>,
    /// Name of the adventure.
    pub name: String,
    /// Further information about the leaderboard and description of the adventure.
    pub leaderboard: Leaderboard,
}

/// Struct containing information about a leaderboard for an adventure.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Leaderboard {
    /// id of the leaderboard.
    #[serde(rename = "guid")]
    pub id: String,
    /// Name of the adventure the leaderboard belongs to.
    pub title: String,
    /// Description of the adventure.
    pub description: String,
}

/// Struct containing information about a sector (area) of a map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Sector {
    /// id of the area.
    #[serde(rename = "sector_id")]
    pub id: u32,
    /// Name of the area.
    pub name: String,
    /// Level of the area.
    pub level: u32,
    /// Coordinates of this area.
    pub coord: Vec<f32>,
    /// Bounds of the area.
    pub bounds: Vec<Vec<f32>>,
}

impl Floor {
    /// Retrieve a map floor by its continent id and floor number.
    pub fn get_map_floor(client: &Client, continent_id: u32, floor: i32) -> Result<Floor, ApiError> {
        let url = format!("{}?continent_id={}&floor={}", ENDPOINT_URL, continent_id, floor);
        client.request(&url)
    }
}

impl Region {}

impl Map {}

impl Poi {}

impl Icon {}

impl GodShrine {}

impl Task {}

impl SkillChallenge {}

impl TrainingPoint {}

impl Adventure {}

impl Leaderboard {}

impl Sector {}

#[cfg(test)]
mod tests {
    use crate::v1::map_floor::*;
    use crate::client::Client;

    // A mix of Ruins of Orr for god shrines and Tangled Depths for adventures. This endpoint is
    // such a mess.
    const JSON_FLOOR: &str = r#"
    {
      "texture_dims": [
          49152,
          49152
      ],
      "clamped_view": [
        [
          0,
          0
        ],
        [
          38000,
          48000
        ]
      ],
      "regions": {
        "3": {
          "name": "Ruins of Orr",
          "label_coord": [
            13600,
            25728
          ],
          "continent_rect": [
            [
              10112,
              22400
            ],
            [
              17792,
              29312
            ]
          ],
          "maps": {
            "51": {
              "name": "Straits of Devastation",
              "min_level": 70,
              "max_level": 75,
              "default_floor": 1,
              "label_coord": [
                15920,
                23808
              ],
              "map_rect": [
                [
                  -39936,
                  -33792
                ],
                [
                  39936,
                  33792
                ]
              ],
              "continent_rect": [
                [
                  14464,
                  22400
                ],
                [
                  17792,
                  25216
                ]
              ],
              "points_of_interest": [
                {
                  "poi_id": 736,
                  "name": "Plaza of Lost Wisdom",
                  "type": "landmark",
                  "floor": 1,
                  "coord": [
                    15529.3,
                    22720.4
                  ]
                },
                {
                  "poi_id": 760,
                  "name": "Waywarde Waypoint",
                  "type": "waypoint",
                  "floor": 1,
                  "coord": [
                    15764.5,
                    24931.8
                  ]
                },
                {
                  "poi_id": 1723,
                  "name": "",
                  "type": "vista",
                  "floor": 1,
                  "coord": [
                    16934.3,
                    23610.7
                  ]
                }
              ],
              "god_shrines": [
                {
                  "id": 5,
                  "name": "Temple of Balthazar <c=#a9a9a9>[Uncontested]</c><br>• All Balthazar statues in Orr are disabled.</c>",
                  "name_contested": "Temple of Balthazar <c=#ff8c00>[Contested]</c><br>• All Balthazar statues in Orr are active.",
                  "coord": [
                    15071.2,
                    24623.8
                  ],
                  "poi_id": 762,
                  "icon": {
                    "file_id": 347219,
                    "signature": "61EB4505770BBC22013E4EBD60D2B878456C9712"
                  },
                  "icon_contested": {
                    "file_id": 347220,
                    "signature": "4B937E3ECA980ACD29655EA1D767DD5E0E23462F"
                  }
                }
              ],
              "tasks": [],
              "skill_challenges": [
                {
                  "expac": 0,
                  "idx": 157,
                  "coord": [
                    14611.5,
                    23328.9
                  ]
                }
              ],
              "sectors": [
                {
                  "sector_id": 692,
                  "name": "Shark's Teeth Archipelago",
                  "level": 73,
                  "coord": [
                    16620.6,
                    24486.7
                  ],
                  "bounds": [
                    [
                      16929.7,
                      24499.7
                    ],
                    [
                      16684.7,
                      24778.7
                    ],
                    [
                      16476.7,
                      24941.7
                    ],
                    [
                      16338.4,
                      24892.1
                    ],
                    [
                      16294.6,
                      24586.3
                    ],
                    [
                      16329.3,
                      24470.4
                    ],
                    [
                      16166,
                      24305.3
                    ],
                    [
                      16664.4,
                      23936.9
                    ],
                    [
                      17100.9,
                      24094.1
                    ],
                    [
                      17221.3,
                      24362.1
                    ],
                    [
                      16929.7,
                      24499.7
                    ]
                  ]
                }
              ],
              "training_points": [
                {
                  "id": 112,
                  "name": "",
                  "description": "",
                  "coord": [
                      3701.83,
                      18299.5
                  ],
                  "type": "Maguuma"
                }
              ],
              "adventures": [
                {
                  "guid": "8D00FA87-28CD-4402-AAEF-501A610E0447",
                  "coord": [
                      4548.5,
                      17730.2
                  ],
                  "name": "Beetle Feast",
                  "leaderboard": {
                      "guid": "31AC0BFC-C14E-4708-91C5-EAC226D8DE7C",
                      "title": "Beetle Feast",
                      "description": "Eat yellow mushrooms to score points and blue mushrooms to gain abilities that help you in your hunt. Beware of the poisonous mushrooms with green spores, and use your burrow ability to bypass closed gates!"
                  }
                }
              ]
            }
          }
        }
      }
    }"#;

    #[test]
    fn create_floor() {
        serde_json::from_str::<Floor>(JSON_FLOOR).unwrap();
    }

    #[test]
    fn get_map_floor() {
        let client = Client::new();
        assert!(Floor::get_map_floor(&client, 1, 1).unwrap().regions.len() > 0)
    }
}
