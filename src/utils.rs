use serde::Deserialize;

/// Possible teams used in WvW or SPvP.
#[derive(Debug, Deserialize, PartialEq, Hash, Eq)]
pub enum Team {
    #[serde(alias = "red")]
    Red,
    #[serde(alias = "green")]
    Green,
    #[serde(alias = "blue")]
    Blue,
    #[serde(alias = "neutral")]
    Neutral,
}

/// All the professions currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Profession {
    Revenant,
    Warrior,
    Guardian,
    Thief,
    Ranger,
    Engineer,
    Necromancer,
    Mesmer,
    Elementalist,
}

/// All possible races currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Race {
    Human,
    Norn,
    Sylvari,
    Asura,
    Charr,
    //Tengu when? :(
}

/// All crafting disciplines currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Discipline {
    Artificer,
    Armorsmith,
    Chef,
    Huntsman,
    Jeweler,
    Leatherworker,
    Scribe,
    Tailor,
    Weaponsmith,
}

/// All item rarities currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Rarity {
    Ascended,
    Basic,
    Exotic,
    Fine,
    Junk,
    Legendary,
    Masterwork,
    Rare,
}

/// All armor weight class.
#[derive(Debug, Deserialize, PartialEq)]
pub enum WeightClass {
    Clothing,
    Heavy,
    Light,
    Medium,
}


/// Convert a `Vec<T>` to a comma-separated `String`
pub fn ids_to_string(ids: Vec<impl ToString>) -> String {
    let mut ids: String = ids.iter().map(|id| format!("{},", id.to_string())).collect();
    ids.pop(); // Remove the last comma
    ids
}

