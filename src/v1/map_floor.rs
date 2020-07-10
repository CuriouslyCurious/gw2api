use crate::client::Client;
use crate::error::ApiError;

use std::collections::BTreeMap;

const ENDPOINT_URL: &str = "/v1/map_floor";

/// Struct containing details about a specified map floor. All coordinates are map coordinates.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Floor {
    /// Tuple describing the dimensions of the texture for the map floor.
    texture_dims: Vec<f32>,
    /// If present, it represents a rectangle of downloadable textures. Every tile coordinate outside this rectangle is not available on the tile server.
    clamped_view: Vec<Vec<f32>>,
    /// A map of the regions of a map, where the region id is the key and the region object is the
    /// value.
    regions: BTreeMap<u32, Region>,
}

/// Struct containing information about a region of a map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Region {
    /// Region name.
    name: String,
    /// The coordinates of the region label.
    label_coord: Vec<f32>,
    /// Dimensions of the region, given as the coordinates of the lower-left (SW) and upper-right (NE)
    /// corners.
    continent_rect: Vec<Vec<i32>>,
    /// Mapping from the map id to an object.
    maps: BTreeMap<u32, Map>
}

/// Struct containing information about a maps in the game, including information about floor and
/// translation data on how to translate between world coordinates and map coordinates.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
    /// Name of the map.
    name: String,
    /// Minimum level (height) of the map.
    min_level: i32,
    /// Maximum level of the map.
    max_level: i32,
    /// Default floor for the map.
    default_floor: i32,
    /// List of available floors.
    #[serde(default)]
    floors: Vec<i32>,
    /// The coordinates of the map label.
    #[serde(default)]
    label_coord: Vec<f32>,
    /// Dimensions of the map, given as the coordinates of the lower-left (SW) and upper-right (NE)
    /// corners.
    #[serde(default)]
    map_rect: Vec<Vec<i32>>,
    /// Dimensions of the map within the continent coordinate system,
    /// given as the coordinates of the lower-left (SW) and upper-right (NE) corners.
    #[serde(default)]
    continent_rect: Vec<Vec<i32>>,
    /// List of points of interests in the map (landmarks, waypoints and vistas)
    #[serde(default)]
    points_of_interest: Vec<Poi>,
    /// List of god shrines (usually empty with the exception of Orr maps).
    #[serde(default)]
    god_shrines: Vec<GodShrine>,
    /// List of renown hearts.
    #[serde(default)]
    tasks: Vec<Task>,
    /// List of skill challenges.
    #[serde(default)]
    skill_challenges: Vec<SkillChallenge>,
    /// List of sectors/sub-regions in the map.
    #[serde(default)]
    sectors: Vec<Sector>,
    /// List of training points.
    #[serde(default)]
    training_points: Vec<TrainingPoint>,
    /// List of adventures.
    #[serde(default)]
    adventures: Vec<Adventure>,
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
    id: u32,
    /// Name of the point of interest.
    name: String,
    /// The kind of point of interest this particular one is.
    #[serde(rename = "type")]
    poi_type: PoiType,
    /// The floor number of this object.
    floor: i32,
    /// The coordinates of this object.
    coord: Vec<f32>,
}

/// Struct containing information about an icon (used for the shrines of the gods).
#[derive(Debug, Deserialize, PartialEq)]
pub struct Icon {
    /// File id of the icon.
    #[serde(rename = "file_id")]
    id: u32,
    /// File signature of the icon.
    signature: String,
}

/// Struct containing information about a god shrine (found in Straits of Devestation)
#[derive(Debug, Deserialize, PartialEq)]
pub struct GodShrine {
    /// id of the god shrine.
    id: u32,
    /// Uncontested name of the god shrine.
    name: String,
    /// Contested name of the god shrine.
    name_contested: String,
    /// Coordinates where the god shrine is.
    coord: Vec<f32>,
    /// id of the point of interest for the shrine.
    poi_id: u32,
    /// Icon for the uncontested shrine.
    icon: Icon,
    /// Icon for the contested version of the shrine.
    icon_contested: Icon,
}

/// Struct containing information about a task (renown heart).
#[derive(Debug, Deserialize, PartialEq)]
pub struct Task {
    /// id of the renown heart.
    #[serde(rename = "task_id")]
    id: u32,
    /// Objective or name of the heart.
    objective: String,
    /// The level of the the heart.
    level: u16,
    /// Coordinates where the task takes place.
    coord: Vec<f32>,
    /// Boundaries of the task.
    bounds: Vec<Vec<f32>>,
}

/// Struct containing information about a skill challenge.
#[derive(Debug, Deserialize, PartialEq)]
pub struct SkillChallenge {
    // TODO: Check if these fields are accurate
    /// The expansion required for the skill challenge.
    expac: u32,
    /// id of the skill challenge
    #[serde(rename = "idx")]
    id: u32,
    /// Coordinates where the skill challenge is.
    coord: Vec<f32>,
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
    id: u32,
    /// Name of the training point.
    name: String,
    /// Description of the training point.
    description: String,
    /// Coordinates where the training point is.
    coord: Vec<f32>,
    /// Type of training point (either Maguuma or Tyria)
    #[serde(rename = "type")]
    point_type: PointType,
}

/// Struct containing information about an adventure. Seemingly only used in a few maps in the
/// Heart of Maguuma.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Adventure {
    /// id of the adventure.
    #[serde(rename = "guid")]
    id: String,
    /// Coordinates where the adventure is.
    coord: Vec<f32>,
    /// Name of the adventure.
    name: String,
    /// Further information about the leaderboard and description of the adventure.
    leaderboard: Leaderboard,
}

/// Struct containing information about a leaderboard for an adventure.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Leaderboard {
    /// id of the leaderboard.
    #[serde(rename = "guid")]
    id: String,
    /// Name of the adventure the leaderboard belongs to.
    title: String,
    /// Description of the adventure.
    description: String,
}

/// Struct containing information about a sector (area) of a map.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Sector {
    /// id of the area.
    #[serde(rename = "sector_id")]
    id: u32,
    /// Name of the area.
    name: String,
    /// Level of the area.
    level: u32,
    /// Coordinates of this area.
    coord: Vec<f32>,
    /// Bounds of the area.
    bounds: Vec<Vec<f32>>,
}

impl Floor {
    /// Retrieve a map floor by its continent id and floor number.
    pub fn get_map_floor(client: &Client, continent_id: u32, floor: i32) -> Result<Floor, ApiError> {
        let url = format!("{}?continent_id={}&floor={}", ENDPOINT_URL, continent_id, floor);
        client.request(&url)
    }

    /// Returns a tuple describing the dimension of the texture for the map floor.
    pub fn texture_dims(&self) -> &Vec<f32> {
        &self.texture_dims
    }

    /// If present, it represents a rectangle of downloadable textures. Every tile coordinate outside this rectangle is not available on the tile server.
    /// Returns the name of the map.
    pub fn clamped_view(&self) -> &Vec<Vec<f32>> {
        &self.clamped_view
    }

    /// Returns a map of the regions of a map, where the region id is key and the region object is the
    /// value.
    pub fn regions(&self) -> &BTreeMap<u32, Region> {
        &self.regions
    }
}

impl Region {
    /// Returns the region name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the coordinates of the region label.
    pub fn label_coord(&self) -> &Vec<f32> {
        self.label_coord.as_ref()
    }

    /// Returns a reference to a Vec object that contains the dimensions of the region, using the
    /// continent coordinate system.
    pub fn continent_rect(&self) -> &Vec<Vec<i32>> {
        self.continent_rect.as_ref()
    }

    /// Returns a reference to a map of maps.
    pub fn maps(&self) -> &BTreeMap<u32, Map> {
        &self.maps
    }
}

impl Map {
    /// Returns the name of the map.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the minimum level (height) of the map.
    pub fn min_level(&self) -> i32 {
        self.min_level
    }

    /// Returns the maximum level (height) of the map.
    pub fn max_level(&self) -> i32 {
        self.max_level
    }

    /// Returns the default floor.
    pub fn default_floor(&self) -> i32 {
        self.default_floor
    }

    /// Returns a reference to a list of floors for the map.
    pub fn floors(&self) -> &Vec<i32> {
        self.floors.as_ref()
    }

    /// Returns a reference to a list of lists that contains the dimensions of the map, using the
    /// maps local coordinates.
    pub fn map_rect(&self) -> &Vec<Vec<i32>> {
        &self.map_rect
    }

    /// Returns a reference to a list of lists that contains the dimensions of the map, using the
    /// continent coordinate system.
    pub fn continent_rect(&self) -> &Vec<Vec<i32>> {
        self.continent_rect.as_ref()
    }

    /// Returns a reference to a list of points of interests in the map.
    pub fn points_of_interest(&self) -> &Vec<Poi> {
        &self.points_of_interest
    }

    /// Returns a reference to a list of god shrines in the map. Only relevant in Orr, should
    /// otherwise be empty.
    pub fn god_shrines(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Returns a reference to a list of tasks (renown hearts) in the map.
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Returns a reference to a list of skill challenges in the map.
    pub fn skill_challenges(&self) -> &Vec<SkillChallenge> {
        &self.skill_challenges
    }

    /// Returns a reference to a list of sectors/sub-regions in the map.
    pub fn sectors(&self) -> &Vec<Sector> {
        &self.sectors
    }

    // TODO: Check if this is accurate.
    /// Returns a reference to a list of training points (probably meaning channelable hero points) in the map.
    pub fn training_points(&self) -> &Vec<TrainingPoint> {
        &self.training_points
    }

    /// Returns a reference to a list of adventures in the map. Only used in Verdant Brink & Tangled Depths.
    pub fn adventures(&self) -> &Vec<Adventure> {
        &self.adventures
    }
}

impl Poi {
    /// Returns the id of the point of interest.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the name of the point of interest.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns what type of point of interest this is. Possible types are: Landmark, Waypoint,
    /// Vista and Unlock.
    pub fn poi_type(&self) -> &PoiType {
        &self.poi_type
    }

    /// Returns the floor number of the point of interest.
    pub fn floor(&self) -> i32 {
        self.floor
    }

    /// Returns the coordinates for the point of interest.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }
}

impl Icon {
    /// Returns the file id of the icon.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the file signature of the icon.
    pub fn signature(&self) -> &str {
        &self.signature
    }
}

impl GodShrine {
    /// Returns the id of the god shrine.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the regular name of the god shrine.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the contested name of the god shrine.
    pub fn name_contested(&self) -> &str {
        &self.name_contested
    }

    /// Returns the coordinates for the god shrine.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }

    /// Returns the id of point of interest of the shrine.
    pub fn poi_id(&self) -> u32 {
        self.poi_id
    }

    /// Returns the icon information for the regular shrine.
    pub fn icon(&self) -> &Icon {
        &self.icon
    }

    /// Returns the icon information for the contested shrine.
    pub fn icon_contested(&self) -> &Icon {
        &self.icon_contested
    }
}

impl Task {
    /// Returns the id of the renown heart.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the objective or the name of the heart.
    pub fn objective(&self) -> &str {
        &self.objective
    }

    /// Returns the recommended character level of the heart.
    pub fn level(&self) -> u16 {
        self.level
    }

    /// Returns the coordinates where the task takes place.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }

    /// Returns the boundaries of the task's area.
    pub fn bounds(&self) -> &Vec<Vec<f32>> {
        &self.bounds
    }
}

impl SkillChallenge {
    /// Returns the expansion required for the skill challenge.
    pub fn expac(&self) -> u32 {
        self.expac
    }

    /// Returns the id of the skill challenge.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the coordinates where the skill challenge is.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }
}

impl TrainingPoint {
    /// Returns the id of the training point.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the name of the training point.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the training point.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the coordinates of where the training point is.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }

    /// Returns the type of training point it is (either Tyria, Maguuma, Desert or Unknown).
    pub fn point_type(&self) -> &PointType {
        &self.point_type
    }
}

impl Adventure {
    /// Returns the id of the adventure.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the coordinates where the adventure is.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }

    /// Returns the name of the adventure.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a Leaderboard object containing further information and description about an
    /// adventure.
    pub fn leaderboard(&self) -> &Leaderboard {
        &self.leaderboard
    }
}

impl Leaderboard {
    /// Returns the id of the leaderboard.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the title of the adventure the leaderboard belongs to.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the description of the adventure the leaderboard belongs to.
    pub fn description(&self) -> &str {
        &self.description
    }

}

impl Sector {
    /// Returns the id of the area.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the name of the area.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the level of the area.
    pub fn level(&self) -> u32 {
        self.level
    }

    /// Returns the coordinates of the area.
    pub fn coord(&self) -> &Vec<f32> {
        &self.coord
    }

    /// Returns the boundaries of the task's area.
    pub fn bounds(&self) -> &Vec<Vec<f32>> {
        &self.bounds
    }
}

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
        match serde_json::from_str::<Floor>(JSON_FLOOR) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_map_floor() {
        let client = Client::new();
        assert!(Floor::get_map_floor(&client, 1, 1).unwrap().regions().len() > 0)
    }
}
