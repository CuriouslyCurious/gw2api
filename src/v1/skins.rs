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

#[cfg(test)]
mod tests {
    use crate::v1::skins::*;
    use crate::client::Client;

    const JSON_SKINS: &str = r#"
    {
      "skins": [
        1343,
        1344,
        1345,
        1346
      ]
    }"#;

    #[test]
    fn create_skins() {
        serde_json::from_str::<Skins>(JSON_SKINS).unwrap();
    }

    #[test]
    fn get_all_skins() {
        let client = Client::new();
        assert!(Skins::get_all(&client).unwrap().skins.len() >= 2000)
    }
}
