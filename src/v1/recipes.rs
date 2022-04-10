use serde::Deserialize;

use crate::client::Client;
use crate::error::ApiError;

const ENDPOINT_URL: &str = "/v1/recipes";

/// Contains ids of all player discovered recipes. Details about a particular recipe can be
/// obtained from the v1/recipe_details resource.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Recipes {
    /// List of ids of all discovered recipes.
    pub recipes: Vec<u32>,
}

impl Recipes {
    /// Retrieve all discovered recipes' ids.
    pub fn get_all(client: &Client) -> Result<Recipes, ApiError> {
        client.request(ENDPOINT_URL)
    }
}

