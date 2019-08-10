use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{ids_to_string, parse_response};

const ENDPOINT_URL: &str = "/v2/pvp/games";

/// A structured PvP game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Game {
    /// The game's UUID.
    id: u32,
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
    scores: Vec<Score>,
    /// Type of game that was played.
    rating_type: RatingType,
    /// Amount which the given player's rating changed
    rating_change: i32,
    /// Season id of the game was played in, if it was played during a season.
    season: Option<String>,
}

/// Possible teams used in WvW or SPvP.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Team {
    Red,
    Green,
    Blue,
}

/// All the professions currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum Profession {
    Revenant,
    Warrior,
    Guardian,
    Thief,
    Ranger,
    Engineer,
    Necromancer,
    Mesmer,
    Elementalist,
}

/// Score that can be in a WvW match or SPvP game.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Score {
    team: Team,
    score: u32,
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
    pub fn get_id(client: &Client, id: u32) -> Result<Game, ApiError> {
        let url = format!("{}?id={}", ENDPOINT_URL, id);
        parse_response(&mut client.authenticated_request(&url)?)
    }

    /// Retrieve all games that have been played, capped at 10 most recent games.
    pub fn get_all_games(client: &Client) -> Result<Vec<Game>, ApiError> {
        parse_response(&mut client.authenticated_request(ENDPOINT_URL)?)
    }

    /// Retrive games by their ids.
    pub fn get_games_by_ids(client: &Client, ids: Vec<u32>) -> Result<Vec<Game>, ApiError> {
        let url = format!("{}?ids={}", ENDPOINT_URL, ids_to_string(ids));
        parse_response(&mut client.authenticated_request(&url)?)
    }

    /// Returns the requested id of the game.
    pub fn id(&self) -> u32 {
        self.id
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
    pub fn player(&self) -> &Profession {
        &self.profession
    }

    /// Returns the scores of both teams during the match.
    pub fn scores(&self) -> &Vec<Score> {
        &self.scores
    }

    /// Returns the type of game that was played (ranked, unranked or custom).
    pub fn rating_type(&self) -> &RatingType {
        &self.rating_type
    }

    /// Returns the amount which the given player's rating changed.
    pub fn rating_change(&self) -> i32 {
        self.rating_change
    }

    /// Returns an `Option` either containing the season id which the game was played in, or None.
    pub fn season(&self) -> &Option<String> {
        &self.season
    }
}

#[cfg(test)]
mod tests {

}

