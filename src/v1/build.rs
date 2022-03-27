use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/build";

/// Contains a Guild Wars 2 build version id
#[derive(Debug, Deserialize)]
pub struct Build {
    /// The current build version id
    #[serde(rename = "build_id")]
    pub id: u32,
}

impl Build {
    /// Returns a Build struct containing the current build version within the `id` field.
    pub fn get_build(client: &Client) -> Result<Build, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::v1::build::Build;
    #[test]
    fn get_build() {
        let client = Client::new();
        assert_ne!(Build::get_build(&client).unwrap().id, 0)
    }
}
