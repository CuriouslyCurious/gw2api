use crate::client::Client;
use crate::error::ApiError;
use crate::attributes::Attribute;
use crate::utils::{parse_response, Rarity};

const ENDPOINT_URL: &str = "/v1/item_details";

/// This endpoint is quite silly, since all numerical values are encoded as strings, meaning when
/// a number is the expected result it will be a string instead, with the exception of
/// infix_upgrade for some reason ¯\_(ツ)_/¯.

/// Struct containing detailed localized information about a requested item.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Item {
    /// id of the item.
    #[serde(rename = "item_id")]
    id: String,
    /// Name of the item.
    name: String,
    /// Description of the item.
    description: Option<String>,
    /// Type of the item.
    #[serde(rename = "type")]
    item_type: ItemType,
    /// Required level for the item.
    level: String,
    /// Rarity of the item.
    rarity: Rarity,
    /// Value in coins when selling to a vendor.
    vendor_value: String,
    /// File id to be used with the render service.
    icon_file_id: String,
    /// File signature to be used with the render service.
    icon_file_signature: String,
    /// Skin id to be used with the skin_details endpoint to get more information. Only present
    /// for item types: Armor, Back and Weapon.
    default_skin: Option<String>,
    // TODO: Find an example.
    /// List of upgrade recipes, whatever they are.
    upgrade_recipes: Vec<String>,
    /// Game types where the item is usable.
    game_types: Vec<GameType>,
    /// Additional item flags.
    flags: Vec<ItemFlag>,
    /// Item usage restrictions based on race or profession.
    restrictions: Vec<Restrictions>,
    /// If the item is a piece of armor, this will be Some().
    armor: Option<Armor>,
    /// If the item is a piece of back, this will be Some().
    back: Option<Back>,
    /// If the item is a bag, this will be Some().
    bag: Option<Bag>,
    /// If the item is a consumable, this will be Some().
    consumable: Option<Consumable>,
    /// If the item is a container, this will be Some().
    container: Option<Container>,
    /// If the item is a crafting material, this will be Some(), containing an empty object.
    crafting_material: Option<CraftingMaterial>,
    /// If the item is a gathering, this will be Some().
    gathering: Option<Gathering>,
    /// If the item is a gizmo, this will be Some().
    gizmo: Option<Gizmo>,
    /// If the item is a mini pet, this will be Some(), containing an empty object.
    mini_pet: Option<MiniPet>,
    /// If the item is a tool, this will be Some().
    tool: Option<Tool>,
    /// If the item is a trinket, this will be Some().
    trinket: Option<Trinket>,
    /// If the item is a trophy, this will be Some(), containing an empty object.
    trophy: Option<Trophy>,
    /// If the item is a upgrade component, this will be Some().
    upgrade_component: Option<UpgradeComponent>,
    /// If the item is a weapon, this will be Some().
    weapon: Option<Weapon>,
}

/// All item restrictions (races and professions).
#[derive(Debug, Deserialize, PartialEq)]
pub enum Restrictions {
    Revenant,
    Warrior,
    Guardian,
    Thief,
    Ranger,
    Engineer,
    Necromancer,
    Mesmer,
    Elementalist,
    Human,
    Norn,
    Sylvari,
    Asura,
    Charr,
    //Tengu when? :(
}

/// Possible item types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ItemType {
    Armor,
    Back,
    Bag,
    Consumable,
    Container,
    CraftingMaterial,
    Gathering,
    Gizmo,
    MiniPet,
    Tool,
    Trait,
    Trinket,
    Trophy,
    UpgradeComponent,
    Weapon,
}

/// Possible additional item flags.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ItemFlag {
    AccountBindOnUse,
    AccountBound,
    BulkConsume,
    DeleteWarning,
    HideSuffix,
    MonsterOnly,
    NoMysticForge,
    NoSalvage,
    NoSell,
    NotUpgradeable,
    NoUnderwater,
    SoulbindOnAcquire,
    SoulBindOnUse, // Anet pls
    Tonic,
    Unique,
}

/// All game types currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum GameType {
    Activity,
    Dungeon,
    Pve,
    Pvp,
    PvpLobby,
    Wvw,
}

/// All armor types currently in the game.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ArmorType {
    Boots,
    Coat,
    Gloves,
    Helm,
    HelmAquatic,
    Leggings,
    Shoulders,
}

/// All armor weight class.
#[derive(Debug, Deserialize, PartialEq)]
pub enum WeightClass {
    Clothing,
    Heavy,
    Light,
    Medium,
}

/// Struct containing information about a piece of armor.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Armor {
    /// Armor piece type.
    #[serde(rename = "type")]
    armor_type: ArmorType,
    /// The armor's weight class.
    weight_class: WeightClass,
    /// Defense value.
    defense: String,
    /// Number and type of infusion slots.
    infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    secondary_suffix_item_id: Option<String>,
    /// List of ids of stat choices that can be made with the item.
    #[serde(default)]
    stat_choices: Vec<u32>,
}

/// All infusion types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum InfusionType {
    Infusion,
    Defense,
    Offense,
    Utility,
}

/// Struct containing information about an infusion slot.
#[derive(Debug, Deserialize, PartialEq)]
pub struct InfusionSlot {
    /// id of the infusion slot, only used by Back, the only known value is 49428 (+5
    /// Agony_Infusion).
    #[serde(rename = "item_id")]
    id: Option<String>,
    /// Types of infusions allowed in this slot.
    flags: Vec<InfusionType>,
}

/// Struct containing information about an upgrade for a bonus given by a piece armor.
#[derive(Debug, Deserialize, PartialEq)]
pub struct InfixUpgrade {
    /// id of the infix upgrade.
    id: u32,
    /// Buff applied by the item. Seemingly only exists a Boon Duration buff.
    buff: Option<Buff>,
    /// Map of stat bonuses given by the item.
    attributes: Vec<Stat>,
}

/// Struct containing information about a stat on an item.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Stat {
    /// Attribute type.
    attribute: Attribute,
    /// How much the attribute is modified by.
    modifier: u32,
}

/// Struct containing information about a buff applied by an item. The buff seems to only be bonus
/// Boon Duration.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Buff {
    /// id of the skill applied by the item. Only known values are: "16517" and "10521".
    skill_id: String,
    /// Description of the effect of the skill. Only known description is "+1% Boon Duration".
    description: String,
}

/// Struct containing information about a back piece.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Back {
    /// List of infusion slot types.
    infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    secondary_suffix_item_id: Option<String>,
    /// List of ids of stat choices that can be made with the item.
    #[serde(default)]
    stat_choices: Vec<u32>,
}

/// Struct containing information about a bag.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Bag {
    /// Whether or not the bag is a special one.
    no_sell_or_sort: Option<String>,
    /// Size of the bag.
    size: Option<String>,
}

/// All unlock types for consumables.
#[derive(Debug, Deserialize, PartialEq)]
pub enum UnlockType {
    BagSlot,
    BankTab,
    CollectibleCapacity,
    Content,
    CraftingRecipe,
    Dye,
    Unknown,
}

/// All consumable types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ConsumableType {
    AppearanceChange,
    Booze,
    ContractNpc,
    Food,
    Generic,
    Halloween,
    Immediate,
    Transmutation,
    Unlock,
    UnTransformation,
    UpgradeRemoval,
    Utility,
}

/// Struct containing information about a consumable.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Consumable {
    /// Consumable type
    #[serde(rename = "type")]
    consumable_type: ConsumableType,
    /// Effect duration in milliseconds (may exist if consumable type is "Generic", "Food" or
    /// "Utility").
    duration_ms: Option<String>,
    /// Description of the effect (may exist if consumable type is "Generic", "Food" or
    /// "Utility").
    description: Option<String>,
    /// The type of unlock (if consumable type is "Unlock").
    unlock_type: Option<UnlockType>,
    /// id of the recipe unlocked by the consumable (if unlock type is "CraftingRecipe").
    recipe_id: Option<String>,
    /// id of the dye unlocked by the consumable (if unlock type is "Dye").
    color_id: Option<String>,
}

/// Possible container types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ContainerType {
    Default,
    GiftBox,
    OpenUI,
}

/// Struct containing information about a container.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Container {
    /// Container type.
    #[serde(rename = "type")]
    container_type: ContainerType,
}

/// Struct describing a CraftingMaterial, will always be empty.
#[derive(Debug, Deserialize, PartialEq)]
pub struct CraftingMaterial {}

/// Possible gathering types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum GatheringType {
    Foraging,
    Logging,
    Mining,
}

/// Struct containing information about a gathering object.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Gathering {
    /// Gathering type.
    #[serde(rename = "type")]
    gathering_type: GatheringType,
}

/// Possible gizmo types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum GizmoType {
    Default,
    ContainerKey,
    RentableContractNpc,
    UnlimitedConsumable,
}

/// Struct containing information about a gizmo.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Gizmo {
    /// Gizmo type.
    #[serde(rename = "type")]
    gizmo_type: GizmoType,
}

/// Struct describing a MiniPet, will always be empty.
#[derive(Debug, Deserialize, PartialEq)]
pub struct MiniPet {}

/// Possible tool types, only known one is Salvage.
#[derive(Debug, Deserialize, PartialEq)]
pub enum ToolType {
    Salvage,
}

/// Struct containing information about a tool.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Tool {
    /// Tool type.
    #[serde(rename = "type")]
    tool_type: ToolType,
    /// Number of charges of the tool.
    charges: i32,
}

/// Possible trinket types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum TrinketType {
    Amulet,
    Accessory,
    Ring,
}

/// Struct containing information about a trinket.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Trinket {
    /// Trinket type.
    #[serde(rename = "type")]
    trinket_type: TrinketType,
    /// List of infusion slot types.
    infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    secondary_suffix_item_id: Option<String>,
    /// List of ids of stat choices that can be made with the item.
    #[serde(default)]
    stat_choices: Vec<u32>,
}

/// Struct describing a trophy, will always be empty.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Trophy {}

/// Possible upgrade component types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum UpgradeType {
    /// Jewels, infusions, and PvP sigils & runes.
    Default,
    /// Universal upgrades and gemstones.
    Gem,
    Rune,
    Sigil,
}

/// All upgrade flags for upgrade components.
#[derive(Debug, Deserialize, PartialEq)]
pub enum UpgradeFlag {
    HeavyArmor,
    LightArmor,
    MediumArmor,
    Axe,
    Dagger,
    Focus,
    Greatsword,
    Hammer,
    Harpoon,
    LongBow,
    Mace,
    Pistol,
    Rifle,
    Scepter,
    Shield,
    ShortBow,
    Speargun,
    Staff,
    Sword,
    Torch,
    Trident,
    Warhorn,
    Trinket,
}

/// Struct containing information about an upgrade component.
#[derive(Debug, Deserialize, PartialEq)]
pub struct UpgradeComponent {
    /// Upgrade component type.
    #[serde(rename = "type")]
    upgrade_type: UpgradeType,
    /// List of items which the upgrade component can be applied to.
    flags: Vec<UpgradeFlag>,
    /// List of infusion slots the upgrade component can be applied to.
    infusion_upgrade_flags: Vec<InfusionType>,
    /// List of rune bonuses, only exits for the upgrade type "Rune">
    bonuses: Option<UpgradeType>,
    /// Potential bonus given by the object. Optional property.
    infix_upgrade: Option<InfixUpgrade>,
    /// Suffix added to the item when it is applied.
    suffix: String,
}

/// Possible damage types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum DamageType {
    Choking,
    Fire,
    Ice,
    Lightning,
    Physical,
}

/// All weapon types.
#[derive(Debug, Deserialize, PartialEq)]
pub enum WeaponType {
    Axe,
    Dagger,
    Focus,
    Greatsword,
    Hammer,
    Harpoon,
    LargeBundle,
    LongBow,
    Mace,
    Pistol,
    Rifle,
    Scepter,
    Shield,
    ShortBow,
    SmallBundle,
    Speargun,
    Staff,
    Sword,
    Torch,
    Toy,
    Trident,
    TwoHandedToy,
    Warhorn,
}

/// Struct containing information about a weapon.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Weapon {
    /// Weapon type.
    #[serde(rename = "type")]
    weapon_type: WeaponType,
    /// Damage type.
    damage_type: DamageType,
    /// Weapon's minimal power rating.
    min_power: String,
    /// Weapon's maximum power rating.
    max_power: String,
    /// The weapon's defense rating.
    defense: String,
    /// List of infusion slots the upgrade component can be applied to.
    infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    secondary_suffix_item_id: Option<String>,
}


impl Item {
    /// Retrieve an item by its id.
    pub fn get_id(client: &Client, id: u32) -> Result<Item, ApiError> {
        let url = format!("{}?item_id={}", ENDPOINT_URL, id);
        parse_response(&mut client.request(&url)?)
    }

    /// Returns the id of the item.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the item.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the item.
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns the type of the item.
    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }

    /// Returns the required level for the item.
    pub fn level(&self) -> &str {
        &self.level
    }

    /// Returns the rarity of the item.
    pub fn rarity(&self) -> &Rarity {
        &self.rarity
    }

    /// Returns the value in coins when selling to a vendor.
    pub fn vendor_value(&self) -> &str {
        &self.vendor_value
    }

    /// Returns the file id to be used with the render service.
    pub fn icon_file_id(&self) -> &str {
        &self.icon_file_id
    }

    /// Returns the file signature to be used with the render service.
    pub fn icon_file_signature(&self) -> &str {
        &self.icon_file_signature
    }

    /// Returns the skin id to be used with the skin_details endpoint to get more information.
    /// Only present for item types: Armor, Back and Weapon.
    pub fn default_skin(&self) -> Option<&String> {
        self.default_skin.as_ref()
    }

    // TODO: Find an example.
    /// Returns the list of upgrade recipes, whatever they are.
    pub fn upgrade_recipes(&self) -> &Vec<String> {
        &self.upgrade_recipes
    }

    /// Returns the game types where the item is usable.
    pub fn game_types(&self) -> &Vec<GameType> {
        &self.game_types
    }

    /// Returns the additional item flags.
    pub fn flags(&self) -> &Vec<ItemFlag> {
        &self.flags
    }

    /// Returns the item usage restrictions based on race or profession.
    pub fn restrictions(&self) -> &Vec<Restrictions> {
        &self.restrictions
    }

    /// Returns Some(&Armor) if the item is a piece of armor, None otherwise.
    pub fn armor(&self) -> Option<&Armor> {
        self.armor.as_ref()
    }

    /// Returns Some(&Back) if the item is a back piece, None otherwise.
    pub fn back(&self) -> Option<&Back> {
        self.back.as_ref()
    }

    /// Returns Some(&Bag) if the item is a bag, None otherwise.
    pub fn bag(&self) -> Option<&Bag> {
        self.bag.as_ref()
    }

    /// Returns Some(&Consumable) if the item is a consumable, None otherwise.
    pub fn consumable(&self) -> Option<&Consumable> {
        self.consumable.as_ref()
    }

    /// Returns Some(&Container) if the item is a container, None otherwise.
    pub fn container(&self) -> Option<&Container> {
        self.container.as_ref()
    }

    /// Returns Some(&CraftingMaterial) if the item is a crafting material, None otherwise.
    pub fn crafting_material(&self) -> Option<&CraftingMaterial> {
        self.crafting_material.as_ref()
    }

    /// Returns Some(&Gathering) if the item is a gathering tool, None otherwise.
    pub fn gathering(&self) -> Option<&Gathering> {
        self.gathering.as_ref()
    }

    /// Returns Some(&Gizmo) if the item is a gizmo, None otherwise.
    pub fn gizmo(&self) -> Option<&Gizmo> {
        self.gizmo.as_ref()
    }

    /// Returns Some(&MiniPet) if the item is a mini pet, None otherwise.
    pub fn mini_pet(&self) -> Option<&MiniPet> {
        self.mini_pet.as_ref()
    }

    /// Returns Some(&Tool) if the item is a tool, None otherwise.
    pub fn tool(&self) -> Option<&Tool> {
        self.tool.as_ref()
    }

    /// Returns Some(&Trinket) if the item is a trinket, None otherwise.
    pub fn trinket(&self) -> Option<&Trinket> {
        self.trinket.as_ref()
    }

    /// Returns Some(&Trophy) if the item is a trophy, None otherwise.
    pub fn trophy(&self) -> Option<&Trophy> {
        self.trophy.as_ref()
    }

    /// Returns Some(&UpgradeComponent) if the item is an upgrade component, None otherwise.
    pub fn upgrade_component(&self) -> Option<&UpgradeComponent> {
        self.upgrade_component.as_ref()
    }

    /// Returns Some(&Weapon) if the item is a weapon, None otherwise.
    pub fn weapon(&self) -> Option<&Weapon> {
        self.weapon.as_ref()
    }
}

impl Armor {
    /// Returns the armor piece type.
    pub fn armor_type(&self) -> &ArmorType {
        &self.armor_type
    }

    /// Returns the the armor's weight class.
    pub fn weight_class(&self) -> &WeightClass {
        &self.weight_class
    }

    /// Returns the defense value.
    pub fn defense(&self) -> &String {
        &self.defense
    }

    /// Returns the number and type of infusion slots.
    pub fn infusion_slots(&self) -> &Vec<InfusionSlot> {
        &self.infusion_slots
    }

    /// Returns the potential bonus given by the object. Optional property.
    pub fn infix_upgrade(&self) -> Option<&InfixUpgrade> {
        self.infix_upgrade.as_ref()
    }

    /// Returns the item id of an already applied upgrade component. Can be empty.
    pub fn suffix_item_id(&self) -> Option<&String> {
        self.suffix_item_id.as_ref()
    }

    /// Returns the potential secondary upgrade component. Always empty.
    pub fn secondary_suffix_item_id(&self) -> Option<&String> {
        self.secondary_suffix_item_id.as_ref()
    }

    /// Returns the list of ids of stat choices that can be made with the item.
    pub fn stat_choices(&self) -> &Vec<u32> {
        &self.stat_choices
    }
}

impl InfusionSlot {
    /// Returns the Optional id of the infusion slot, only used by Back, the only known value is 49428
    /// (+5 Agony_Infusion).
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Returns the types of infusions allowed in this slot.
    pub fn flags(&self) -> &Vec<InfusionType> {
        &self.flags
    }
}

impl InfixUpgrade {
    /// Returns the id of the infix upgrade.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the buff applied by the item. Seemingly only exists a Boon Duration buff.
    pub fn buff(&self) -> Option<&Buff> {
        self.buff.as_ref()
    }

    /// Returns the map of stat bonuses given by the item.
    pub fn attributes(&self) -> &Vec<Stat> {
        &self.attributes
    }
}

impl Stat {
    /// Returns the attribute type.
    pub fn attribute(&self) -> &Attribute {
        &self.attribute
    }

    /// Returns how much the attribute is modified by.
    pub fn modifier(&self) -> u32 {
        self.modifier
    }
}

impl Buff {
    /// Returns the id of the skill applied by the item. Only known values are: "16517" and "10521".
    pub fn skill_id(&self) -> &str {
        &self.skill_id
    }

    /// Returns the description of the effect of the skill. Only known description is "+1% Boon Duration".
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Back {
    /// Returns the list of infusion slot types.
    pub fn infusion_slots(&self) -> &Vec<InfusionSlot> {
        &self.infusion_slots
    }

    /// Returns the potential bonus given by the object. Optional property.
    pub fn infix_upgrade(&self) -> Option<&InfixUpgrade> {
        self.infix_upgrade.as_ref()
    }

    /// Returns the item id of an already applied upgrade component. Can be empty.
    pub fn suffix_item_id(&self) -> Option<&String> {
        self.suffix_item_id.as_ref()
    }

    /// Returns the potential secondary upgrade component. Always empty.
    pub fn secondary_suffix_item_id(&self) -> Option<&String> {
        self.secondary_suffix_item_id.as_ref()
    }

    /// Returns the list of ids of stat choices that can be made with the item.
    pub fn stat_choices(&self) -> &Vec<u32> {
        &self.stat_choices
    }
}

impl Bag {
    /// Returns whether or not the bag is a special one.
    pub fn no_sell_or_sort(&self) -> Option<&String> {
        self.no_sell_or_sort.as_ref()
    }

    /// Returns the size of the bag.
    pub fn size(&self) -> Option<&String> {
        self.size.as_ref()
    }
}

impl Consumable {
    /// Returns the consumable type
    pub fn consumable_type(&self) -> &ConsumableType {
        &self.consumable_type
    }

    /// Returns the potential effect duration in milliseconds (may exist if consumable type
    /// is "Generic", "Food" or "Utility").
    pub fn duration_ms(&self) -> Option<&String> {
        self.duration_ms.as_ref()
    }

    /// Returns the description of the effect (may exist if consumable type is "Generic", "Food" or
    /// "Utility").
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns the type of unlock (if consumable type is "Unlock").
    pub fn unlock_type(&self) -> Option<&UnlockType> {
        self.unlock_type.as_ref()
    }

    /// Returns the id of the recipe unlocked by the consumable (if unlock type is "CraftingRecipe").
    pub fn recipe_id(&self) -> Option<&String> {
        self.recipe_id.as_ref()
    }

    /// Returns the id of the dye unlocked by the consumable (if unlock type is "Dye").
    pub fn color_id(&self) -> Option<&String> {
        self.color_id.as_ref()
    }
}

impl Container {
    /// Returns the container type.
    pub fn container_type(&self) -> &ContainerType {
        &self.container_type
    }
}

impl CraftingMaterial {}

impl Gathering {
    /// Returns the gathering type.
    pub fn gathering_type(&self) -> &GatheringType {
        &self.gathering_type
    }
}

impl Gizmo {
    /// Returns the gizmo type.
    pub fn gizmo_type(&self) -> &GizmoType {
        &self.gizmo_type
    }
}

impl MiniPet {}

impl Tool {
    /// Returns the tool type.
    pub fn tool_type(&self) -> &ToolType {
        &self.tool_type
    }

    /// Returns the number of charges of the tool.
    pub fn charges(&self) -> i32 {
        self.charges
    }
}

impl Trinket {
    /// Returns the trinket type.
    pub fn trinket_type(&self) -> &TrinketType {
        &self.trinket_type
    }

    /// Returns the list of infusion slot types.
    pub fn infusion_slots(&self) -> &Vec<InfusionSlot> {
        &self.infusion_slots
    }

    /// Returns the potential bonus given by the object. Optional property.
    pub fn infix_upgrade(&self) -> Option<&InfixUpgrade> {
        self.infix_upgrade.as_ref()
    }

    /// Returns the item id of an already applied upgrade component. Can be empty.
    pub fn suffix_item_id(&self) -> Option<&String> {
        self.suffix_item_id.as_ref()
    }

    /// Returns the potential secondary upgrade component. Always empty.
    pub fn secondary_suffix_item_id(&self) -> Option<&String> {
        self.secondary_suffix_item_id.as_ref()
    }

    /// Returns the list of ids of stat choices that can be made with the item.
    pub fn stat_choices(&self) -> &Vec<u32> {
        &self.stat_choices
    }
}

impl Trophy {}

impl UpgradeComponent {
    /// Returns the upgrade component type.
    pub fn upgrade_type(&self) -> &UpgradeType {
        &self.upgrade_type
    }

    /// Returns the list of items which the upgrade component can be applied to.
    pub fn flags(&self) -> &Vec<UpgradeFlag> {
        &self.flags
    }

    /// Returns the list of infusion slots the upgrade component can be applied to.
    pub fn infusion_upgrade_flags(&self) -> &Vec<InfusionType> {
        &self.infusion_upgrade_flags
    }

    /// Returns the list of rune bonuses, only exits for the upgrade type "Rune"
    pub fn bonuses(&self) -> Option<&UpgradeType> {
        self.bonuses.as_ref()
    }

    /// Returns the potential bonus given by the object. Optional property.
    pub fn infix_upgrade(&self) -> Option<&InfixUpgrade> {
        self.infix_upgrade.as_ref()
    }

    /// Returns the suffix added to the item when it is applied.
    pub fn suffix(&self) -> &str {
        &self.suffix
    }
}

impl Weapon {
    /// Returns the weapon type.
    pub fn weapon_type(&self) -> &WeaponType {
        &self.weapon_type
    }

    /// Returns the damage type.
    pub fn damage_type(&self) -> &DamageType {
        &self.damage_type
    }

    /// Returns the weapon's minimal power rating.
    pub fn min_power(&self) -> &str {
        &self.min_power
    }

    /// Returns the weapon's maximum power rating.
    pub fn max_power(&self) -> &str {
        &self.max_power
    }

    /// Returns the the weapon's defense rating.
    pub fn defense(&self) -> &str {
        &self.defense
    }

    /// Returns the list of infusion slots the upgrade component can be applied to.
    pub fn infusion_slots(&self) -> &Vec<InfusionSlot> {
        &self.infusion_slots
    }

    /// Returns the potential bonus given by the object. Optional property.
    pub fn infix_upgrade(&self) -> Option<&InfixUpgrade> {
        self.infix_upgrade.as_ref()
    }

    /// Returns the item id of an already applied upgrade component. Can be empty.
    pub fn suffix_item_id(&self) -> Option<&String> {
        self.suffix_item_id.as_ref()
    }

    /// Returns the potential secondary upgrade component. Always empty.
    pub fn secondary_suffix_item_id(&self) -> Option<&String> {
        self.secondary_suffix_item_id.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::item_details::*;
    use crate::client::Client;

    const JSON_ARMOR: &str = r#"
    {
      "item_id": "100",
      "name": "Rampager's Seer Coat of Divinity",
      "description": "",
      "type": "Armor",
      "level": "72",
      "rarity": "Exotic",
      "vendor_value": "360",
      "icon_file_id": "61011",
      "icon_file_signature": "FB0AA64F98303AE5112408EF3DC8C7307EA118F8",
      "default_skin": "9",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "SoulBindOnUse"
      ],
      "restrictions": [],
      "armor": {
        "type": "Coat",
        "weight_class": "Light",
        "defense": "271",
        "infusion_slots": [],
        "infix_upgrade": {
          "id": 159,
          "attributes": [
            {
              "attribute": "Power",
              "modifier": 83
            },
            {
              "attribute": "Precision",
              "modifier": 116
            },
            {
              "attribute": "ConditionDamage",
              "modifier": 83
            }
          ]
        },
        "suffix_item_id": "24732",
        "secondary_suffix_item_id": ""
      }
    }"#;

    #[test]
    fn get_armor() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_ARMOR).unwrap();
        assert_eq!(Item::get_id(&client, 100).unwrap(), item)
    }

    const JSON_BACK: &str = r#"
    {
      "item_id": "77474",
      "name": "The Ascension",
      "type": "Back",
      "level": "80",
      "rarity": "Legendary",
      "vendor_value": "100000",
      "icon_file_id": "1313065",
      "icon_file_signature": "C72336A9140CC9F2B34CCFCDCE2B217B380A1ADC",
      "default_skin": "6561",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "HideSuffix",
        "AccountBound",
        "NoSalvage",
        "NoSell",
        "NotUpgradeable",
        "Unique",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "back": {
        "infusion_slots": [
          {
            "flags": [
              "Infusion"
            ]
          },
          {
            "flags": [
              "Infusion"
            ]
          }
        ],
        "suffix_item_id": "",
        "secondary_suffix_item_id": "",
        "stat_choices": [
          584,
          656,
          658,
          1119,
          657,
          1038,
          1097,
          659,
          690,
          583,
          585,
          1037,
          586,
          1035,
          588,
          1114,
          1128,
          1163,
          1066,
          1064,
          660,
          1430,
          1436,
          591,
          581,
          592,
          1145,
          1098,
          1134,
          1162,
          1115,
          1130,
          1139,
          1125,
          1220,
          1329,
          1345,
          1337,
          1486,
          1538
        ]
      }
    }"#;

    #[test]
    fn get_back() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_BACK).unwrap();
        assert_eq!(Item::get_id(&client, 77474).unwrap(), item)
    }

    const JSON_BAG: &str = r#"
    {
      "item_id": "9480",
      "name": "8 Slot Invisible Bag",
      "description": "8 Slots. Items in this bag will never appear in a sell-to-vendor list and will not move when inventory is sorted.",
      "type": "Bag",
      "level": "0",
      "rarity": "Fine",
      "vendor_value": "22",
      "icon_file_id": "219376",
      "icon_file_signature": "647CF53A7C4903C2E09091CBDC4F215834C50A4B",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [],
      "restrictions": [],
      "bag": {
        "no_sell_or_sort": "1",
        "size": "8"
      }
    }"#;

    #[test]
    fn get_bag() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_BAG).unwrap();
        assert_eq!(Item::get_id(&client, 9480).unwrap(), item)
    }

    const JSON_CONSUMABLE_RECIPE: &str = r#"
    {
      "item_id": "10000",
      "name": "Recipe: Satchel of Rejuvenating Masquerade Armor (Rare)",
      "description": "Double-click to learn the Tailor recipe for Satchel of Rejuvenating Masquerade Armor (Rare)",
      "type": "Consumable",
      "level": "0",
      "rarity": "Rare",
      "vendor_value": "0",
      "icon_file_id": "849411",
      "icon_file_signature": "0D9835CC3F70CC70EF0F2655C764C728580E6CEE",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSalvage"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Unlock",
        "unlock_type": "CraftingRecipe",
        "recipe_id": "2756"
      }
    }"#;

    #[test]
    fn get_consumable_recipe() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_RECIPE).unwrap();
        assert_eq!(Item::get_id(&client, 10000).unwrap(), item)
    }

    const JSON_CONSUMABLE_DYE: &str = r#"
    {
      "item_id": "20356",
      "name": "Abyss Dye",
      "description": "Double-click to unlock this dye color for all characters.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Rare",
      "vendor_value": "50",
      "icon_file_id": "66649",
      "icon_file_signature": "DFFC622FB06E0511A49C53961703331CE030E7BC",
      "upgrade_recipes": [],
      "game_types": [
        "Pvp",
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSell"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Unlock",
        "unlock_type": "Dye",
        "color_id": "473"
      }
    }"#;

    #[test]
    fn get_consumable_dye() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_DYE).unwrap();
        assert_eq!(Item::get_id(&client, 20356).unwrap(), item)
    }

    const JSON_CONSUMABLE_OTHERS: &str = r#"
    {
      "item_id": "19988",
      "name": "Limited-Use Cow Finisher",
      "description": "Crush your opponents with this falling cow finishing move in PvP and WvW.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Basic",
      "vendor_value": "0",
      "icon_file_id": "340513",
      "icon_file_signature": "BD32DB030170FA01A03E77BB4313DEC2D8BC20E8",
      "upgrade_recipes": [],
      "game_types": [
        "Pvp",
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoMysticForge",
        "NoSalvage",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Unlock",
        "unlock_type": "Content"
      }
    }"#;

    #[test]
    fn get_consumable_others() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_OTHERS).unwrap();
        assert_eq!(Item::get_id(&client, 19988).unwrap(), item)
    }

    const JSON_CONSUMABLE_GENERIC_WITH_EFFECT: &str = r#"
    {
      "item_id": "50018",
      "name": "Maintenance Oil Station",
      "description": "Utility Station: Double-click to set out a model of Gate Hub Plaza which allows players in the area to apply maintenance oil to their weapons. Station stays active for five minutes.",
      "type": "Consumable",
      "level": "80",
      "rarity": "Masterwork",
      "vendor_value": "66",
      "icon_file_id": "740204",
      "icon_file_signature": "43C10FB3C60AFBC9CFD50413170A4D553A0046D8",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSell"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Generic",
        "duration_ms": "5400000",
        "apply_count": 1,
        "name": "Enhancement",
        "icon": {
          "file_id": 436368,
          "signature": "64976F59BF060718C9109D36C27AD0F40F06BB32"
        },
        "description": "Gain Concentration Equal to 3% of Your Precision\nGain Concentration Equal to 6% of Your Healing Power\n+10% Experience from Kills"
      }
    }"#;

    #[test]
    fn get_consumable_generic_with_effect() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_GENERIC_WITH_EFFECT).unwrap();
        assert_eq!(Item::get_id(&client, 50018).unwrap(), item)
    }

    const JSON_CONSUMABLE_GENERIC_WITHOUT_EFFECT: &str = r#"
    {
      "item_id": "24",
      "name": "Sealed Package of Snowballs",
      "description": "Open this package to create several snowballs that can hit anyone else holding a snowball.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Basic",
      "vendor_value": "0",
      "icon_file_id": "219347",
      "icon_file_signature": "1D05D1EE04E16E69710E1EAB11AC466BBF105778",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [],
      "restrictions": [],
      "consumable": {
        "type": "Generic"
      }
    }"#;

    #[test]
    fn get_consumable_generic_without_effect() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_GENERIC_WITHOUT_EFFECT).unwrap();
        assert_eq!(Item::get_id(&client, 24).unwrap(), item)
    }

    const JSON_UTILITY: &str = r#"
    {
      "item_id": "50082",
      "name": "Powerful Potion of Slaying Scarlet's Armies",
      "type": "Consumable",
      "level": "80",
      "rarity": "Fine",
      "vendor_value": "100",
      "icon_file_id": "222720",
      "icon_file_signature": "7A9EA9E0C702F8BA595973FED3A853646B2E07F8",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSell"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Utility",
        "duration_ms": "3600000",
        "apply_count": 1,
        "name": "Enhancement",
        "icon": {
          "file_id": 436368,
          "signature": "64976F59BF060718C9109D36C27AD0F40F06BB32"
        },
        "description": "+10% damage vs. Toxic Alliance, Watchwork, Aetherblade, and Molten Alliance\n-10% damage from Toxic Alliance, Watchwork, Aetherblade, and Molten Alliance\n+15% Experience from kills"
      }
    }"#;

    #[test]
    fn get_utility() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UTILITY).unwrap();
        assert_eq!(Item::get_id(&client, 50082).unwrap(), item)
    }

    const JSON_CONSUMABLE_BOOZE: &str = r#"
    {
      "item_id": "8553",
      "name": "Bottle of Red Wine",
      "type": "Consumable",
      "level": "0",
      "rarity": "Basic",
      "vendor_value": "4",
      "icon_file_id": "63083",
      "icon_file_signature": "E745C51C0D067B030A6629C80A9B08A4B0146556",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "BulkConsume"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Booze"
      }
    }"#;

    #[test]
    fn get_consumable_booze() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_BOOZE).unwrap();
        assert_eq!(Item::get_id(&client, 8553).unwrap(), item)
    }

    const JSON_CONSUMABLE_TRANSMUTATION: &str = r#"
    {
      "item_id": "8471",
      "name": "Transmutation Crystal",
      "description": "Double-click to consume 1 transmutation crystal for 1 transmutation charge.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Masterwork",
      "vendor_value": "64",
      "icon_file_id": "63056",
      "icon_file_signature": "B50BF7D22026382EEE702714761DB87901EF710E",
      "upgrade_recipes": [],
      "game_types": [
        "Pvp",
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoSalvage",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Generic"
      }
    }"#;

    #[test]
    fn get_consumable_transmutation() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_TRANSMUTATION).unwrap();
        assert_eq!(Item::get_id(&client, 8471).unwrap(), item)
    }

    const JSON_CONSUMABLE_UNTRANSMUTATION: &str = r#"
    {
      "item_id": "8472",
      "name": "Transmutation Splitter",
      "description": "Double-click to consume for transmutation charges.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Masterwork",
      "vendor_value": "0",
      "icon_file_id": "631525",
      "icon_file_signature": "F3E0F168BCC054462BF068643F5992525C7D52EE",
      "upgrade_recipes": [],
      "game_types": [
        "Pvp",
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoMysticForge",
        "NoSalvage",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Generic"
      }
    }"#;

    #[test]
    fn get_consumable_untransmutation() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_UNTRANSMUTATION).unwrap();
        assert_eq!(Item::get_id(&client, 8472).unwrap(), item)
    }

    const JSON_CONSUMABLE_IMMEDIATE: &str = r#"
    {
      "item_id": "8473",
      "name": "Supply Package",
      "description": "Supply can be used to build various siege engines and repair structures in WvW.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Fine",
      "vendor_value": "0",
      "icon_file_id": "62264",
      "icon_file_signature": "A64CDE12565E07F00470D710377A5EAB5E953816",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "SoulbindOnAcquire",
        "SoulBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Immediate"
      }
    }"#;

    #[test]
    fn get_consumable_immediate() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_IMMEDIATE).unwrap();
        assert_eq!(Item::get_id(&client, 8473).unwrap(), item)
    }


    const JSON_CONSUMABLE_FOOD: &str = r#"
    {
      "item_id": "12549",
      "name": "Chili Pepper Popper",
      "type": "Consumable",
      "level": "10",
      "rarity": "Fine",
      "vendor_value": "10",
      "icon_file_id": "219493",
      "icon_file_signature": "54B71F6FBFFF193168C0B2B86C436EA3AE09AB7E",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSell"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Food",
        "duration_ms": "1800000",
        "apply_count": 1,
        "name": "Nourishment",
        "icon": {
          "file_id": 436367,
          "signature": "779D3F0ABE5B46C09CFC57374DA8CC3A495F291C"
        },
        "description": "8% Chance to Gain Might on Critical Hit during the Day\n+10% Experience from Kills"
      }
    }"#;

    #[test]
    fn get_consumable_food() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONSUMABLE_FOOD).unwrap();
        assert_eq!(Item::get_id(&client, 12549).unwrap(), item)
    }

    const JSON_UTILITY_WITHOUT_EFFECT: &str = r#"
    {
      "item_id": "8640",
      "name": "Grawl Snowman Potion",
      "description": "You become a Grawl Snowman for 5 min.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Masterwork",
      "vendor_value": "22",
      "icon_file_id": "66150",
      "icon_file_signature": "1275404EE30C0FDCC1EE6AB3A70136FB020158D7",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "Tonic"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Utility"
      }
    }"#;

    #[test]
    fn get_utility_without_effect() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UTILITY_WITHOUT_EFFECT).unwrap();
        assert_eq!(Item::get_id(&client, 8640).unwrap(), item)
    }

    const JSON_UTILITY_HALLOWEEN: &str = r#"
    {
      "item_id": "20010",
      "name": "Rejuvenation Booster",
      "description": "Double-click to gain a boon that regenerates health for one hour. Does not work in PvP, WvW, or raids.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Masterwork",
      "vendor_value": "0",
      "icon_file_id": "434412",
      "icon_file_signature": "0439C04FE14D6AEC802A47139F78DD6729EC63A0",
      "upgrade_recipes": [],
      "game_types": [
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoSalvage",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "Halloween"
      }
    }"#;

    #[test]
    fn get_utility_halloween() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UTILITY_HALLOWEEN).unwrap();
        assert_eq!(Item::get_id(&client, 20010).unwrap(), item)
    }

    const JSON_UTILITY_CONTRACTNPC: &str = r#"
    {
      "item_id": "20017",
      "name": "Trading Post Express",
      "description": "Double-click to summon a Black Lion Delivery Agent for 15 minutes, allowing you and your allies to pick up trading post purchases and profits. Single-use.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Fine",
      "vendor_value": "4",
      "icon_file_id": "220569",
      "icon_file_signature": "550278CE4303EB99084810B33F6A9ECA1FEBE693",
      "upgrade_recipes": [],
      "game_types": [
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoMysticForge",
        "NoSalvage",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "ContractNpc"
      }
    }"#;

    #[test]
    fn get_utility_contractnpc() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UTILITY_CONTRACTNPC).unwrap();
        assert_eq!(Item::get_id(&client, 20017).unwrap(), item)
    }

    const JSON_UTILITY_UPGRADEREMOVAL: &str = r#"
    {
      "item_id": "20349",
      "name": "Upgrade Extractor",
      "description": "This tool will remove the upgrades from any item, without destroying either the upgrade or the item.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Fine",
      "vendor_value": "0",
      "icon_file_id": "674829",
      "icon_file_signature": "0E92A458B335144232EB357A3C96DB273B09AD3B",
      "upgrade_recipes": [],
      "game_types": [
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoMysticForge",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "UpgradeRemoval"
      }
    }"#;

    #[test]
    fn get_utility_upgraderemoval() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UTILITY_UPGRADEREMOVAL).unwrap();
        assert_eq!(Item::get_id(&client, 20349).unwrap(), item)
    }

    const JSON_UTILITY_APPEARANCECHANGE: &str = r#"
    {
      "item_id": "36284",
      "name": "Self-Style Hair Kit",
      "description": "Double-click to modify your appearance settings for hair style, hair color, horns, and facial hair. This kit is consumed on accepting a new appearance.",
      "type": "Consumable",
      "level": "0",
      "rarity": "Masterwork",
      "vendor_value": "8",
      "icon_file_id": "495499",
      "icon_file_signature": "575DAC7F994037D4740E5067277B63B90DDE0FCE",
      "upgrade_recipes": [],
      "game_types": [
        "PvpLobby",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoMysticForge",
        "NoSalvage",
        "NoSell",
        "SoulbindOnAcquire",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "consumable": {
        "type": "AppearanceChange"
      }
    }"#;

    #[test]
    fn get_utility_appearancechange() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UTILITY_APPEARANCECHANGE).unwrap();
        assert_eq!(Item::get_id(&client, 36284).unwrap(), item)
    }

    const JSON_CONTAINER: &str = r#"
    {
      "item_id": "36520",
      "name": "Bag of Coins",
      "description": "Double-click to open.",
      "type": "Container",
      "level": "0",
      "rarity": "Fine",
      "vendor_value": "0",
      "icon_file_id": "62857",
      "icon_file_signature": "3BF274D1A323C002CDCFE80724912CDB7C78ABEE",
      "upgrade_recipes": [],
      "game_types": [
        "Pvp",
        "PvpLobby",
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSell",
        "SoulbindOnAcquire",
        "SoulBindOnUse"
      ],
      "restrictions": [],
      "container": {
        "type": "Default"
      }
    }"#;

    #[test]
    fn get_container() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CONTAINER).unwrap();
        assert_eq!(Item::get_id(&client, 36520).unwrap(), item)
    }

    const JSON_CRAFTING_MATERIAL: &str = r#"
    {
      "item_id": "13000",
      "name": "Bronze Trident Head",
      "description": "Used to craft Tridents.",
      "type": "CraftingMaterial",
      "level": "0",
      "rarity": "Basic",
      "vendor_value": "2",
      "icon_file_id": "63494",
      "icon_file_signature": "9498D20ACAD065EA9848264A42CDCDF4271D0B62",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [],
      "restrictions": []
    }"#;

    #[test]
    fn get_crafting_material() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_CRAFTING_MATERIAL).unwrap();
        assert_eq!(Item::get_id(&client, 13000).unwrap(), item)
    }

    const JSON_GATHERING: &str = r#"
    {
      "item_id": "78806",
      "name": "Unbreakable Logging Axe",
      "description": "Extremely efficient at gathering resources despite its unremarkable appearance, this Black Lion tool will never break or require replacing.",
      "type": "Gathering",
      "level": "0",
      "rarity": "Rare",
      "vendor_value": "0",
      "icon_file_id": "1459294",
      "icon_file_signature": "E80A16DD259FD93E0E0F5B2AE09C60F60E07F8C4",
      "default_skin": "2388",
      "upgrade_recipes": [],
      "game_types": [
        "PvpLobby",
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "HideSuffix",
        "AccountBound",
        "NoSalvage",
        "NoSell",
        "DeleteWarning",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "gathering": {
        "type": "Logging"
      }
    }"#;

    #[test]
    fn get_gathering() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_GATHERING).unwrap();
        assert_eq!(Item::get_id(&client, 78806).unwrap(), item)
    }

    const JSON_GIZMO: &str = r#"
    {
      "item_id": "22335",
      "name": "Commander's Compendium",
      "description": "Double-click to use. Unlock the ability to become a Commander.",
      "type": "Gizmo",
      "level": "11",
      "rarity": "Fine",
      "vendor_value": "125000",
      "icon_file_id": "66753",
      "icon_file_signature": "1F64A2799FC8666FFC66FE56DAA75C02E7E9BA6F",
      "upgrade_recipes": [],
      "game_types": [
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "AccountBound",
        "NoSell",
        "AccountBindOnUse"
      ],
      "restrictions": [],
      "gizmo": {
        "type": "Default"
      }
    }"#;

    #[test]
    fn get_gizmo() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_GIZMO).unwrap();
        assert_eq!(Item::get_id(&client, 22335).unwrap(), item)
    }

    const JSON_MINI_PET: &str = r#"
    {
      "item_id": "20211",
      "name": "Mini Black Moa",
      "description": "Double-click to summon this mini to follow you around. Only one mini may be in use at a time.",
      "type": "MiniPet",
      "level": "0",
      "rarity": "Exotic",
      "vendor_value": "100",
      "icon_file_id": "220516",
      "icon_file_signature": "F0350D36C73DCE3EA29A0451A4403BC3A8A172ED",
      "upgrade_recipes": [],
      "game_types": [
        "PvpLobby",
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSell",
        "DeleteWarning"
      ],
      "restrictions": [],
      "minipet": {
        "minipet_id": 102
      }
    }"#;

    #[test]
    fn get_mini_pet() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_MINI_PET).unwrap();
        assert_eq!(Item::get_id(&client, 20211).unwrap(), item)
    }

    const JSON_TRINKET: &str = r#"
    {
      "item_id": "81908",
      "name": "Aurora",
      "type": "Trinket",
      "level": "80",
      "rarity": "Legendary",
      "vendor_value": "825",
      "icon_file_id": "1729332",
      "icon_file_signature": "2E5391DBF100D5A4413A07C3A13FC3513F29C394",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "HideSuffix",
        "AccountBound",
        "NoMysticForge",
        "NoSalvage",
        "NoSell",
        "NotUpgradeable",
        "Unique",
        "AccountBindOnUse",
        "DeleteWarning"
      ],
      "restrictions": [],
      "trinket": {
        "type": "Accessory",
        "infusion_slots": [
          {
            "flags": [
              "Infusion"
            ]
          }
        ],
        "suffix_item_id": "",
        "secondary_suffix_item_id": "",
        "stat_choices": [
          584,
          656,
          658,
          1119,
          657,
          1038,
          1097,
          659,
          690,
          583,
          585,
          1037,
          586,
          1035,
          588,
          1114,
          1128,
          1163,
          1066,
          1064,
          660,
          1430,
          1436,
          591,
          581,
          592,
          1145,
          1098,
          1134,
          1162,
          1115,
          1130,
          1139,
          1125,
          1220,
          1329,
          1345,
          1337,
          1486,
          1538
        ]
      }
    }"#;

    #[test]
    fn get_trinket() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_TRINKET).unwrap();
        assert_eq!(Item::get_id(&client, 81908).unwrap(), item)
    }

    const JSON_TROPHY: &str = r#"
    {
      "item_id": "43555",
      "name": "Aetherized Metal Scrap",
      "description": "Salvage Item",
      "type": "Trophy",
      "level": "0",
      "rarity": "Rare",
      "vendor_value": "105",
      "icon_file_id": "598598",
      "icon_file_signature": "034978F6ED34E80168CF40179CAC090E5CC1C249",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [],
      "restrictions": []
    }"#;

    #[test]
    fn get_trophy() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_TROPHY).unwrap();
        assert_eq!(Item::get_id(&client, 43555).unwrap(), item)
    }

    const JSON_UPGRADE_COMPONENT: &str = r#"
    {
      "item_id": "49424",
      "name": "+1 Agony Infusion",
      "description": "Double-click to apply to an unused infusion slot. Used by artificers to craft more powerful agony infusions.",
      "type": "UpgradeComponent",
      "level": "0",
      "rarity": "Ascended",
      "vendor_value": "330",
      "icon_file_id": "511839",
      "icon_file_signature": "C605E2EF280B5E4CF9A249E80AB3053843C5EBE3",
      "upgrade_recipes": [],
      "game_types": [
        "Activity",
        "Wvw",
        "Dungeon",
        "Pve"
      ],
      "flags": [
        "NoSalvage",
        "NoSell"
      ],
      "restrictions": [],
      "upgrade_component": {
        "type": "Default",
        "flags": [
          "ShortBow",
          "HeavyArmor",
          "LightArmor",
          "Dagger",
          "MediumArmor",
          "Focus",
          "Greatsword",
          "Hammer",
          "Trinket",
          "Harpoon",
          "Mace",
          "Pistol",
          "Rifle",
          "Scepter",
          "Shield",
          "Speargun",
          "Axe",
          "Staff",
          "Sword",
          "Torch",
          "Trident",
          "Warhorn",
          "LongBow"
        ],
        "infusion_upgrade_flags": [
          "Infusion"
        ],
        "infix_upgrade": {
          "id": 764,
          "buff": {
            "skill_id": "22100",
            "description": "+1 Agony Resistance"
          },
          "attributes": [
            {
              "attribute": "AgonyResistance",
              "modifier": 1
            }
          ]
        },
        "suffix": ""
      }
    }"#;

    #[test]
    fn get_upgrade_component() {
        let client = Client::new();
        let item = serde_json::from_str::<Item>(JSON_UPGRADE_COMPONENT).unwrap();
        assert_eq!(Item::get_id(&client, 49424).unwrap(), item)
    }
}
