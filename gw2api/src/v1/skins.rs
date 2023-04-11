use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Contains ids of all skins. Details about a particular skin can be obtained from the
/// v1/skins_details resource.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/skins"]
pub struct Skins {
    /// List of ids of all skins.
    pub skins: Vec<u32>,
}
