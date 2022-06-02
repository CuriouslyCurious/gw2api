use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/event_names";

/// NOTE: This endpoint is disabled, and will only return errors.
/// Struct containing an unordered list of (localized) event names.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    /// Event id. First digit indicates the event's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the event.
    pub name: String,
}

impl Event {
    /// Retrieve all event names that are in the game.
    #[allow(deprecated)]
    pub fn get_all(client: &Client) -> Result<Vec<Event>, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::event_names::Event;

    const JSON_EVENT: &str = r#"
    {
      "id": "A3A8140E-A1E3-466E-97B8-F3F80A47538D",
      "name": "Capture the beast."
    }"#;

    #[test]
    fn create_event() {
        serde_json::from_str::<Event>(JSON_EVENT).unwrap();
    }

    //#[test]
    //fn get_all_events() {
    //    let client = Client::new();
    //    assert!(Event::get_all(&client).is_err())
    //}
}
