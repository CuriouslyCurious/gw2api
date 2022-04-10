use httpmock::prelude::*;
use serde::de::DeserializeOwned;
use similar_asserts::assert_eq;

use gw2api::client::Client;

use std::path::PathBuf;
use std::fs::read_to_string;
use std::fmt::Debug;

const JSON_PATH: &str = "./tests/json/";

pub fn mock_endpoint<T: DeserializeOwned + Debug + PartialEq>(endpoint: &str) -> T {
    let endpoint = endpoint.strip_prefix('/').unwrap().to_owned();
    // Coincidentally this just works out
    let mut path = PathBuf::from(JSON_PATH.clone());
    path.push(endpoint.clone() + ".json");

    let json = read_to_string(&path).unwrap();
    let expected = serde_json::from_str::<T>(&json).unwrap();

    let server = MockServer::start();
    let m = server.mock(|when, then| {
        when.path("/".to_owned() + &endpoint);
        then.status(200)
            .body_from_file(path.to_str().unwrap());
    });

    let client = Client::new().set_base_url(server.base_url());
    let result = client.request(&endpoint).unwrap();

    // Check that the server has received a request
    m.assert();

    assert_eq!(expected, result);
    result
}
