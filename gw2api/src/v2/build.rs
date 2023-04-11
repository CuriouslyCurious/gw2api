use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Contains a Guild Wars 2 build version id
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v2/build"]
pub struct Build {
    /// The current build version id
    pub id: u32,
}
