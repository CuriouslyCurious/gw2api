use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/build";

/// Contains a Guild Wars 2 build version id
#[derive(Debug, Deserialize, PartialEq)]
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
    use crate::v1::build::Build;
    use similar_asserts::assert_eq;

    #[test]
    fn create_builds() {
        let build = Build { id: 115267 };
        assert_eq!(build.id, 115267);
    }
}
