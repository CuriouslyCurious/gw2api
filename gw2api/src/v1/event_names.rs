use serde::Deserialize;

use gw2api_derive::Endpoint;

/// NOTE: This endpoint is disabled, and will only return HTTP error codes.
/// Struct containing the id and (localized) event name.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/event_names"]
#[localised = true]
pub struct EventName {
    /// Event id. First digit indicates the event's region: 1 = North America, 2 = Europe.
    pub id: String,
    /// Localized name of the event.
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::v1::event_names::EventName;

    const JSON_EVENT: &str = r#"
    {
      "id": "A3A8140E-A1E3-466E-97B8-F3F80A47538D",
      "name": "Capture the beast."
    }"#;

    #[test]
    fn create_event() {
        serde_json::from_str::<EventName>(JSON_EVENT).unwrap();
    }

    //#[test]
    //fn get_all_events() {
    //    let client = Client::new();
    //    assert!(Event::get_all(&client).is_err())
    //}
}
