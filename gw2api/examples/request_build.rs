use gw2api::v1::Build;
use gw2api_core::client::Client;
use gw2api_core::endpoint::Endpoint;

fn main() {
    let client = Client::new();
    let build = Build::get(&client).unwrap();
    println!("Build version: {}", build.id);
}
