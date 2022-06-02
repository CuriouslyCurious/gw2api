use serde::Deserialize;

use std::collections::HashMap;

use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{ids_to_string, Profession, Team};

const ENDPOINT_URL: &str = "/v2/pvp/games";

/// A structured PvP game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Game {
    /// The game's UUID.
    pub id: String,
    /// Map id of the map the game was played on.
    pub map_id: u32,
    /// Timestamp of when the match was started.
    #[serde(rename = "started")]
    pub start_time: String,
    /// Timestamp of when the match ended.
    #[serde(rename = "ended")]
    pub end_time: String,
    /// Result of the match, win or loss.
    pub result: String,
    /// Which team the player was on during the match.
    pub team: Team,
    /// Profession that was played during the match by the player.
    pub profession: Profession,
    /// Scores of both teams during the match.
    pub scores: HashMap<Team, u32>,
    /// Type of game that was played.
    pub rating_type: RatingType,
    /// Amount which the given player's rating changed, if it was played during a season.
    pub rating_change: Option<i32>,
    /// Season id of the game was played in, if it was played during a season.
    pub season: Option<String>,
}

/// Possible types of Structured PvP games.
#[derive(Debug, Deserialize, PartialEq)]
pub enum RatingType {
    Ranked,
    Unranked,
    #[serde(rename(deserialize = "None"))]
    Custom,
}

impl Game {
    /// Retrieve a game by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Game, ApiError> {
        let url = format!("{}?id={}", ENDPOINT_URL, id);
        client.authenticated_request(&url)
    }

    /// Retrieve ids of all recently played games.
    pub fn get_all_ids(client: &Client) -> Result<Vec<String>, ApiError> {
        client.authenticated_request(ENDPOINT_URL)
    }

    /// Retrieve all games that have been played, capped at 10 most recent games.
    pub fn get_all_games(client: &Client) -> Result<Vec<Game>, ApiError> {
        let url = format!("{}?ids=all", ENDPOINT_URL);
        client.authenticated_request(&url)
    }

    /// Retrive games by their ids.
    pub fn get_games_by_ids(client: &Client, ids: Vec<String>) -> Result<Vec<Game>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        client.authenticated_request(&url)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::error::ApiError;
    use crate::v2::pvp::games::{Game, Profession, RatingType, Team};
    use std::collections::HashMap;
    use std::env;

    const JSON_GAME: &str = r#"{
            "id": "ABCDE02B-8888-FEBA-1234-DE98765C7DEF",
            "map_id": 894,
            "started": "2015-07-08T21:29:50.000Z",
            "ended": "2015-07-08T21:37:02.000Z",
            "result": "Defeat",
            "team": "Red",
            "profession": "Guardian",
            "scores": {
              "red": 165,
              "blue":507
            },
            "rating_type" : "Ranked",
            "rating_change" : -26,
            "season" : "49CCE661-9DCC-473B-B106-666FE9942721"
         }"#;

    #[test]
    fn create_game() {
        serde_json::from_str::<Game>(JSON_GAME).unwrap();
    }

    #[test]
    fn get_all_games() {
        let api_key =
            env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let mut client = Client::new();
        client.set_api_key(api_key);
        Game::get_all_games(&client).unwrap();
    }

    // Since, the most recent PvP games are both dependant on the key and who played them, testing
    // against a particular game in a static unit test would not work, so we instead test the error
    // state, which means the function is behaving properly on errors at least.
    #[test]
    fn get_invalid_id() {
        let api_key =
            env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let mut client = Client::new();
        client.set_api_key(api_key);
        let id = "1".to_string();
        // TODO:
        //assert_eq!(Err(ApiError::new("{\"text\":\"no such id\"}".to_string())), Game::get_id(&client, id.clone()));
    }

    #[test]
    fn get_invalid_games_by_ids() {
        let api_key =
            env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let ids = vec!["1".to_string(), "2".to_string()];
        let mut client = Client::new();
        client.set_api_key(api_key);
        //assert_eq!(Err(ApiError::new("{\"text\":\"all ids provided are invalid\"}".to_string())), Game::get_games_by_ids(&client, ids));
    }
}
