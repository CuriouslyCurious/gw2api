use serde::Deserialize;

use gw2api_core::utils::Discipline;

use gw2api_derive::ParamEndpoint;

/// Struct containing information about a requested recipe.
#[derive(ParamEndpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/recipe_details?recipe_id={}"]
pub struct Recipe {
    /// id of the recipe.
    #[serde(rename = "recipe_id")]
    pub id: String,
    /// Type of the recipe.
    #[serde(rename = "type")]
    pub recipe_type: RecipeType,
    /// The id of the produced item.
    pub output_item_id: String,
    /// The number of the produced.
    pub output_item_count: String,
    /// Minimum rating of the recipe.
    pub min_rating: String,
    /// Time it takes to craft the item in ms.
    pub time_to_craft_ms: String,
    /// Potential value in coins when selling to a vendor.
    #[serde(default)]
    pub vendor_value: String,
    /// Crafting disciplines that can use the recipe.
    #[serde(default)]
    pub disciplines: Vec<Discipline>,
    /// Additional recipe flags.
    #[serde(default)]
    pub flags: Vec<RecipeFlags>,
    /// List of objects describing the ingredients used for this recipe.
    #[serde(default)]
    pub ingredients: Vec<Ingredient>,
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
    pub id: String,
    /// Amount of ingredients required.
    pub count: String,
}

// impl Recipe {
//     /// Retrieve a recipe by its id.
//     pub fn get_id(client: &Client, id: String) -> Result<Recipe, ApiError> {
//         let url = format!("{}?recipe_id={}", ENDPOINT_URL, id);
//         client.request(&url)
//     }
// }

impl Ingredient {}

#[cfg(test)]
mod tests {
    use crate::v1::recipe_details::*;

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
        serde_json::from_str::<Recipe>(JSON_RECIPE).unwrap();
    }
}
