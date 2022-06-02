use httpmock::prelude::*;
use serde::de::DeserializeOwned;
use similar_asserts::assert_eq;

use gw2api::client::Client;

use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

const JSON_PATH: &str = "./tests/json/";
// Note: this is not a valid API key
const API_KEY: &str = "";

pub fn auth_endpoint<T: DeserializeOwned + Debug + PartialEq>(endpoint: &str) -> T {
    mock_endpoint(endpoint, true, None)
}

pub fn auth_endpoint_with_path<T: DeserializeOwned + Debug + PartialEq>(
    endpoint: &str,
    path: &Path,
) -> T {
    mock_endpoint(endpoint, true, Some(path))
}

pub fn request_endpoint<T: DeserializeOwned + Debug + PartialEq>(endpoint: &str) -> T {
    mock_endpoint(endpoint, false, None)
}

pub fn request_endpoint_with_path<T: DeserializeOwned + Debug + PartialEq>(
    endpoint: &str,
    path: &Path,
) -> T {
    mock_endpoint(endpoint, false, Some(path))
}

fn mock_endpoint<T: DeserializeOwned + Debug + PartialEq>(
    endpoint: &str,
    use_authentication: bool,
    path: Option<&Path>,
) -> T {
    let endpoint = endpoint
        .strip_prefix('/')
        .expect("failed to strip prefix")
        .to_owned();
    let mut json_path = PathBuf::from(JSON_PATH.to_owned());
    match path {
        Some(p) => json_path.push(p),
        None => json_path.push(endpoint.clone() + ".json"),
    };

    let json = read_to_string(&json_path).expect("failed to read path");
    let expected = serde_json::from_str::<T>(&json).expect("file was not valid json");

    let server = MockServer::start();
    let m = server.mock(|when, then| {
        when.path("/".to_owned() + &endpoint);
        then.status(200).body_from_file(
            json_path
                .to_str()
                .expect("could not read file to endpoint body"),
        );
    });

    let mut client = Client::new();
    client.set_base_url(server.base_url());
    let result = if use_authentication {
        client.set_api_key(API_KEY.to_owned());
        client.authenticated_request(&endpoint).expect("authenticated endpoint request failed")
    } else {
        client.request(&endpoint).expect("endpoint request failed")
    };

    // Check that the server has received a request
    m.assert();

    assert_eq!(expected, result);
    result
}
