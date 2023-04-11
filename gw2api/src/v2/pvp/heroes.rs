use serde::Deserialize;

use gw2api_derive::ParamEndpoint;

/// A hero used in the Stronghold game structured PvP game type.
#[derive(ParamEndpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v2/pvp/heroes?hero_id={}"]
#[localised = true]
pub struct Hero {
    /// id of the hero.
    pub id: String,
    /// Name of the hero.
    pub name: String,
    /// Flavor text describing the lore behind the hero.
    pub description: String,
    /// Flavor type describing the hero.
    #[serde(rename = "type")]
    pub flavor_type: String,
    /// A struct containing the champion's stats: offense, defense and speed.
    pub stats: Stats,
    /// Url to the overlay art for the champion.
    #[serde(rename = "overlay")]
    pub overlay_url: String,
    /// Url to the underlay art for the champion.
    #[serde(rename = "underlay")]
    pub underlay_url: String,
    /// A `Vec` of the skins available to the given hero.
    pub skins: Vec<Skin>,
}

/// Struct that contains the offense, defense and speed stats for a given hero.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Stats {
    pub offense: u32,
    pub defense: u32,
    pub speed: u32,
}

/// Cosmetic skin information of a hero.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Skin {
    /// Skin id
    pub id: u32,
    /// Name of the skin
    pub name: String,
    /// Url to the icon
    #[serde(rename = "icon")]
    pub icon_url: String,
    /// Whether the skin is the default for that hero or not.
    pub default: bool,
    /// Item ids which unlock the skin.
    #[serde(default)]
    pub unlock_items: Vec<u32>,
}

impl Stats {}

impl Skin {}

#[cfg(test)]
mod tests {
    use crate::v2::pvp::heroes::*;

    const JSON_HERO: &str = r#"
    {
      "id": "115C140F-C2F5-40EB-8EA2-C3773F2AE468",
      "name": "Nika",
      "description": "Nika was a proficient assassin schooled in her youth at Shing Jea Monastery. She served Cantha as a member of the Obsidian Flame.",
      "type": "Specialist Hero",
      "stats": {
        "offense": 3,
        "defense": 2,
        "speed": 4
      },
      "overlay": "https://render.guildwars2.com/file/2CACF4120E370D1997A4C3D69BF592D7CC1870C8/993693.png",
      "underlay": "https://render.guildwars2.com/file/103108E0D8EDD22C577FA4171618D004A82AD955/993694.png",
      "skins": [
        {
          "id": 1,
          "name": "Nika",
          "icon": "https://render.guildwars2.com/file/4602BDC15B73422011AC664425D93750707F04F3/1058576.png",
          "default": true,
          "unlock_items": [
            70076
          ]
        },
        {
          "id": 7,
          "name": "Shadow Assassin Nika",
          "icon": "https://render.guildwars2.com/file/01643F1BD1202007BEE8E37F7DA3EA31AEE9536C/1322841.png",
          "default": false,
          "unlock_items": [
            72077
          ]
        },
        {
          "id": 15,
          "name": "Festive Nika",
          "icon": "https://render.guildwars2.com/file/002248777FC6341B1650040AF1ADBD79A4772CA5/1322839.png",
          "default": false,
          "unlock_items": [
            77642
          ]
        },
        {
          "id": 11,
          "name": "Sneakthief Nika",
          "icon": "https://render.guildwars2.com/file/DB2DCD0AEDDCD0474F4FC2426203384E06D2380D/1322842.png",
          "default": false,
          "unlock_items": [
            73002
          ]
        },
        {
          "id": 12,
          "name": "Strider's Nika",
          "icon": "https://render.guildwars2.com/file/CE35793C96D74CC657736D15FB02C7B64E610208/1322843.png",
          "default": false,
          "unlock_items": [
            76274
          ]
        }
      ]
    }"#;

    const JSON_STATS: &str = r#"
    {
        "offense": 3,
        "defense": 2,
        "speed": 4
    }"#;

    const JSON_SKIN: &str = r#"
    {
        "id": 1,
        "name": "Nika",
        "icon": "https://render.guildwars2.com/file/4602BDC15B73422011AC664425D93750707F04F3/1058576.png",
        "default": true,
        "unlock_items": [
          70076
        ]
    }"#;

    #[test]
    fn create_hero() {
        serde_json::from_str::<Hero>(JSON_HERO).unwrap();
    }

    #[test]
    fn create_stats() {
        serde_json::from_str::<Stats>(JSON_STATS).unwrap();
    }

    #[test]
    fn create_skin() {
        serde_json::from_str::<Skin>(JSON_SKIN).unwrap();
    }

    // #[test]
    // fn get_id() {
    //     let client = Client::new();
    //     let hero = serde_json::from_str::<Hero>(JSON_HERO).unwrap();
    //     assert_eq!(hero, Hero::get_id(&client, hero.id.to_string()).unwrap());
    // }
    //
    // #[test]
    // fn get_all_ids() {
    //     let client = Client::new();
    //     let ids = vec![
    //         "115C140F-C2F5-40EB-8EA2-C3773F2AE468",
    //         "B7EA9889-5F16-4636-9705-4FCAF8B39ECD",
    //         "BEA79596-CA8B-4D46-9B9C-EA1B606BCF42",
    //         "CF977AE5-C605-4586-A802-3E25F0F35772",
    //     ];
    //     assert_eq!(ids, Hero::get_all_ids(&client).unwrap());
    // }
    //
    // #[test]
    // fn get_all_heroes() {
    //     let client = Client::new();
    //     let ids = vec![
    //         "115C140F-C2F5-40EB-8EA2-C3773F2AE468",
    //         "B7EA9889-5F16-4636-9705-4FCAF8B39ECD",
    //         "BEA79596-CA8B-4D46-9B9C-EA1B606BCF42",
    //         "CF977AE5-C605-4586-A802-3E25F0F35772",
    //     ];
    //     assert!(Hero::get_all_heroes(&client).unwrap().len() == ids.len());
    // }
    //
    // #[test]
    // fn get_heroes_by_ids() {
    //     let client = Client::new();
    //     let ids = vec![
    //         "115C140F-C2F5-40EB-8EA2-C3773F2AE468".to_string(),
    //         "B7EA9889-5F16-4636-9705-4FCAF8B39ECD".to_string(),
    //     ];
    //     assert!(Hero::get_heroes_by_ids(&client, ids.clone()).unwrap().len() == ids.len());
    // }
}
