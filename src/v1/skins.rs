use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/skins";

/// Contains ids of all skins. Details about a particular skin can be obtained from the
/// v1/skins_details resource.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Skins {
    /// List of ids of all skins.
    pub skins: Vec<u32>,
}

impl Skins {
    /// Retrieve all skins' ids.
    pub fn get_all(client: &Client) -> Result<Skins, ApiError> {
        client.request(ENDPOINT_URL)
    }
}
