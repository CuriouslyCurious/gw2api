use serde::Deserialize;

use std::collections::HashMap;

use gw2api_core::attributes::Attribute;

/// Returns information about the PvP amulets.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Amulet {
    /// id of the amulet. NOTE: These are not necessarily in succession in the API.
    pub id: u32,
    /// Name of the amulet.
    pub name: String,
    /// A url to an image representing the amulet in-game.
    #[serde(rename = "icon")]
    pub icon_url: String,
    /// HashMap containing all the amulets attribute names as keys and the attribute values
    /// as the values.
    pub attributes: HashMap<Attribute, i32>,
}

// impl Amulet {
//     /// Retrieve an amulet by its id.
//     pub fn get_id(client: &Client, id: u32) -> Result<Amulet, ApiError> {
//         let url = format!("{}?id={}", ENDPOINT_URL, id);
//         client.request(&url)
//     }
//
//     /// Retrieve all ids for the available PvP amulets, returning a `Vec` of ids.
//     pub fn get_all_ids(client: &Client) -> Result<Vec<u32>, ApiError> {
//         client.request(ENDPOINT_URL)
//     }
//
//     /// Retrieve multiple amulets by their ids, if any of the ids do not exist it will not be in the
//     /// returned `Vec`, if all the ids are invalid the `Vec` will be empty.
//     pub fn get_amulets_by_ids(client: &Client, ids: Vec<u32>) -> Result<Vec<Amulet>, ApiError> {
//         let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
//         client.request(&url)
//     }
//
//     /// Retrieve all available amulets, returning a `Vec` of `Amulet` objects.
//     pub fn get_all_amulets(client: &Client) -> Result<Vec<Amulet>, ApiError> {
//         let url = format!("{}?ids=all", ENDPOINT_URL);
//         client.request(&url)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::v2::pvp::amulets::*;

    const JSON_AMULET: &str = r#"
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

    #[test]
    fn create_amulet() {
        serde_json::from_str::<Amulet>(JSON_AMULET).unwrap();
    }

    // #[test]
    // fn get_all_ids() {
    //     let client = Client::new();
    //     // Arbitrary number that the official number of amulets should exceed
    //     let num_amulets = 12;
    //     assert!(Amulet::get_all_ids(&client).unwrap().len() >= num_amulets);
    // }
    //
    // #[test]
    // fn get_amulet_by_id() {
    //     let client = Client::new();
    //     let amulet = serde_json::from_str::<Amulet>(JSON_AMULET).unwrap();
    //     assert_eq!(amulet, Amulet::get_id(&client, amulet.id).unwrap());
    // }
    //
    // #[test]
    // fn get_ids() {
    //     let client = Client::new();
    //     let mut ids: Vec<u32> = Vec::new();
    //     ids.push(1);
    //     ids.push(90909); // does not exist
    //     let amulets = Amulet::get_amulets_by_ids(&client, ids).unwrap();
    //     assert!(amulets.len() == 1);
    // }
    //
    // #[test]
    // fn get_all_amulets() {
    //     let client = Client::new();
    //     // Arbitrary number that the official number of amulets should exceed
    //     let num_amulets = 12;
    //     let amulets = Amulet::get_all_amulets(&client).unwrap();
    //     assert!(amulets.len() >= num_amulets);
    // }
}
