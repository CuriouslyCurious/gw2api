use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;
use crate::utils::ids_to_string;

const ENDPOINT_URL: &str = "/v2/pvp/standings";

/// Information about a player's SPvP pips for all PvP seasons, and rating for seasons 5 and up.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Standing {
    /// Current or what the standing was at the end of the season.
    pub current: Current,
    /// The best standing during the season.
    pub best: Best,
}

/// Information of the current pip values and rating, or final standings if the season has ended.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Current {
    /// Total number of pips collected.
    pub total_points: u32,
    /// Player's division. 0-indexed and refers to [Season](division).
    pub division: u32,
    /// Player's tier. 0-indexed and refers to [Season](tier).
    pub tier: u32,
}

/// Information
#[derive(Debug, Deserialize, PartialEq)]
pub struct Best {

}
