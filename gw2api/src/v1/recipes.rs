use serde::Deserialize;

use gw2api_derive::Endpoint;

/// Contains ids of all player discovered recipes. Details about a particular recipe can be
/// obtained from the v1/recipe_details resource.
#[derive(Endpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/recipes"]
pub struct Recipes {
    /// List of ids of all discovered recipes.
    pub recipes: Vec<u32>,
}

// impl Recipes {
//     /// Retrieve all discovered recipes' ids.
//     pub fn get_all(client: &Client) -> Result<Recipes, ApiError> {
//         client.request(ENDPOINT_URL)
//     }
// }
