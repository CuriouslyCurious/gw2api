use crate::client::Client;
use crate::error::ApiError;
use crate::utils::ids_to_string;

const ENDPOINT_URL: &str = "/v2/pvp/heroes";

/// A hero used in the Stronghold game structured PvP game type.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Hero {
    /// id of the hero.
    id: String,
    /// Name of the hero.
    name: String,
    /// Flavor text describing the lore behind the hero.
    description: String,
    /// Flavor type describing the hero.
    #[serde(rename = "type")]
    flavor_type: String,
    /// A struct containing the champion's stats: offense, defense and speed.
    stats: Stats,
    /// Url to the overlay art for the champion.
    #[serde(rename = "overlay")]
    overlay_url: String,
    /// Url to the underlay art for the champion.
    #[serde(rename = "underlay")]
    underlay_url: String,
    /// A `Vec` of the skins available to the given hero.
    skins: Vec<Skin>,
}

/// Struct that contains the offense, defense and speed stats for a given hero.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Stats {
    offense: u32,
    defense: u32,
    speed: u32,
}

/// Cosmetic skin information of a hero.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Skin {
    /// Skin id
    id: u32,
    /// Name of the skin
    name: String,
    /// Url to the icon
    #[serde(rename = "icon")]
    icon_url: String,
    /// Whether the skin is the default for that hero or not.
    default: bool,
    /// Item ids which unlock the skin.
    #[serde(default)]
    unlock_items: Vec<u32>,
}

impl Hero {
    /// Retrieve a hero by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Hero, ApiError> {
        let url = format!("{}?id={}", ENDPOINT_URL, id);
        client.request(&url)
    }

    /// Retrieve the ids for all available heroes.
    pub fn get_all_ids(client: &Client) -> Result<Vec<String>, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Retrieve all heroes that are available.
    pub fn get_all_heroes(client: &Client) -> Result<Vec<Hero>, ApiError> {
        let url = format!("{}?ids=all", ENDPOINT_URL);
        client.request(&url)
    }

    /// Retrive heroes by their ids.
    pub fn get_heroes_by_ids(client: &Client, ids: Vec<String>) -> Result<Vec<Hero>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        client.request(&url)
    }

    /// Returns the requested id of the hero.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the hero.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the flavor description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the flavor type, e.g "Specialist Hero".
    pub fn flavor_type(&self) -> &str {
        &self.flavor_type
    }

    /// Returns the stats of the given hero.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Returns the url to the overlay art for the hero.
    pub fn overlay_url(&self) -> &str {
        &self.overlay_url
    }

    /// Returns the url to the underlay art for the hero.
    pub fn underlay_url(&self) -> &str {
        &self.underlay_url
    }

    /// Returns a `Vec` containing the available skins for the given hero.
    pub fn skins(&self) -> &Vec<Skin> {
        &self.skins
    }
}

impl Stats {
    /// Returns the offense stat of the hero.
    pub fn offense(&self) -> u32 {
        self.offense
    }

    /// Returns the defense stat of the hero.
    pub fn defense(&self) -> u32 {
        self.defense
    }

    /// Returns the speed stat of the hero.
    pub fn speed(&self) -> u32 {
        self.speed
    }
}

impl Skin {
    /// Returns the id for the hero skin.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the name of the skin.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the url for the skin's icon.
    pub fn icon_url(&self) -> &str {
        &self.icon_url
    }

    /// Returns true if the skin is the default for the hero, false otherwise.
    pub fn default(&self) -> bool {
        self.default
    }

    /// Returns the item ids of items that can unlock the skin.
    pub fn unlock_items(&self) -> &Vec<u32> {
        &self.unlock_items
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::pvp::heroes::*;
    use crate::client::Client;

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
        match serde_json::from_str::<Hero>(JSON_HERO) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn create_stats() {
        match serde_json::from_str::<Stats>(JSON_STATS) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn create_skin() {
        match serde_json::from_str::<Skin>(JSON_SKIN) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_id() {
        let client = Client::new();
        let hero = serde_json::from_str::<Hero>(JSON_HERO).unwrap();
        assert_eq!(hero, Hero::get_id(&client, hero.id().to_string()).unwrap());
    }

    #[test]
    fn get_all_ids() {
        let client = Client::new();
        let ids = vec!(
            "115C140F-C2F5-40EB-8EA2-C3773F2AE468",
            "B7EA9889-5F16-4636-9705-4FCAF8B39ECD",
            "BEA79596-CA8B-4D46-9B9C-EA1B606BCF42",
            "CF977AE5-C605-4586-A802-3E25F0F35772",
        );
        assert_eq!(ids, Hero::get_all_ids(&client).unwrap());
    }

    #[test]
    fn get_all_heroes() {
        let client = Client::new();
        let ids = vec!(
            "115C140F-C2F5-40EB-8EA2-C3773F2AE468",
            "B7EA9889-5F16-4636-9705-4FCAF8B39ECD",
            "BEA79596-CA8B-4D46-9B9C-EA1B606BCF42",
            "CF977AE5-C605-4586-A802-3E25F0F35772",
        );
        assert!(Hero::get_all_heroes(&client).unwrap().len() == ids.len());
    }

    #[test]
    fn get_heroes_by_ids() {
        let client = Client::new();
        let ids = vec!(
            "115C140F-C2F5-40EB-8EA2-C3773F2AE468".to_string(),
            "B7EA9889-5F16-4636-9705-4FCAF8B39ECD".to_string(),
        );
        assert!(Hero::get_heroes_by_ids(&client, ids.clone()).unwrap().len() == ids.len());
    }

    #[test]
    fn accessors() {
        let hero = serde_json::from_str::<Hero>(JSON_HERO).unwrap();
        assert_eq!("115C140F-C2F5-40EB-8EA2-C3773F2AE468", hero.id());
        assert_eq!("Nika", hero.name());
        assert_eq!("Nika was a proficient assassin schooled in her youth at Shing Jea Monastery. She served Cantha as a member of the Obsidian Flame.", hero.description());
        assert_eq!("Specialist Hero", hero.flavor_type());
        // Stats
        let stats = serde_json::from_str::<Stats>(JSON_STATS).unwrap();
        assert_eq!(&stats, hero.stats());
        assert_eq!(3, hero.stats().offense());
        assert_eq!(2, hero.stats().defense());
        assert_eq!(4, hero.stats().speed());
        assert_eq!("https://render.guildwars2.com/file/2CACF4120E370D1997A4C3D69BF592D7CC1870C8/993693.png", hero.overlay_url());
        assert_eq!("https://render.guildwars2.com/file/103108E0D8EDD22C577FA4171618D004A82AD955/993694.png", hero.underlay_url());
        // Skins
        let skin = serde_json::from_str::<Skin>(JSON_SKIN).unwrap();
        let hero_skin = hero.skins().first().unwrap();
        assert_eq!(&skin, hero_skin);
        assert_eq!(1, hero_skin.id());
        assert_eq!("Nika", hero_skin.name());
        assert_eq!("https://render.guildwars2.com/file/4602BDC15B73422011AC664425D93750707F04F3/1058576.png", hero_skin.icon_url());
        assert_eq!(true, hero_skin.default());
        assert_eq!(&vec!(70076), hero_skin.unlock_items());
    }
}

