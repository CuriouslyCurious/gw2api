use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/items";

/// Contains ids of all player discovered items. Details about a particular item can be
/// obtained from the v1/item_details resource.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Items {
    /// List of ids of all discovered items.
    pub items: Vec<u32>,
}

impl Items {
    /// Retrieve all discovered items' ids.
    pub fn get_all(client: &Client) -> Result<Items, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::items::*;
    use crate::client::Client;

    const JSON_ITEMS: &str = r#"
    {
      "items": [
        12546,
        38875,
        26706
      ]
    }"#;

    #[test]
    fn create_items() {
        serde_json::from_str::<Items>(JSON_ITEMS).unwrap();
    }

    #[test]
    fn get_all_items() {
        let client = Client::new();
        assert!(Items::get_all(&client).unwrap().items.len() >= 2000)
    }
}
