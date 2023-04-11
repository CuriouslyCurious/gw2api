use httpmock::prelude::*;
use similar_asserts::assert_eq;

use gw2api_core::client::{Client, RequestBuilder};
use gw2api_core::endpoint::Endpoint;

use std::fs::read_to_string;
use std::path::{Path, PathBuf};

const JSON_PATH: &str = "./tests/json/";
// Note: this is not a valid API key
const API_KEY: &str = "";

macro_rules! mock {
    ($type:ty, $endpoint:expr) => {
        crate::common::mock_endpoint::<$type>($endpoint, None, None);
    };
    ($endpoint:expr, $path:expr) => {
        crate::common::mock_endpoint::<$type>($endpoint, $path, None);
    };
}

macro_rules! mock_with_builder {
    ($type:ty, $endpoint:expr, $builder:expr) => {
        mock_endpoint::<$type>($endpoint, None, $builder);
    };
    ($type:ty, $endpoint:expr, $auth:expr, $builder:expr) => {
        mock_endpoint::<$type>($endpoint, $auth, $builder);
    };
}

pub fn mock_endpoint<T>(endpoint: &str, path: Option<&Path>, req_builder: Option<RequestBuilder>) -> T
where
    T: Endpoint,
{
    let endpoint_path = endpoint
        .strip_prefix('/')
        .expect("failed to strip prefix")
        .to_owned();
    let mut json_path = PathBuf::from(JSON_PATH.to_owned());
    match path {
        Some(p) => json_path.push(p),
        None => json_path.push(endpoint_path.clone() + ".json"),
    };
    let json = read_to_string(&json_path).expect("failed to read path");
    let expected = serde_json::from_str::<T>(&json).expect("file was not valid json");

    let server = MockServer::start();
    let m = server.mock(|when, then| {
        when.path(endpoint);
        then.status(200).body_from_file(
            json_path
                .to_str()
                .expect("could not read file to endpoint body"),
        );
    });

    let response = minreq::get(format!("http://{}", m.server_address().to_string())).send().unwrap();
    dbg!(response);

    let mut client = Client::new();
    client.set_base_url(server.base_url());
    // client.set_api_key(API_KEY.to_owned());
    let response = T::get(&client).expect("failed to get endpoint");
    // client
    //     .authenticated_request(&endpoint)
    //     .expect("authenticated endpoint request failed")
    // } else {
    //     client.request(&endpoint).expect("endpoint request failed")
    // };

   // Check that the server has received a request
    m.assert();

    assert_eq!(response, expected);
    response
}
