use serde::Deserialize;

use std::collections::HashMap;

use crate::client::Client;
use crate::error::ApiError;

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
    pub name: String,
    /// Level of the event.
    pub level: u32,
    /// The map id of the map where the event takes place.
    pub map_id: u32,
    /// A list of additional flags.
    pub flags: Vec<Flag>,
    /// The location of the event.
    pub location: Location,
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
    pub shape: Shape,
    /// Coordinates for the center of the event.
    pub center: Vec<f32>,
    /// Height of a non-sphere.
    #[serde(default)]
    pub height: f32,
    /// Radius of a sphere/cylinder.
    #[serde(default)]
    pub radius: f32,
    /// Rotation of a sphere/cylinder.
    #[serde(default)]
    pub rotation: f32,
    /// Range of the polygon.
    #[serde(default)]
    pub z_range: Vec<f32>,
    /// Points of the polygon.
    #[serde(default)]
    pub points: Vec<(f32, f32)>,
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
}

impl Event {}

impl Location  {}

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
        serde_json::from_str::<Event>(JSON_EVENT).unwrap();
    }

    #[test]
    fn get_all_events() {
        let client = Client::new();
        assert!(Events::get_all_events(&client).unwrap().events.len() >= 1000)
    }
}
