use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Contains ids of all player discovered items. Details about a particular item can be
/// obtained from the v1/item_details resource.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/items"]
pub struct Items {
    /// List of ids of all discovered items.
    pub items: Vec<u32>,
}


#[cfg(test)]
mod tests {
    use crate::v1::items::*;

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
}
