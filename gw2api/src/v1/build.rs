//! Get current Guild Wars 2 build version.

use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Contains a Guild Wars 2 build version id.
///
/// //```
/// //use gw2api::client::Client;
/// //use gw2api::v1::Build;
///
/// //let client = Client::new();
/// //let current_build = client.get::<Build>().unwrap();
/// //println!("{}", current_build.id);
/// //```
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/build"]
pub struct Build {
    /// The current build version id
    #[serde(rename = "build_id")]
    pub id: u32,
}

#[cfg(test)]
mod tests {
    use crate::v1::build::Build;
    use similar_asserts::assert_eq;

    #[test]
    fn create_build() {
        let build = Build { id: 115267 };
        assert_eq!(build.id, 115267);
    }
}
