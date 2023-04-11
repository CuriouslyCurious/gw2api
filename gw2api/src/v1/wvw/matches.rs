use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Struct containing information about currently running WvW matches. Further information can be
/// requested using the match_details endpoint.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/wvw/matches"]
pub struct Matches {
    /// List of objects describing currently running WvW matches.
    #[serde(rename = "wvw_matches")]
    pub matches: Vec<Match>,
}

/// Contains information about a WvW match, like when it started & ended and the ids of the worlds.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Match {
    /// Match id.
    #[serde(rename = "wvw_match_id")]
    pub match_id: String,
    /// World id of the red world.
    pub red_world_id: u32,
    /// World id of the blue world.
    pub blue_world_id: u32,
    /// World id of the green world.
    pub green_world_id: u32,
    /// Start time of the match.
    pub start_time: String,
    /// End time of the match.
    pub end_time: String,
}

impl Match {}

#[cfg(test)]
mod tests {
    use gw2api_core::client::Client;
    use crate::v1::wvw::matches::*;

    const JSON_MATCHES: &str = r#"
    {
      "wvw_matches": [
        {
          "wvw_match_id": "2-2",
          "red_world_id": 2104,
          "blue_world_id": 2301,
          "green_world_id": 2202,
          "start_time": "2013-08-02T18:00:00Z",
          "end_time": "2013-08-09T18:00:00Z"
        },
        {
          "wvw_match_id": "1-3",
          "red_world_id": 1016,
          "blue_world_id": 1021,
          "green_world_id": 1014,
          "start_time": "2013-08-03T01:00:00Z",
          "end_time": "2013-08-10T01:00:00Z"
        }
      ]
    }"#;

    #[test]
    fn create_matches() {
        serde_json::from_str::<Matches>(JSON_MATCHES).unwrap();
    }

    #[test]
    fn get_all_matches() {
        let client = Client::new();
        // Currently 8 matches are running simultaneously, so 4 should be a sufficient check.
        // assert!(Matches::get(&client).unwrap().matches.len() >= 4)
    }
}
