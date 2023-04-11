//! Module for testing all the v1 endpoints with at the time current json output from the official
//! endpoints which are then mocked, so no need for reliance on the GW2 API's uptime.
//!
//! I collected them all here since it was easier for me to bundle them all together here instead
//! of separated into their corresponding files. They might end up being split again, however.

#[macro_use]
mod common;

use gw2api::v1::wvw::Match;
use gw2api::v1::wvw::Matches;
use gw2api::v1::wvw::Objective;
use gw2api::v1::Build;
use gw2api::v1::Colors;
use gw2api::v1::Continents;
use gw2api::v1::Events;
use gw2api::v1::Files;
use gw2api::v1::Floor;
use gw2api::v1::Guild;
use gw2api::v1::Item;
use gw2api::v1::MapName;
use gw2api::v1::Maps;
use gw2api::v1::Recipe;
use gw2api::v1::Recipes;
use gw2api::v1::Skin;
use gw2api::v1::Skins;
use gw2api::v1::World;

// v1/build.rs
#[test]
fn get_build() {
    mock!(Build, "/v1/build");
}

// v1/colors.rs
#[test]
fn get_all_dyes() {
    mock!(Colors, "/v1/colors");
}

// v1/continents.rs
#[test]
fn get_all_continents() {
    mock!(Continents, "/v1/continents");
}

// v1/event_details.rs
#[test]
fn get_all_events() {
    mock!(Events, "/v1/event_details");
}

// v1/event_names.rs skipped since it is deprecated

// v1/files.rs
#[test]
fn get_all_files() {
    mock!(Files, "/v1/files");
}

// v1/guild_details.rs
#[test]
fn get_guild_by_id() {
    mock!(Guild, "/v1/guild_details");
}

//#[test]
//fn get_guild_by_name() {
//    mock!("/v1/guild_details/name");
//    let client = Client::new();
//    let name = "The Doppelgangers";
//    assert_eq!(Guild::get_by_name(&client, name.to_string()).unwrap().name, name)
//}

// v1/item_details.rs
#[test]
fn get_armor() {
    mock!(Item, "/v1/item_details/armor");
}

#[test]
fn get_back() {
    mock!(Item, "/v1/item_details/back");
}

#[test]
fn get_bag() {
    mock!(Item, "/v1/item_details/bag");
}

#[test]
fn get_consumable_recipe() {
    mock!(Item, "/v1/item_details/consumable_recipe");
}

#[test]
fn get_consumable_dye() {
    mock!(Item, "/v1/item_details/consumable_dye");
}

#[test]
fn get_consumable_others() {
    mock!(Item, "/v1/item_details/consumable_others");
}

#[test]
fn get_consumable_generic_with_effect() {
    mock!(Item, "/v1/item_details/consumable_generic_with_effect");
}

#[test]
fn get_consumable_generic_without_effect() {
    mock!(Item, "/v1/item_details/consumable_generic_without_effect");
}

#[test]
fn get_consumable_booze() {
    mock!(Item, "/v1/item_details/consumable_booze");
}

#[test]
fn get_consumable_transmutation() {
    mock!(Item, "/v1/item_details/consumable_transmutation");
}

#[test]
fn get_consumable_untransmutation() {
    mock!(Item, "/v1/item_details/consumable_untransmutation");
}

#[test]
fn get_consumable_immediate() {
    mock!(Item, "/v1/item_details/consumable_immediate");
}

#[test]
fn get_consumable_food() {
    mock!(Item, "/v1/item_details/consumable_food");
}

#[test]
fn get_utility() {
    mock!(Item, "/v1/item_details/utility");
}

#[test]
fn get_utility_without_effect() {
    mock!(Item, "/v1/item_details/utility_without_effect");
}

#[test]
fn get_utility_halloween() {
    mock!(Item, "/v1/item_details/utility_halloween");
}
#[test]
fn get_utility_contractnpc() {
    mock!(Item, "/v1/item_details/utility_contractnpc");
}

#[test]
fn get_utility_upgraderemoval() {
    mock!(Item, "/v1/item_details/utility_upgraderemoval");
}

#[test]
fn get_utility_appearancechange() {
    mock!(Item, "/v1/item_details/utility_appearancechange");
}

#[test]
fn get_container() {
    mock!(Item, "/v1/item_details/container");
}

#[test]
fn get_crafting_material() {
    mock!(Item, "/v1/item_details/crafting_material");
}

#[test]
fn get_gathering() {
    mock!(Item, "/v1/item_details/gathering");
}

#[test]
fn get_gizmo() {
    mock!(Item, "/v1/item_details/gizmo");
}

#[test]
fn get_mini_pet() {
    mock!(Item, "/v1/item_details/mini_pet");
}

#[test]
fn get_trinket() {
    mock!(Item, "/v1/item_details/trinket");
}

#[test]
fn get_trophy() {
    mock!(Item, "/v1/item_details/trophy");
}

#[test]
fn get_upgrade_component() {
    mock!(Item, "/v1/item_details/upgrade_component");
}

// v1/map_floor.rs
/* A mix of Ruins of Orr for god shrines and Tangled Depths for adventures. This endpoint
 * is such a mess. */
// #[test]
// fn get_map_floor() {
//     mock!("/v1/map_floor");
// }
//
// // v1/map_names.rs
// #[test]
// fn get_all_map_names() {
//     mock!("/v1/map_names");
// }

// v1/maps.rs
#[test]
fn get_all_maps() {
    mock!(Maps, "/v1/maps");
}

// disabled
// v1/recipe_details.rs
#[test]
fn get_recipe_details() {
    mock!(Recipe, "/v1/recipe_details");
}

// v1/recipes.rs
#[test]
fn get_all_recipes() {
    mock!(Recipes, "/v1/recipes");
}

// v1/skin_details.rs
#[test]
fn get_all_skin_details() {
    mock!(Skin, "/v1/skin_details");
}

// v1/skins.rs
#[test]
fn get_all_skins() {
    mock!(Skins, "/v1/skins");
}

// v1/world_names.rs
#[test]
fn get_all_world_names() {
    mock!(World, "/v1/world_names");
}

// v1/wvw/match_details.rs
#[test]
fn get_match_details() {
    mock!(Match, "/v1/wvw/match_details");
}

// v1/wvw/matches.rs
#[test]
fn get_matches() {
    mock!(Matches, "/v1/wvw/matches");
}

// v1/wvw/objective_names.rs
// #[test]
// fn get_objective_names() {
//     mock!("/v1/wvw/objective_names");
// }
