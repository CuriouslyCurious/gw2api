use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;

const ENDPOINT_URL: &str = "/v1/items";

/// Contains ids of all player discovered items.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Items {
    /// List of ids of all discovered items.
    items: Vec<u32>,
}

impl Items {
    /// Retrieve all discovered items' ids.
    pub fn get_all(client: &Client) -> Result<Items, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
    }

    /// Returns the list of all player discovered item ids.
    pub fn items(&self) -> &Vec<u32> {
        &self.items
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
        match serde_json::from_str::<Items>(JSON_ITEMS) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_items() {
        let client = Client::new();
        assert!(Items::get_all(&client).unwrap().items().len() >= 2000)
    }
}
