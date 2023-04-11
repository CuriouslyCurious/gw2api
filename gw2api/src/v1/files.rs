use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Struct containing all possible files.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/files"]
pub struct Files {
    pub map_complete: File,
    pub map_dungeon: File,
    pub map_heart_empty: File,
    pub map_heart_full: File,
    pub map_node_harvesting: File,
    pub map_node_logging: File,
    pub map_node_mining: File,
    pub map_poi: File,
    pub map_special_event: File,
    pub map_story: File,
    pub map_waypoint: File,
    pub map_waypoint_contested: File,
    pub map_waypoint_hover: File,
    pub map_vista: File,
    pub map_heropoint: File,
    pub wvw_battles_hollow_blue: File,
    pub wvw_battles_hollow_green: File,
    pub wvw_battles_hollow_red: File,
    pub wvw_battles_hollow_white: File,
    pub wvw_bauers_estate_blue: File,
    pub wvw_bauers_estate_green: File,
    pub wvw_bauers_estate_red: File,
    pub wvw_bauers_estate_white: File,
    pub wvw_carvers_ascent_blue: File,
    pub wvw_carvers_ascent_green: File,
    pub wvw_carvers_ascent_red: File,
    pub wvw_carvers_ascent_white: File,
    pub wvw_orchard_overlook_blue: File,
    pub wvw_orchard_overlook_green: File,
    pub wvw_orchard_overlook_red: File,
    pub wvw_orchard_overlook_white: File,
    pub wvw_temple_of_lost_prayers_blue: File,
    pub wvw_temple_of_lost_prayers_green: File,
    pub wvw_temple_of_lost_prayers_red: File,
    pub wvw_temple_of_lost_prayers_white: File,
    pub wvw_camp: File,
    pub wvw_tower: File,
    pub wvw_keep: File,
    pub wvw_castle: File,
    pub ui_upgrade_slot_open: File,
    pub ui_infusion_slot_agony: File,
    pub ui_infusion_slot_defensive: File,
    pub ui_infusion_slot_offensive: File,
    pub ui_infusion_slot_utility: File,
    pub ui_coin_gold: File,
    pub ui_coin_silver: File,
    pub ui_coin_copper: File,
    pub ui_gem: File,
    pub ui_supply: File,
    pub ui_minor_trait_mask: File,
    pub ui_major_trait_mask: File,
    pub icon_guardian: File,
    pub icon_warrior: File,
    pub icon_revenant: File,
    pub icon_necromancer: File,
    pub icon_mesmer: File,
    pub icon_elementalist: File,
    pub icon_thief: File,
    pub icon_ranger: File,
    pub icon_engineer: File,
    pub icon_guardian_big: File,
    pub icon_warrior_big: File,
    pub icon_revenant_big: File,
    pub icon_necromancer_big: File,
    pub icon_mesmer_big: File,
    pub icon_elementalist_big: File,
    pub icon_thief_big: File,
    pub icon_ranger_big: File,
    pub icon_engineer_big: File,
    pub map_crafting_scribe: File,
    pub map_bank: File,
    pub map_guild_bank: File,
    pub map_trading_post: File,
    pub map_crafting_armorsmith: File,
    pub map_crafting_artificer: File,
    pub map_crafting_cook: File,
    pub map_crafting_huntsman: File,
    pub map_crafting_jeweler: File,
    pub map_crafting_leatherworker: File,
    pub map_crafting_tailor: File,
    pub map_crafting_weaponsmith: File,
    pub map_guild_registrar: File,
    pub map_profession_trainer: File,
    pub map_repair: File,
    pub map_vendor: File,
    pub map_vendor_armor: File,
    pub map_vendor_weapons: File,
    pub map_vendor_mystic_forge: File,
    pub map_vendor_laurel: File,
    pub map_fractal: File,
    pub map_raid_entrance: File,
    pub map_adventure: File,
    pub map_adventure_complete: File,
    pub map_adventure_locked: File,
    pub map_outpost: File,
    pub map_outpost_locked: File,
    pub map_outpost_contested: File,
    pub map_outpost_active: File,
    pub map_vendor_festival: File,
    pub map_guild_armorer: File,
    pub map_guild_weaponsmith: File,
    pub map_vendor_ecto: File,
    pub map_vendor_guild: File,
    pub map_vendor_cultural_armor: File,
    pub map_vendor_cultural_weapons: File,
    pub map_vendor_crystalline_ore: File,
    pub map_vendor_airship_parts: File,
    pub map_vendor_aurillium: File,
    pub map_vendor_leyline_crystals: File,
    pub map_vendor_dungeon: File,
    pub map_elevator_down: File,
    pub map_elevator_up: File,
    pub map_ramp_down: File,
    pub map_ramp_up: File,
    pub map_stairs_down: File,
    pub map_stairs_up: File,
}

/// Contains information about an event.
#[derive(Debug, Deserialize, PartialEq)]
pub struct File {
    /// File id of the asset.
    #[serde(rename = "file_id")]
    pub id: u32,
    /// File signature of the asset.
    pub signature: String,
}

impl File {}

#[cfg(test)]
mod tests {
    use crate::v1::files::*;

    // map_stairs_up
    const JSON_FILE: &str = r#"
    {
      "file_id": 102442,
      "signature": "0F80D8D900E1F356B41F752594BEFCE52693DEB3"
    }"#;

    #[test]
    fn create_event() {
        serde_json::from_str::<File>(JSON_FILE).unwrap();
    }
}
