//! Module for testing all the v2 endpoints with at the time current json output from the official
//! endpoints which are then mocked, so no need for reliance on the GW2 API's uptime.
//!
//! I collected them all here since it was easier for me to bundle them all together here instead
//! of separated into their corresponding files. They might end up back there again, however.
//!
mod common;

use gw2api::v2::build::Build;
use gw2api::v2::tokeninfo::TokenInfo;

use crate::common::request_endpoint;

#[test]
fn get_build() {
    request_endpoint::<Build>("/v2/build");
}

#[test]
fn get_tokeninfo() {
    request_endpoint::<TokenInfo>("/v2/tokeninfo");
}

#[test]
fn get_worlds() {
    request_endpoint::<TokenInfo>("/v2/worlds");
}
