use crate::client::Client;
use crate::error::ApiError;
use crate::utils::{parse_response, Discipline};

const ENDPOINT_URL: &str = "/v1/recipe_details";

/// Struct containing information about a requested recipe.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Recipe {
    /// id of the recipe.
    #[serde(rename = "recipe_id")]
    id: String,
    /// Type of the recipe.
    #[serde(rename = "type")]
    recipe_type: RecipeType,
    /// The id of the produced item.
    output_item_id: String,
    /// The number of the produced.
    output_item_count: String,
    /// Minimum rating of the recipe.
    min_rating: String,
    /// Time it takes to craft the item in ms.
    time_to_craft_ms: String,
    /// Potential value in coins when selling to a vendor.
    #[serde(default)]
    vendor_value: String,
    /// Crafting disciplines that can use the recipe.
    #[serde(default)]
    disciplines: Vec<Discipline>,
    /// Additional recipe flags.
    #[serde(default)]
    flags: Vec<RecipeFlags>,
    /// List of objects describing the ingredients used for this recipe.
    #[serde(default)]
    ingredients: Vec<Ingredient>,
}

/// Possible recipe types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum RecipeType {
    Amulet,
    Axe,
    Backpack,
    Bag,
    Boots,
    Bulk,
    Coat,
    Component,
    Consumable,
    Dagger,
    Dessert,
    Dye,
    Earring,
    Feast,
    Focus,
    Gloves,
    Greatsword,
    Hammer,
    Harpoon,
    Helm,
    IngredientCooking,
    Inscription,
    Insignia,
    Leggings,
    LongBow,
    Mace,
    Meal,
    Pistol,
    Potion,
    Refinement,
    RefinementEctoplasm,
    RefinementObsidian,
    Rifle,
    Ring,
    Scepter,
    Seasoning,
    Shield,
    ShortBow,
    Shoulders,
    Snack,
    Soup,
    Speargun,
    Staff,
    Sword,
    Torch,
    Trident,
    UpgradeComponent,
    Warhorn,
}

/// Flags for additional information about a recipe.
#[derive(Debug, Deserialize, PartialEq)]
pub enum RecipeFlags {
    /// Indicates that the recipe automatically unlocks upon reaching the required rating.
    AutoLearned,
    /// Indicates that an item must be unlocked from a recipe sheet.
    LearnedFromItem,
}

/// Object containing id and amount needed of an ingredient for a recipe.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Ingredient {
    /// id of the ingredient.
    #[serde(rename = "item_id")]
    id: String,
    /// Amount of ingredients required.
    count: String,
}

impl Recipe {
    /// Retrieve a recipe by its id.
    pub fn get_id(client: &Client, id: String) -> Result<Recipe, ApiError> {
        let url = format!("{}?recipe_id={}", ENDPOINT_URL, id);
        parse_response(&mut client.request(&url)?)
    }

    /// Returns the id of the recipe.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the type of the recipe.
    pub fn recipe_type(&self) -> &RecipeType {
        &self.recipe_type
    }

    /// Returns the id of the produced item.
    pub fn output_item_id(&self) -> &str {
        &self.output_item_id
    }

    /// Returns the amount of items produced.
    pub fn output_item_count(&self) -> &str {
        &self.output_item_count
    }

    /// Returns the minimum rating of the recipe.
    pub fn min_rating(&self) -> &str {
        &self.min_rating
    }

    /// Returns the time it takes to craft the item in ms.
    pub fn time_to_craft_ms(&self) -> &str {
        &self.time_to_craft_ms
    }

    /// Returns the potential value in coins received when selling the item to a vendor.
    pub fn vendor_value(&self) -> &str {
        &self.vendor_value
    }

    /// Returns the list of crafting disciplines that can use the recipe.
    pub fn disciplines(&self) -> &Vec<Discipline> {
        &self.disciplines
    }

    /// Returns additional flags for the recipe.
    pub fn flags(&self) -> &Vec<RecipeFlags> {
        &self.flags
    }

    /// Returns a list of objects describing the ingredients for this recipe.
    pub fn ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }
}

impl Ingredient {
    /// Returns the item id of the ingredient.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the amount of this specific ingredient that is required.
    pub fn count(&self) -> &str {
        &self.count
    }

}

#[cfg(test)]
mod tests {
    use crate::v1::recipe_details::*;
    use crate::client::Client;

    const JSON_RECIPE: &str = r#"
    {
      "recipe_id": "1275",
      "type": "Coat",
      "output_item_id": "11541",
      "output_item_count": "1",
      "min_rating": "25",
      "time_to_craft_ms": "1000",
      "disciplines": [ "Leatherworker" ],
      "flags": [],
      "ingredients": [
        { "item_id": "19797", "count": "1" },
        { "item_id": "13094", "count": "1" },
        { "item_id": "13093", "count": "1" }
      ]
    }"#;

    #[test]
    fn create_recipe() {
        match serde_json::from_str::<Recipe>(JSON_RECIPE) {
            Ok(_) => assert!(true),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn get_recipe() {
        let client = Client::new();
        let recipe = serde_json::from_str::<Recipe>(JSON_RECIPE).unwrap();
        assert_eq!(Recipe::get_id(&client, "1275".to_string()).unwrap(), recipe)
    }
}
