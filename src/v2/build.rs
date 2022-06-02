use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v2/build";

/// Contains a Guild Wars 2 build version id
#[derive(Debug, Deserialize, PartialEq)]
pub struct Build {
    /// The current build version id
    pub id: u32,
}

impl Build {
    /// Returns a Build struct containing the current build version within the `id` field.
    pub fn get_build(client: &Client) -> Result<Build, ApiError> {
        client.request(ENDPOINT_URL)
    }
}
