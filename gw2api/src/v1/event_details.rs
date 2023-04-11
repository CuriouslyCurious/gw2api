use serde::Deserialize;

use gw2api_derive::Endpoint;

use std::collections::HashMap;

/// Struct containing a hashmap of all available events in the game.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/event_details"]
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

impl Event {}

impl Location {}

#[cfg(test)]
mod tests {
    use crate::v1::event_details::*;

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
}
