use crate::client::Client;
use crate::error::ApiError;


const ENDPOINT_URL: &str = "/v1/wvw/objective_names";

/// Struct containing an unordered list of (localized) WvW objective names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Objective {
    /// Objective id.
    id: String,
    /// Localized name of the objective.
    name: String,
}

impl Objective {
    /// Retrieve all objective names that are in the game.
    pub fn get_all(client: &Client) -> Result<Vec<Objective>, ApiError> {
        client.request(ENDPOINT_URL)
    }

    /// Returns the id of the objective.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the localized name of the objective.
    pub fn name(&self) -> &str {
        &self.name
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
        match serde_json::from_str::<Objective>(JSON_OBJECTIVE) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_objectives() {
        let client = Client::new();
        let objective = serde_json::from_str::<Objective>(JSON_OBJECTIVE).unwrap();
        assert!(Objective::get_all(&client).unwrap().contains(&objective))
    }
}
