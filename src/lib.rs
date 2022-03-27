#![warn(clippy::all)]
//! gw2api is a fairly simple wrapper over the Guild Wars 2 API.
//!
//! ```
//! use gw2api::client::Client;
//! use gw2api::v1::build::Build;
//!
//! let client = Client::new();
//! let build = Build::get_build(&client).unwrap();
//! println!("Current build id: {}", build.id());
//! ```
//!

pub use serde::{Deserialize, Serialize};

// Client
pub mod client;
pub mod error;
pub mod utils;

// Object and enum definitions
pub mod attributes;

// Endpoints
pub mod v1;
pub mod v2;

