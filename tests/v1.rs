//! Module for testing all the v1 endpoints with at the time current json output from the official
//! endpoints which are then mocked, so no need for reliance on the GW2 API's uptime.
//!
//! I collected them all here since it was easier for me to bundle them all together here instead
//! of separated into their corresponding files. They might end up back there agian, however.

mod common;

use gw2api::v1::build::Build;
use gw2api::v1::colors::Colors;
use gw2api::v1::continents::Continents;
use gw2api::v1::event_details::Events;
use gw2api::v1::files::Files;
use gw2api::v1::guild_details::Guild;
use gw2api::v1::item_details::Item;
use gw2api::v1::map_floor::Floor;
use gw2api::v1::map_names::MapName;
use gw2api::v1::maps::Maps;
use gw2api::v1::recipe_details::Recipe;
use gw2api::v1::recipes::Recipes;
use gw2api::v1::skin_details::Skin;
use gw2api::v1::skins::Skins;
use gw2api::v1::world_names::World;
use gw2api::v1::wvw::match_details::Match;
use gw2api::v1::wvw::matches::Matches;
use gw2api::v1::wvw::objective_names::Objective;

use crate::common::request_endpoint;

// v1/build.rs
#[test]
fn get_build() {
    request_endpoint::<Build>("/v1/build");
}

// v1/colors.rs
#[test]
fn get_all_dyes() {
    request_endpoint::<Colors>("/v1/colors");
}

// v1/continents.rs
#[test]
fn get_all_continents() {
    request_endpoint::<Continents>("/v1/continents");
}

// v1/event_details.rs
#[test]
fn get_all_events() {
    request_endpoint::<Events>("/v1/event_details");
}

// v1/event_names.rs skipped since it is deprecated

// v1/files.rs
#[test]
fn get_all_files() {
    request_endpoint::<Files>("/v1/files");
}

// v1/guild_details.rs
#[test]
fn get_guild_by_id() {
    request_endpoint::<Guild>("/v1/guild_details");
}

//#[test]
//fn get_guild_by_name() {
//    request_endpoint::<Item>("/v1/guild_details/name");
//    let client = Client::new();
//    let name = "The Doppelgangers";
//    assert_eq!(Guild::get_by_name(&client, name.to_string()).unwrap().name, name)
//}

// v1/item_details.rs
#[test]
fn get_armor() {
    request_endpoint::<Item>("/v1/item_details/armor");
}

#[test]
fn get_back() {
    request_endpoint::<Item>("/v1/item_details/back");
}

#[test]
fn get_bag() {
    request_endpoint::<Item>("/v1/item_details/bag");
}

#[test]
fn get_consumable_recipe() {
    request_endpoint::<Item>("/v1/item_details/consumable_recipe");
}

#[test]
fn get_consumable_dye() {
    request_endpoint::<Item>("/v1/item_details/consumable_dye");
}

#[test]
fn get_consumable_others() {
    request_endpoint::<Item>("/v1/item_details/consumable_others");
}

#[test]
fn get_consumable_generic_with_effect() {
    request_endpoint::<Item>("/v1/item_details/consumable_generic_with_effect");
}

#[test]
fn get_consumable_generic_without_effect() {
    request_endpoint::<Item>("/v1/item_details/consumable_generic_without_effect");
}

#[test]
fn get_consumable_booze() {
    request_endpoint::<Item>("/v1/item_details/consumable_booze");
}

#[test]
fn get_consumable_transmutation() {
    request_endpoint::<Item>("/v1/item_details/consumable_transmutation");
}

#[test]
fn get_consumable_untransmutation() {
    request_endpoint::<Item>("/v1/item_details/consumable_untransmutation");
}

#[test]
fn get_consumable_immediate() {
    request_endpoint::<Item>("/v1/item_details/consumable_immediate");
}

#[test]
fn get_consumable_food() {
    request_endpoint::<Item>("/v1/item_details/consumable_food");
}

#[test]
fn get_utility() {
    request_endpoint::<Item>("/v1/item_details/utility");
}

#[test]
fn get_utility_without_effect() {
    request_endpoint::<Item>("/v1/item_details/utility_without_effect");
}

#[test]
fn get_utility_halloween() {
    request_endpoint::<Item>("/v1/item_details/utility_halloween");
}
#[test]
fn get_utility_contractnpc() {
    request_endpoint::<Item>("/v1/item_details/utility_contractnpc");
}

#[test]
fn get_utility_upgraderemoval() {
    request_endpoint::<Item>("/v1/item_details/utility_upgraderemoval");
}

#[test]
fn get_utility_appearancechange() {
    request_endpoint::<Item>("/v1/item_details/utility_appearancechange");
}

#[test]
fn get_container() {
    request_endpoint::<Item>("/v1/item_details/container");
}

#[test]
fn get_crafting_material() {
    request_endpoint::<Item>("/v1/item_details/crafting_material");
}

#[test]
fn get_gathering() {
    request_endpoint::<Item>("/v1/item_details/gathering");
}

#[test]
fn get_gizmo() {
    request_endpoint::<Item>("/v1/item_details/gizmo");
}

#[test]
fn get_mini_pet() {
    request_endpoint::<Item>("/v1/item_details/mini_pet");
}

#[test]
fn get_trinket() {
    request_endpoint::<Item>("/v1/item_details/trinket");
}

#[test]
fn get_trophy() {
    request_endpoint::<Item>("/v1/item_details/trophy");
}

#[test]
fn get_upgrade_component() {
    request_endpoint::<Item>("/v1/item_details/upgrade_component");
}

// v1/map_floor.rs
/* A mix of Ruins of Orr for god shrines and Tangled Depths for adventures. This endpoint
 * is such a mess. */
#[test]
fn get_map_floor() {
    request_endpoint::<Floor>("/v1/map_floor");
}

// v1/map_names.rs
#[test]
fn get_all_map_names() {
    request_endpoint::<Vec<MapName>>("/v1/map_names");
}

// v1/maps.rs
#[test]
fn get_all_maps() {
    request_endpoint::<Maps>("/v1/maps");
}

// disabled
// v1/recipe_details.rs
#[test]
fn get_recipe_details() {
    request_endpoint::<Recipe>("/v1/recipe_details");
}

// v1/recipes.rs
#[test]
fn get_all_recipes() {
    request_endpoint::<Recipes>("/v1/recipes");
}

// v1/skin_details.rs
#[test]
fn get_all_skin_details() {
    request_endpoint::<Skin>("/v1/skin_details");
}

// v1/skins.rs
#[test]
fn get_all_skins() {
    request_endpoint::<Skins>("/v1/skins");
}

// v1/world_names.rs
#[test]
fn get_all_world_names() {
    request_endpoint::<Vec<World>>("/v1/world_names");
}

// v1/wvw/match_details.rs
#[test]
fn get_match_details() {
    request_endpoint::<Match>("/v1/wvw/match_details");
}

// v1/wvw/matches.rs
#[test]
fn get_matches() {
    request_endpoint::<Matches>("/v1/wvw/matches");
}

// v1/wvw/objective_names.rs
#[test]
fn get_objective_names() {
    request_endpoint::<Vec<Objective>>("/v1/wvw/objective_names");
}
