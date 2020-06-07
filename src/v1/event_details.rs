use crate::client::Client;
use crate::error::ApiError;

use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v1/event_details";

/// Struct containing a hashmap of all available events in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Events {
    events: HashMap<String, Event>,
}

/// Flags representing the type of the event.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Flag {
    #[serde(rename = "group_event")]
    GroupEvent,
    #[serde(rename = "map_wide")]
    MapWide,
    #[serde(rename = "meta_event")]
    MetaEvent,
    #[serde(rename = "dungeon_event")]
    DungeonEvent,
}

/// Contains information about an event.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    /// Name of the event.
    name: String,
    /// Level of the event.
    level: u32,
    /// The map id of the map where the event takes place.
    map_id: u32,
    /// A list of additional flags.
    flags: Vec<Flag>,
    /// The location of the event.
    location: Location,
}

/// Possible shapes of the event area.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Shape {
    #[serde(rename = "sphere")]
    Sphere,
    #[serde(rename = "cylinder")]
    Cylinder,
    #[serde(rename = "poly")]
    Poly,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Location {
    /// Shape of the event area (sphere, cylinder, poly).
    #[serde(rename = "type")]
    shape: Shape,
    /// Coordinates for the center of the event.
    center: Vec<f32>,
    /// Height of a non-sphere.
    #[serde(default)]
    height: f32,
    /// Radius of a sphere/cylinder.
    #[serde(default)]
    radius: f32,
    /// Rotation of a sphere/cylinder.
    #[serde(default)]
    rotation: f32,
    /// Range of the polygon.
    #[serde(default)]
    z_range: Vec<f32>,
    /// Points of the polygon.
    #[serde(default)]
    points: Vec<(f32, f32)>,
}

impl Events {
    /// Retrieve an event by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Events, ApiError> {
        let url = format!("{}?event_id={}", ENDPOINT_URL, id);
        client.request(&url)
    }

    /// Retrieve all continents that are in the game.
    pub fn get_all_events(client: &Client) -> Result<Events, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Returns the hashmap containing all the events.
    pub fn events(&self) -> &HashMap<String, Event> {
        &self.events
    }
}

impl Event {
    /// Returns the name of the continent.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the level of the continent.
    pub fn level(&self) -> u32 {
        self.level
    }

    /// Returns the map id of the map where the event takes place.
    pub fn map_id(&self) -> u32 {
        self.map_id
    }

    /// Returns the flags describing the type of event it is (can be empty).
    pub fn flags(&self) -> &Vec<Flag> {
        &self.flags
    }

    /// Returns location & shape of the event.
    pub fn location(&self) -> &Location {
        &self.location
    }
}

impl Location {
    /// Returns the shape of the event.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Returns the coordinates for the center of the event.
    pub fn center(&self) -> &Vec<f32> {
        &self.center
    }

    /// Returns the height of a non-sphere event zone.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Returns the radius of a sphere/cylinder event zone.
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Returns the rotation of a sphere/cylinder event zone.
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// Returns the range of the polygon.
    pub fn z_range(&self) -> &Vec<f32> {
        &self.z_range
    }

    /// Returns the points of a polygon.
    pub fn points(&self) -> &Vec<(f32, f32)> {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::event_details::*;
    use crate::client::Client;

    const JSON_EVENT: &str = r#"
    {
      "name": "Defeat the renegade charr.",
      "level": 42,
      "map_id": 20,
      "flags": [],
      "location": {
        "type": "sphere",
        "center": [ -9463.6, -40310.2, -785.799 ],
        "radius": 2500,
        "rotation": 0
      }
    }"#;

    #[test]
    fn create_event() {
        match serde_json::from_str::<Event>(JSON_EVENT) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_events() {
        let client = Client::new();
        assert!(Events::get_all_events(&client).unwrap().events().len() >= 1000)
    }
}
