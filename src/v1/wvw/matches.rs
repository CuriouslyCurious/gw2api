use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;

const ENDPOINT_URL: &str = "/v1/wvw/matches";

/// Struct containing information about currently running WvW matches. Further information can be
/// requested using the match_details endpoint.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Matches {
    /// List of objects describing currently running WvW matches.
    #[serde(rename = "wvw_matches")]
    matches: Vec<Match>,
}

/// Contains information about a WvW match, like when it started & ended and the ids of the worlds.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Match {
    /// Match id.
    #[serde(rename = "wvw_match_id")]
    match_id: String,
    /// World id of the red world.
    red_world_id: u32,
    /// World id of the blue world.
    blue_world_id: u32,
    /// World id of the green world.
    green_world_id: u32,
    /// Start time of the match.
    start_time: String,
    /// End time of the match.
    end_time: String,
}

impl Matches {
    /// Retrieve all currently running WvW matches.
    pub fn get_all(client: &Client) -> Result<Matches, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
    }

    /// Returns a list of WvW matches.
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }
}

impl Match {
    /// Returns the match id.
    pub fn match_id(&self) -> &str {
        &self.match_id
    }

    /// Returns the id for the red world.
    pub fn red_world_id(&self) -> u32 {
        self.red_world_id
    }

    /// Returns the id for the blue world.
    pub fn blue_world_id(&self) -> u32 {
        self.blue_world_id
    }

    /// Returns the id for the green world.
    pub fn green_world_id(&self) -> u32 {
        self.green_world_id
    }

    /// Returns the timestamp for when the match started.
    pub fn start_time(&self) -> &str {
        &self.start_time
    }

    /// Returns the timestamp for when the match ended.
    pub fn end_time(&self) -> &str {
        &self.end_time
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::wvw::matches::*;
    use crate::client::Client;

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
        match serde_json::from_str::<Matches>(JSON_MATCHES) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_matches() {
        let client = Client::new();
         // Currently 8 matches are running simultaneously, so 4 should be a sufficient check.
        assert!(Matches::get_all(&client).unwrap().matches().len() >= 4)
    }
}
