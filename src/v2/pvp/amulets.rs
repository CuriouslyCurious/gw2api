use crate::attributes::Attribute;
use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{ids_to_string, parse_response};

use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v2/pvp/amulets";

/// Returns information about the PvP amulets.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Amulet {
    /// id of the amulet. NOTE: These are not necessarily in succession in the API.
    id: u32,
    /// Name of the amulet.
    name: String,
    /// A url to an image representing the amulet in-game.
    icon: String,
    /// HashMap containing all the amulets attribute names as keys and the attribute values
    /// as the values.
    attributes: HashMap<Attribute, i32>,
}

impl Amulet {
    /// Get all ids for the available PvP amulets, returning a `Vec` of ids.
    pub fn get_ids(client: &Client) -> Result<Vec<u32>, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
    }

    /// Get an amulet by its id.
    pub fn get_amulet_by_id(client: &Client, id: u32) -> Result<Amulet, ApiError> {
        let url = format!("{}?id={}", ENDPOINT_URL, id);
        parse_response(&mut client.request(&url)?)
    }

    /// Get multiple amulets by their ids, if any of the ids do not exist it will not be in the
    /// returned `Vec`, if all the ids are invalid the `Vec` will be empty.
    pub fn get_amulets_by_ids(client: &Client, ids: Vec<u32>) -> Result<Vec<Amulet>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        parse_response(&mut client.request(&url)?)
    }

    /// Get all available amulets, returning a `Vec` of `Amulet` objects.
    pub fn get_all_amulets(client: &Client) -> Result<Vec<Amulet>, ApiError> {
        let url = format!("{}?ids=all", ENDPOINT_URL);
        parse_response(&mut client.request(&url)?)
    }

    /// Returns the ID of the `Amulet` object.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns a string slice of the name of the `Amulet`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a url to the icon for the `Amulet`.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns a `Vec` containing the attributes associated with the `Amulet`.
    pub fn attributes(&self) -> &HashMap<Attribute, i32> {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::pvp::amulets::Amulet;
    use crate::attributes::Attribute;
    use crate::client::Client;
    use std::collections::HashMap;

    #[test]
    fn create_amulet() {
        let json_amulet = r#"
        {
          "id": 4,
          "name": "Assassin Amulet",
          "icon": "https://render.guildwars2.com/file/02E9EFDEF9587130A25F17AC396913FBBE3C716D/455602.png",
          "attributes": {
            "Precision": 1200,
            "Power": 900,
            "CritDamage": 900
          }
        }"#;

        match serde_json::from_str::<Amulet>(json_amulet) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_ids() {
        let client = Client::new();
        // Current PvP amulet ids
        let ids: Vec<u32> = vec!(1, 4, 5, 7, 8, 9, 12, 13, 14, 18, 20, 22, 25, 28, 29,
                                 30, 31, 33, 34, 35, 36, 39, 40, 41, 42, 43, 44, 45);
        assert_eq!(ids, Amulet::get_ids(&client).unwrap());
    }

    #[test]
    fn get_amulet_by_id() {
        let client = Client::new();
        let mut attributes = HashMap::new();
        attributes.insert(Attribute::Precision, 1200);
        attributes.insert(Attribute::Power, 900);
        attributes.insert(Attribute::CritDamage, 900);
        let amulet = Amulet {
            id: 4,
            name: "Assassin Amulet".to_string(),
            icon: "https://render.guildwars2.com/file/02E9EFDEF9587130A25F17AC396913FBBE3C716D/455602.png".to_string(),
            attributes,
        };
        assert_eq!(amulet, Amulet::get_amulet_by_id(&client, amulet.id()).unwrap());
    }

    #[test]
    fn get_amulets_by_ids() {
        let client = Client::new();
        let mut ids: Vec<u32> = Vec::new();
        ids.push(1);
        ids.push(3); // does not exist
        let amulets = Amulet::get_amulets_by_ids(&client, ids).unwrap();
        assert!(amulets.len() == 1);
    }
}
