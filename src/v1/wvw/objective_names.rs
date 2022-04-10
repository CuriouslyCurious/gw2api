use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/wvw/objective_names";

/// Struct containing an unordered list of (localized) WvW objective names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Objective {
    /// Objective id.
    pub id: String,
    /// Localized name of the objective.
    pub name: String,
}

impl Objective {
    /// Retrieve all objective names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<Objective>, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::wvw::objective_names::*;
    use crate::client::Client;

    const JSON_OBJECTIVE: &str = r#"
    {
      "id": "95-103",
      "name": "Border"
    }"#;

    #[test]
    fn create_objective() {
        serde_json::from_str::<Objective>(JSON_OBJECTIVE).unwrap();
    }
}
