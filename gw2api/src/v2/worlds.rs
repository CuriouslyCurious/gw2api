use serde::Deserialize;
use gw2api_derive::ParamEndpoint;

/// Information about a world, a.k.a server, mostly only relevant for World vs. World.
#[derive(ParamEndpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v2/worlds?id={}"]
pub struct World {
    /// World ID.
    ///
    /// First digit indicates the region, 1 being North America, 2 is Europe.
    ///
    /// Second digit indicates the world's assigned language: 0 = English, 1 = French, 2 = German,
    /// 3 = Spanish.
    ///
    /// Third and forth digit are numbers to make the world unique.
    pub id: u32,
    /// Localized name of the world.
    pub name: String,
    /// World's population level.
    pub population: Population,
}

/// Different values of population a world can have.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Population {
    Low,
    Medium,
    High,
    VeryHigh,
    Full,
}
