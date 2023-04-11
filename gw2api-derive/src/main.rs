use gw2api_derive::ParamEndpoint;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug, ParamEndpoint)]
#[endpoint = "/v2/api/gay?id=1?build=2"]
#[localised = true]
pub struct Resource {
    id: u32,
    name: String,
    description: String,
}

fn main() {
    let _ = Resource {
        id: 0,
        name: "Wish I wasn't like this".to_owned(),
        description: "Suffering is eternal".to_owned(),
    };
}
