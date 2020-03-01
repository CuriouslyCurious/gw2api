use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{ids_to_string, parse_response, Profession, Team};
use std::collections::HashMap;

const ENDPOINT_URL: &str = "/v2/pvp/games";

/// A structured PvP game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Game {
    /// The game's UUID.
    id: String,
    /// Map id of the map the game was played on.
    map_id: u32,
    /// Timestamp of when the match was started.
    #[serde(rename = "started")]
    start_time: String,
    /// Timestamp of when the match ended.
    #[serde(rename = "ended")]
    end_time: String,
    /// Result of the match, win or loss.
    result: String,
    /// Which team the player was on during the match.
    team: Team,
    /// Profession that was played during the match by the player.
    profession: Profession,
    /// Scores of both teams during the match.
    scores: HashMap<Team, u32>,
    /// Type of game that was played.
    rating_type: RatingType,
    /// Amount which the given player's rating changed, if it was played during a season.
    rating_change: Option<i32>,
    /// Season id of the game was played in, if it was played during a season.
    season: Option<String>,
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
        parse_response(&mut client.authenticated_request(&url)?)
    }

    /// Retrieve ids of all recently played games.
    pub fn get_all_ids(client: &Client) -> Result<Vec<String>, ApiError> {
        parse_response(&mut client.authenticated_request(ENDPOINT_URL)?)
    }

    /// Retrieve all games that have been played, capped at 10 most recent games.
    pub fn get_all_games(client: &Client) -> Result<Vec<Game>, ApiError> {
        let url = format!("{}?ids=all", ENDPOINT_URL);
        parse_response(&mut client.authenticated_request(&url)?)
    }

    /// Retrive games by their ids.
    pub fn get_games_by_ids(client: &Client, ids: Vec<String>) -> Result<Vec<Game>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        parse_response(&mut client.authenticated_request(&url)?)
    }

    /// Returns the requested id of the game.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the map id of the map the game was played on.
    pub fn map_id(&self) -> u32 {
        self.map_id
    }

    /// Returns the timestamp of when the match was started.
    pub fn start_time(&self) -> &str {
        &self.start_time
    }

    /// Returns the timestamp of when the match ended.
    pub fn end_time(&self) -> &str {
        &self.end_time
    }

    /// Returns the tesult of the match, Victory or Defeat.
    pub fn result(&self) -> &str {
        &self.result
    }

    /// Returns which team the player was on during the match.
    pub fn team(&self) -> &Team {
        &self.team
    }

    /// Returns the profession that was played during the match by the player.
    pub fn profession(&self) -> &Profession {
        &self.profession
    }

    /// Returns the scores of both teams during the match.
    pub fn scores(&self) -> &HashMap<Team, u32> {
        &self.scores
    }

    /// Returns the type of game that was played (ranked, unranked or custom).
    pub fn rating_type(&self) -> &RatingType {
        &self.rating_type
    }

    /// Returns an `Option` either containing the amount which the given player's rating changed,
    /// or None.
    pub fn rating_change(&self) -> &Option<i32> {
        &self.rating_change
    }

    /// Returns an `Option` either containing the season id which the game was played in, or None.
    pub fn season(&self) -> &Option<String> {
        &self.season
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::pvp::games::{Game, Team, Profession, RatingType};
    use crate::client::Client;
    use crate::error::ApiError;
    use std::env;
    use std::collections::HashMap;

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
        match serde_json::from_str::<Game>(JSON_GAME) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string())
        }
    }

    #[test]
    fn accessors() {
        let game = serde_json::from_str::<Game>(JSON_GAME).unwrap();
        assert_eq!("ABCDE02B-8888-FEBA-1234-DE98765C7DEF", game.id());
        assert_eq!(894, game.map_id());
        assert_eq!("2015-07-08T21:29:50.000Z", game.start_time());
        assert_eq!("2015-07-08T21:37:02.000Z", game.end_time());
        assert_eq!("Defeat", game.result());
        assert_eq!(&Team::Red, game.team());
        assert_eq!(&Profession::Guardian, game.profession());
        let mut scores = HashMap::new();
        scores.insert(Team::Red, 165);
        scores.insert(Team::Blue, 507);
        assert_eq!(&scores, game.scores());
        assert_eq!(&RatingType::Ranked, game.rating_type());
        assert_eq!(&Some(-26), game.rating_change());
        assert_eq!(&Some("49CCE661-9DCC-473B-B106-666FE9942721".to_string()), game.season());
    }

    #[test]
    fn get_all_games() {
        let api_key = env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let client = Client::new().set_api_key(api_key);
        match Game::get_all_games(&client) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.description().to_string()),
        };
    }

    // Since, the most recent PvP games are both dependant on the key and who played them, testing
    // against a particular game in a static unit test would not work, so we instead test the error
    // state, which means the function is behaving properly on errors at least.
    #[test]
    fn get_invalid_id() {
        let api_key = env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let client = Client::new().set_api_key(api_key);
        let id = "1".to_string();
        assert_eq!(Err(ApiError::new("{\n  \"text\": \"no such id\"\n}".to_string())), Game::get_id(&client, id.clone()));
    }

    #[test]
    fn get_invalid_games_by_ids() {
        let api_key = env::var("GW2_TEST_KEY").expect("GW2_TEST_KEY environment variable is not set.");
        let ids = vec!("1".to_string(), "2".to_string());
        let client = Client::new().set_api_key(api_key);
        assert_eq!(Err(ApiError::new("{\n  \"text\": \"all ids provided are invalid\"\n}".to_string())), Game::get_games_by_ids(&client, ids));
    }

}
