use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Struct containing an unordered list of (localized) WvW objective names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Objective {
    /// Objective id.
    pub id: String,
    /// Localized name of the objective.
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::v1::wvw::objective_names::*;

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
