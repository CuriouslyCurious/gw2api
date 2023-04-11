//! API version 1 endpoints.

mod build;
mod colors;
mod continents;
mod event_details;
mod event_names;
mod files;
mod guild_details;
mod item_details;
mod items;
mod map_floor;
mod map_names;
mod maps;
mod recipe_details;
mod recipes;
mod skin_details;
mod skins;
mod world_names;
pub mod wvw;

// Re-export the separate files so it looks prettier for users.
pub use crate::v1::build::Build;
pub use crate::v1::colors::Colors;
pub use crate::v1::continents::Continents;
#[deprecated = "No longer responds to requests, use v2::event_details instead"]
pub use crate::v1::event_details::Events;
#[deprecated = "No longer responds to requests, use v2::events instead"]
pub use crate::v1::event_names::EventName;
pub use crate::v1::files::Files;
pub use crate::v1::guild_details::Guild;
pub use crate::v1::item_details::Item;
pub use crate::v1::items::Items;
pub use crate::v1::map_floor::Floor;
pub use crate::v1::map_names::MapName;
pub use crate::v1::maps::Maps;
pub use crate::v1::recipe_details::Recipe;
pub use crate::v1::recipes::Recipes;
pub use crate::v1::skin_details::Skin;
pub use crate::v1::skins::Skins;
pub use crate::v1::world_names::World;
