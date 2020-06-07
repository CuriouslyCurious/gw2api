use crate::client::Client;
use crate::error::ApiError;


const ENDPOINT_URL: &str = "/v1/files";

/// Struct containing all possible files.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Files {
    map_complete: File,
    map_dungeon: File,
    map_heart_empty: File,
    map_heart_full: File,
    map_node_harvesting: File,
    map_node_logging: File,
    map_node_mining: File,
    map_poi: File,
    map_special_event: File,
    map_story: File,
    map_waypoint: File,
    map_waypoint_contested: File,
    map_waypoint_hover: File,
    map_vista: File,
    map_heropoint: File,
    wvw_battles_hollow_blue: File,
    wvw_battles_hollow_green: File,
    wvw_battles_hollow_red: File,
    wvw_battles_hollow_white: File,
    wvw_bauers_estate_blue: File,
    wvw_bauers_estate_green: File,
    wvw_bauers_estate_red: File,
    wvw_bauers_estate_white: File,
    wvw_carvers_ascent_blue: File,
    wvw_carvers_ascent_green: File,
    wvw_carvers_ascent_red: File,
    wvw_carvers_ascent_white: File,
    wvw_orchard_overlook_blue: File,
    wvw_orchard_overlook_green: File,
    wvw_orchard_overlook_red: File,
    wvw_orchard_overlook_white: File,
    wvw_temple_of_lost_prayers_blue: File,
    wvw_temple_of_lost_prayers_green: File,
    wvw_temple_of_lost_prayers_red: File,
    wvw_temple_of_lost_prayers_white: File,
    wvw_camp: File,
    wvw_tower: File,
    wvw_keep: File,
    wvw_castle: File,
    ui_upgrade_slot_open: File,
    ui_infusion_slot_agony: File,
    ui_infusion_slot_defensive: File,
    ui_infusion_slot_offensive: File,
    ui_infusion_slot_utility: File,
    ui_coin_gold: File,
    ui_coin_silver: File,
    ui_coin_copper: File,
    ui_gem: File,
    ui_supply: File,
    ui_minor_trait_mask: File,
    ui_major_trait_mask: File,
    icon_guardian: File,
    icon_warrior: File,
    icon_revenant: File,
    icon_necromancer: File,
    icon_mesmer: File,
    icon_elementalist: File,
    icon_thief: File,
    icon_ranger: File,
    icon_engineer: File,
    icon_guardian_big: File,
    icon_warrior_big: File,
    icon_revenant_big: File,
    icon_necromancer_big: File,
    icon_mesmer_big: File,
    icon_elementalist_big: File,
    icon_thief_big: File,
    icon_ranger_big: File,
    icon_engineer_big: File,
    map_crafting_scribe: File,
    map_bank: File,
    map_guild_bank: File,
    map_trading_post: File,
    map_crafting_armorsmith: File,
    map_crafting_artificer: File,
    map_crafting_cook: File,
    map_crafting_huntsman: File,
    map_crafting_jeweler: File,
    map_crafting_leatherworker: File,
    map_crafting_tailor: File,
    map_crafting_weaponsmith: File,
    map_guild_registrar: File,
    map_profession_trainer: File,
    map_repair: File,
    map_vendor: File,
    map_vendor_armor: File,
    map_vendor_weapons: File,
    map_vendor_mystic_forge: File,
    map_vendor_laurel: File,
    map_fractal: File,
    map_raid_entrance: File,
    map_adventure: File,
    map_adventure_complete: File,
    map_adventure_locked: File,
    map_outpost: File,
    map_outpost_locked: File,
    map_outpost_contested: File,
    map_outpost_active: File,
    map_vendor_festival: File,
    map_guild_armorer: File,
    map_guild_weaponsmith: File,
    map_vendor_ecto: File,
    map_vendor_guild: File,
    map_vendor_cultural_armor: File,
    map_vendor_cultural_weapons: File,
    map_vendor_crystalline_ore: File,
    map_vendor_airship_parts: File,
    map_vendor_aurillium: File,
    map_vendor_leyline_crystals: File,
    map_vendor_dungeon: File,
    map_elevator_down: File,
    map_elevator_up: File,
    map_ramp_down: File,
    map_ramp_up: File,
    map_stairs_down: File,
    map_stairs_up: File,
}

/// Contains information about an event.
#[derive(Debug, Deserialize, PartialEq)]
pub struct File {
    /// File id of the asset.
    #[serde(rename = "file_id")]
    id: u32,
    /// File signature of the asset.
    signature: String,
}

impl Files {
    /// Retrieve information about commonly requested in-game assets (icons) that are in the game.
    pub fn get_all_files(client: &Client) -> Result<Files, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Return File object for map_complete
    pub fn map_complete(&self) -> &File {
        &self.map_complete
    }

    /// Return File object for map_dungeon
    pub fn map_dungeon(&self) -> &File {
        &self.map_dungeon
    }

    /// Return File object for map_heart_empty
    pub fn map_heart_empty(&self) -> &File {
        &self.map_heart_empty
    }

    /// Return File object for map_heart_full
    pub fn map_heart_full(&self) -> &File {
        &self.map_heart_full
    }

    /// Return File object for map_node_harvesting
    pub fn map_node_harvesting(&self) -> &File {
        &self.map_node_harvesting
    }

    /// Return File object for map_node_logging
    pub fn map_node_logging(&self) -> &File {
        &self.map_node_logging
    }

    /// Return File object for map_node_mining
    pub fn map_node_mining(&self) -> &File {
        &self.map_node_mining
    }

    /// Return File object for map_poi
    pub fn map_poi(&self) -> &File {
        &self.map_poi
    }

    /// Return File object for map_special_event
    pub fn map_special_event(&self) -> &File {
        &self.map_special_event
    }

    /// Return File object for map_story
    pub fn map_story(&self) -> &File {
        &self.map_story
    }

    /// Return File object for map_waypoint
    pub fn map_waypoint(&self) -> &File {
        &self.map_waypoint
    }

    /// Return File object for map_waypoint_contested
    pub fn map_waypoint_contested(&self) -> &File {
        &self.map_waypoint_contested
    }

    /// Return File object for map_waypoint_hover
    pub fn map_waypoint_hover(&self) -> &File {
        &self.map_waypoint_hover
    }

    /// Return File object for map_vista
    pub fn map_vista(&self) -> &File {
        &self.map_vista
    }

    /// Return File object for map_heropoint
    pub fn map_heropoint(&self) -> &File {
        &self.map_heropoint
    }

    /// Return File object for wvw_battles_hollow_blue
    pub fn wvw_battles_hollow_blue(&self) -> &File {
        &self.wvw_battles_hollow_blue
    }

    /// Return File object for wvw_battles_hollow_green
    pub fn wvw_battles_hollow_green(&self) -> &File {
        &self.wvw_battles_hollow_green
    }

    /// Return File object for wvw_battles_hollow_red
    pub fn wvw_battles_hollow_red(&self) -> &File {
        &self.wvw_battles_hollow_red
    }

    /// Return File object for wvw_battles_hollow_white
    pub fn wvw_battles_hollow_white(&self) -> &File {
        &self.wvw_battles_hollow_white
    }

    /// Return File object for wvw_bauers_estate_blue
    pub fn wvw_bauers_estate_blue(&self) -> &File {
        &self.wvw_bauers_estate_blue
    }

    /// Return File object for wvw_bauers_estate_green
    pub fn wvw_bauers_estate_green(&self) -> &File {
        &self.wvw_bauers_estate_green
    }

    /// Return File object for wvw_bauers_estate_red
    pub fn wvw_bauers_estate_red(&self) -> &File {
        &self.wvw_bauers_estate_red
    }

    /// Return File object for wvw_bauers_estate_white
    pub fn wvw_bauers_estate_white(&self) -> &File {
        &self.wvw_bauers_estate_white
    }

    /// Return File object for wvw_carvers_ascent_blue
    pub fn wvw_carvers_ascent_blue(&self) -> &File {
        &self.wvw_carvers_ascent_blue
    }

    /// Return File object for wvw_carvers_ascent_green
    pub fn wvw_carvers_ascent_green(&self) -> &File {
        &self.wvw_carvers_ascent_green
    }

    /// Return File object for wvw_carvers_ascent_red
    pub fn wvw_carvers_ascent_red(&self) -> &File {
        &self.wvw_carvers_ascent_red
    }

    /// Return File object for wvw_carvers_ascent_white
    pub fn wvw_carvers_ascent_white(&self) -> &File {
        &self.wvw_carvers_ascent_white
    }

    /// Return File object for wvw_orchard_overlook_blue
    pub fn wvw_orchard_overlook_blue(&self) -> &File {
        &self.wvw_orchard_overlook_blue
    }

    /// Return File object for wvw_orchard_overlook_green
    pub fn wvw_orchard_overlook_green(&self) -> &File {
        &self.wvw_orchard_overlook_green
    }

    /// Return File object for wvw_orchard_overlook_red
    pub fn wvw_orchard_overlook_red(&self) -> &File {
        &self.wvw_orchard_overlook_red
    }

    /// Return File object for wvw_orchard_overlook_white
    pub fn wvw_orchard_overlook_white(&self) -> &File {
        &self.wvw_orchard_overlook_white
    }

    /// Return File object for wvw_temple_of_lost_prayers_blue
    pub fn wvw_temple_of_lost_prayers_blue(&self) -> &File {
        &self.wvw_temple_of_lost_prayers_blue
    }

    /// Return File object for wvw_temple_of_lost_prayers_green
    pub fn wvw_temple_of_lost_prayers_green(&self) -> &File {
        &self.wvw_temple_of_lost_prayers_green
    }

    /// Return File object for wvw_temple_of_lost_prayers_red
    pub fn wvw_temple_of_lost_prayers_red(&self) -> &File {
        &self.wvw_temple_of_lost_prayers_red
    }

    /// Return File object for wvw_temple_of_lost_prayers_white
    pub fn wvw_temple_of_lost_prayers_white(&self) -> &File {
        &self.wvw_temple_of_lost_prayers_white
    }

    /// Return File object for wvw_camp
    pub fn wvw_camp(&self) -> &File {
        &self.wvw_camp
    }

    /// Return File object for wvw_tower
    pub fn wvw_tower(&self) -> &File {
        &self.wvw_tower
    }

    /// Return File object for wvw_keep
    pub fn wvw_keep(&self) -> &File {
        &self.wvw_keep
    }

    /// Return File object for wvw_castle
    pub fn wvw_castle(&self) -> &File {
        &self.wvw_castle
    }

    /// Return File object for ui_upgrade_slot_open
    pub fn ui_upgrade_slot_open(&self) -> &File {
        &self.ui_upgrade_slot_open
    }

    /// Return File object for ui_infusion_slot_agony
    pub fn ui_infusion_slot_agony(&self) -> &File {
        &self.ui_infusion_slot_agony
    }

    /// Return File object for ui_infusion_slot_defensive
    pub fn ui_infusion_slot_defensive(&self) -> &File {
        &self.ui_infusion_slot_defensive
    }

    /// Return File object for ui_infusion_slot_offensive
    pub fn ui_infusion_slot_offensive(&self) -> &File {
        &self.ui_infusion_slot_offensive
    }

    /// Return File object for ui_infusion_slot_utility
    pub fn ui_infusion_slot_utility(&self) -> &File {
        &self.ui_infusion_slot_utility
    }

    /// Return File object for ui_coin_gold
    pub fn ui_coin_gold(&self) -> &File {
        &self.ui_coin_gold
    }

    /// Return File object for ui_coin_silver
    pub fn ui_coin_silver(&self) -> &File {
        &self.ui_coin_silver
    }

    /// Return File object for ui_coin_copper
    pub fn ui_coin_copper(&self) -> &File {
        &self.ui_coin_copper
    }

    /// Return File object for ui_gem
    pub fn ui_gem(&self) -> &File {
        &self.ui_gem
    }

    /// Return File object for ui_supply
    pub fn ui_supply(&self) -> &File {
        &self.ui_supply
    }

    /// Return File object for ui_minor_trait_mask
    pub fn ui_minor_trait_mask(&self) -> &File {
        &self.ui_minor_trait_mask
    }

    /// Return File object for ui_major_trait_mask
    pub fn ui_major_trait_mask(&self) -> &File {
        &self.ui_major_trait_mask
    }

    /// Return File object for icon_guardian
    pub fn icon_guardian(&self) -> &File {
        &self.icon_guardian
    }

    /// Return File object for icon_warrior
    pub fn icon_warrior(&self) -> &File {
        &self.icon_warrior
    }

    /// Return File object for icon_revenant
    pub fn icon_revenant(&self) -> &File {
        &self.icon_revenant
    }

    /// Return File object for icon_necromancer
    pub fn icon_necromancer(&self) -> &File {
        &self.icon_necromancer
    }

    /// Return File object for icon_mesmer
    pub fn icon_mesmer(&self) -> &File {
        &self.icon_mesmer
    }

    /// Return File object for icon_elementalist
    pub fn icon_elementalist(&self) -> &File {
        &self.icon_elementalist
    }

    /// Return File object for icon_thief
    pub fn icon_thief(&self) -> &File {
        &self.icon_thief
    }

    /// Return File object for icon_ranger
    pub fn icon_ranger(&self) -> &File {
        &self.icon_ranger
    }

    /// Return File object for icon_engineer
    pub fn icon_engineer(&self) -> &File {
        &self.icon_engineer
    }

    /// Return File object for icon_guardian_big
    pub fn icon_guardian_big(&self) -> &File {
        &self.icon_guardian_big
    }

    /// Return File object for icon_warrior_big
    pub fn icon_warrior_big(&self) -> &File {
        &self.icon_warrior_big
    }

    /// Return File object for icon_revenant_big
    pub fn icon_revenant_big(&self) -> &File {
        &self.icon_revenant_big
    }

    /// Return File object for icon_necromancer_big
    pub fn icon_necromancer_big(&self) -> &File {
        &self.icon_necromancer_big
    }

    /// Return File object for icon_mesmer_big
    pub fn icon_mesmer_big(&self) -> &File {
        &self.icon_mesmer_big
    }

    /// Return File object for icon_elementalist_big
    pub fn icon_elementalist_big(&self) -> &File {
        &self.icon_elementalist_big
    }

    /// Return File object for icon_thief_big
    pub fn icon_thief_big(&self) -> &File {
        &self.icon_thief_big
    }

    /// Return File object for icon_ranger_big
    pub fn icon_ranger_big(&self) -> &File {
        &self.icon_ranger_big
    }

    /// Return File object for icon_engineer_big
    pub fn icon_engineer_big(&self) -> &File {
        &self.icon_engineer_big
    }

    /// Return File object for map_crafting_scribe
    pub fn map_crafting_scribe(&self) -> &File {
        &self.map_crafting_scribe
    }

    /// Return File object for map_bank
    pub fn map_bank(&self) -> &File {
        &self.map_bank
    }

    /// Return File object for map_guild_bank
    pub fn map_guild_bank(&self) -> &File {
        &self.map_guild_bank
    }

    /// Return File object for map_trading_post
    pub fn map_trading_post(&self) -> &File {
        &self.map_trading_post
    }

    /// Return File object for map_crafting_armorsmith
    pub fn map_crafting_armorsmith(&self) -> &File {
        &self.map_crafting_armorsmith
    }

    /// Return File object for map_crafting_artificer
    pub fn map_crafting_artificer(&self) -> &File {
        &self.map_crafting_artificer
    }

    /// Return File object for map_crafting_cook
    pub fn map_crafting_cook(&self) -> &File {
        &self.map_crafting_cook
    }

    /// Return File object for map_crafting_huntsman
    pub fn map_crafting_huntsman(&self) -> &File {
        &self.map_crafting_huntsman
    }

    /// Return File object for map_crafting_jeweler
    pub fn map_crafting_jeweler(&self) -> &File {
        &self.map_crafting_jeweler
    }

    /// Return File object for map_crafting_leatherworker
    pub fn map_crafting_leatherworker(&self) -> &File {
        &self.map_crafting_leatherworker
    }

    /// Return File object for map_crafting_tailor
    pub fn map_crafting_tailor(&self) -> &File {
        &self.map_crafting_tailor
    }

    /// Return File object for map_crafting_weaponsmith
    pub fn map_crafting_weaponsmith(&self) -> &File {
        &self.map_crafting_weaponsmith
    }

    /// Return File object for map_guild_registrar
    pub fn map_guild_registrar(&self) -> &File {
        &self.map_guild_registrar
    }

    /// Return File object for map_profession_trainer
    pub fn map_profession_trainer(&self) -> &File {
        &self.map_profession_trainer
    }

    /// Return File object for map_repair
    pub fn map_repair(&self) -> &File {
        &self.map_repair
    }

    /// Return File object for map_vendor
    pub fn map_vendor(&self) -> &File {
        &self.map_vendor
    }

    /// Return File object for map_vendor_armor
    pub fn map_vendor_armor(&self) -> &File {
        &self.map_vendor_armor
    }

    /// Return File object for map_vendor_weapons
    pub fn map_vendor_weapons(&self) -> &File {
        &self.map_vendor_weapons
    }

    /// Return File object for map_vendor_mystic_forge
    pub fn map_vendor_mystic_forge(&self) -> &File {
        &self.map_vendor_mystic_forge
    }

    /// Return File object for map_vendor_laurel
    pub fn map_vendor_laurel(&self) -> &File {
        &self.map_vendor_laurel
    }

    /// Return File object for map_fractal
    pub fn map_fractal(&self) -> &File {
        &self.map_fractal
    }

    /// Return File object for map_raid_entrance
    pub fn map_raid_entrance(&self) -> &File {
        &self.map_raid_entrance
    }

    /// Return File object for map_adventure
    pub fn map_adventure(&self) -> &File {
        &self.map_adventure
    }

    /// Return File object for map_adventure_complete
    pub fn map_adventure_complete(&self) -> &File {
        &self.map_adventure_complete
    }

    /// Return File object for map_adventure_locked
    pub fn map_adventure_locked(&self) -> &File {
        &self.map_adventure_locked
    }

    /// Return File object for map_outpost
    pub fn map_outpost(&self) -> &File {
        &self.map_outpost
    }

    /// Return File object for map_outpost_locked
    pub fn map_outpost_locked(&self) -> &File {
        &self.map_outpost_locked
    }

    /// Return File object for map_outpost_contested
    pub fn map_outpost_contested(&self) -> &File {
        &self.map_outpost_contested
    }

    /// Return File object for map_outpost_active
    pub fn map_outpost_active(&self) -> &File {
        &self.map_outpost_active
    }

    /// Return File object for map_vendor_festival
    pub fn map_vendor_festival(&self) -> &File {
        &self.map_vendor_festival
    }

    /// Return File object for map_guild_armorer
    pub fn map_guild_armorer(&self) -> &File {
        &self.map_guild_armorer
    }

    /// Return File object for map_guild_weaponsmith
    pub fn map_guild_weaponsmith(&self) -> &File {
        &self.map_guild_weaponsmith
    }

    /// Return File object for map_vendor_ecto
    pub fn map_vendor_ecto(&self) -> &File {
        &self.map_vendor_ecto
    }

    /// Return File object for map_vendor_guild
    pub fn map_vendor_guild(&self) -> &File {
        &self.map_vendor_guild
    }

    /// Return File object for map_vendor_cultural_armor
    pub fn map_vendor_cultural_armor(&self) -> &File {
        &self.map_vendor_cultural_armor
    }

    /// Return File object for map_vendor_cultural_weapons
    pub fn map_vendor_cultural_weapons(&self) -> &File {
        &self.map_vendor_cultural_weapons
    }

    /// Return File object for map_vendor_crystalline_ore
    pub fn map_vendor_crystalline_ore(&self) -> &File {
        &self.map_vendor_crystalline_ore
    }

    /// Return File object for map_vendor_airship_parts
    pub fn map_vendor_airship_parts(&self) -> &File {
        &self.map_vendor_airship_parts
    }

    /// Return File object for map_vendor_aurillium
    pub fn map_vendor_aurillium(&self) -> &File {
        &self.map_vendor_aurillium
    }

    /// Return File object for map_vendor_leyline_crystals
    pub fn map_vendor_leyline_crystals(&self) -> &File {
        &self.map_vendor_leyline_crystals
    }

    /// Return File object for map_vendor_dungeon
    pub fn map_vendor_dungeon(&self) -> &File {
        &self.map_vendor_dungeon
    }

    /// Return File object for map_elevator_down
    pub fn map_elevator_down(&self) -> &File {
        &self.map_elevator_down
    }

    /// Return File object for map_elevator_up
    pub fn map_elevator_up(&self) -> &File {
        &self.map_elevator_up
    }

    /// Return File object for map_ramp_down
    pub fn map_ramp_down(&self) -> &File {
        &self.map_ramp_down
    }

    /// Return File object for map_ramp_up
    pub fn map_ramp_up(&self) -> &File {
        &self.map_ramp_up
    }

    /// Return File object for map_stairs_down
    pub fn map_stairs_down(&self) -> &File {
        &self.map_stairs_down
    }

    /// Return File object for map_stairs_up
    pub fn map_stairs_up(&self) -> &File {
        &self.map_stairs_up
    }
}

impl File {
    /// Returns the file id of the asset.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the signature of the asset..
    pub fn signature(&self) -> &str {
        &self.signature
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::files::*;
    use crate::client::Client;

    // map_stairs_up
    const JSON_FILE: &str = r#"
    {
      "file_id": 102442,
      "signature": "0F80D8D900E1F356B41F752594BEFCE52693DEB3"
    }"#;

    #[test]
    fn create_event() {
        match serde_json::from_str::<File>(JSON_FILE) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_files() {
        let client = Client::new();
        let file = serde_json::from_str::<File>(JSON_FILE).unwrap();
        assert_eq!(Files::get_all_files(&client).unwrap().map_stairs_up(), &file)
    }
}
