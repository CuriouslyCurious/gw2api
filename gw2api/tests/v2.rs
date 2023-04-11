//! Module for testing all the v2 endpoints with at the time current json output from the official
//! endpoints which are then mocked, so no need for reliance on the GW2 API's uptime.
//!
//! I collected them all here since it was easier for me to bundle them all together here instead
//! of separated into their corresponding files. They might end up being split again, however.
//!
#[macro_use]
mod common;

use gw2api::v2::build::Build;
use gw2api::v2::tokeninfo::TokenInfo;
use gw2api::v2::worlds::World;

#[test]
fn get_build() {
    mock!(Build, "/v2/build");
}

#[test]
fn get_tokeninfo() {
    mock!(TokenInfo, "/v2/tokeninfo");
}

#[test]
fn get_worlds() {
    mock!(World, "/v2/worlds");
}
