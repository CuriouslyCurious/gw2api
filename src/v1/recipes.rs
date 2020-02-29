use crate::client::Client;
use crate::error::ApiError;
use crate::utils::parse_response;

const ENDPOINT_URL: &str = "/v1/recipes";

/// Contains ids of all player discovered recipes. Details about a particular recipe can be
/// obtained from the v1/recipe_details resource.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Recipes {
    /// List of ids of all discovered recipes.
    recipes: Vec<u32>,
}

impl Recipes {
    /// Retrieve all discovered recipes' ids.
    pub fn get_all(client: &Client) -> Result<Recipes, ApiError> {
        parse_response(&mut client.request(ENDPOINT_URL)?)
    }

    /// Returns the list of all player discovered recipe ids.
    pub fn recipes(&self) -> &Vec<u32> {
        &self.recipes
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::recipes::*;
    use crate::client::Client;

    const JSON_RECIPES: &str = r#"
    {
      "recipes": [
        1275,
        3147
      ]
    }"#;

    #[test]
    fn create_recipes() {
        match serde_json::from_str::<Recipes>(JSON_RECIPES) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_all_recipes() {
        let client = Client::new();
        assert!(Recipes::get_all(&client).unwrap().recipes().len() >= 2000)
    }
}
