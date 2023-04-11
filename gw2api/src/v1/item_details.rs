use serde::Deserialize;

use gw2api_core::attributes::Attribute;
use gw2api_core::utils::Rarity;

use gw2api_derive::ParamEndpoint;

/// This endpoint is quite silly, since all numerical values are encoded as strings, meaning when
/// a number is the expected result it will be a string instead, with the exception of
/// infix_upgrade for some reason which is only used by backpieces afaik ¯\_(ツ)_/¯.

/// Struct containing detailed localized information about a requested item.
#[derive(ParamEndpoint, Debug, Deserialize, PartialEq)]
#[endpoint = "/v1/item_details?item_id={}"]
pub struct Item {
    /// id of the item.
    #[serde(rename = "item_id")]
    pub id: String,
    /// Name of the item.
    pub name: String,
    /// Description of the item.
    pub description: Option<String>,
    /// Type of the item.
    #[serde(rename = "type")]
    pub item_type: ItemType,
    /// Required level for the item.
    pub level: String,
    /// Rarity of the item.
    pub rarity: Rarity,
    /// Value in coins when selling to a vendor.
    pub vendor_value: String,
    /// File id to be used with the render service.
    pub icon_file_id: String,
    /// File signature to be used with the render service.
    pub icon_file_signature: String,
    /// Skin id to be used with the skin_details endpoint to get more information. Only present
    /// for item types: Armor, Back and Weapon.
    pub default_skin: Option<String>,
    // TODO: Find an example.
    /// List of upgrade recipes, whatever they are.
    pub upgrade_recipes: Vec<String>,
    /// Game types where the item is usable.
    pub game_types: Vec<GameType>,
    /// Additional item flags.
    pub flags: Vec<ItemFlag>,
    /// Item usage restrictions based on race or profession.
    pub restrictions: Vec<Restrictions>,
    /// If the item is a piece of armor, this will be Some().
    pub armor: Option<Armor>,
    /// If the item is a piece of back, this will be Some().
    pub back: Option<Back>,
    /// If the item is a bag, this will be Some().
    pub bag: Option<Bag>,
    /// If the item is a consumable, this will be Some().
    pub consumable: Option<Consumable>,
    /// If the item is a container, this will be Some().
    pub container: Option<Container>,
    /// If the item is a crafting material, this will be Some(), containing an empty object.
    pub crafting_material: Option<CraftingMaterial>,
    /// If the item is a gathering, this will be Some().
    pub gathering: Option<Gathering>,
    /// If the item is a gizmo, this will be Some().
    pub gizmo: Option<Gizmo>,
    /// If the item is a mini pet, this will be Some(), containing an empty object.
    pub mini_pet: Option<MiniPet>,
    /// If the item is a tool, this will be Some().
    pub tool: Option<Tool>,
    /// If the item is a trinket, this will be Some().
    pub trinket: Option<Trinket>,
    /// If the item is a trophy, this will be Some(), containing an empty object.
    pub trophy: Option<Trophy>,
    /// If the item is a upgrade component, this will be Some().
    pub upgrade_component: Option<UpgradeComponent>,
    /// If the item is a weapon, this will be Some().
    pub weapon: Option<Weapon>,
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
    pub armor_type: ArmorType,
    /// The armor's weight class.
    pub weight_class: WeightClass,
    /// Defense value.
    pub defense: String,
    /// Number and type of infusion slots.
    pub infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    pub infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    pub suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    pub secondary_suffix_item_id: Option<String>,
    /// List of ids of stat choices that can be made with the item.
    #[serde(default)]
    pub stat_choices: Vec<u32>,
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
    pub id: Option<String>,
    /// Types of infusions allowed in this slot.
    pub flags: Vec<InfusionType>,
}

/// Struct containing information about an upgrade for a bonus given by a piece armor.
#[derive(Debug, Deserialize, PartialEq)]
pub struct InfixUpgrade {
    /// id of the infix upgrade.
    pub id: u32,
    /// Buff applied by the item. Seemingly only exists a Boon Duration buff.
    pub buff: Option<Buff>,
    /// Map of stat bonuses given by the item.
    pub attributes: Vec<Stat>,
}

/// Struct containing information about a stat on an item.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Stat {
    /// Attribute type.
    pub attribute: Attribute,
    /// How much the attribute is modified by.
    pub modifier: u32,
}

/// Struct containing information about a buff applied by an item. The buff seems to only be bonus
/// Boon Duration.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Buff {
    /// id of the skill applied by the item. Only known values are: "16517" and "10521".
    pub skill_id: String,
    /// Description of the effect of the skill. Only known description is "+1% Boon Duration".
    pub description: String,
}

/// Struct containing information about a back piece.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Back {
    /// List of infusion slot types.
    /// Only used by Back, the only known value is 49428 (+5 Agony_Infusion).
    pub infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    pub infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    pub suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    pub secondary_suffix_item_id: Option<String>,
    /// List of ids of stat choices that can be made with the item.
    #[serde(default)]
    pub stat_choices: Vec<u32>,
}

/// Struct containing information about a bag.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Bag {
    /// Whether or not the bag is a special one.
    pub no_sell_or_sort: Option<String>,
    /// Size of the bag.
    pub size: Option<String>,
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
    pub consumable_type: ConsumableType,
    /// Effect duration in milliseconds (may exist if consumable type is "Generic", "Food" or
    /// "Utility").
    pub duration_ms: Option<String>,
    /// Description of the effect (may exist if consumable type is "Generic", "Food" or
    /// "Utility").
    pub description: Option<String>,
    /// The type of unlock (if consumable type is "Unlock").
    pub unlock_type: Option<UnlockType>,
    /// id of the recipe unlocked by the consumable (if unlock type is "CraftingRecipe").
    pub recipe_id: Option<String>,
    /// id of the dye unlocked by the consumable (if unlock type is "Dye").
    pub color_id: Option<String>,
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
    pub container_type: ContainerType,
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
    pub gathering_type: GatheringType,
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
    pub gizmo_type: GizmoType,
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
    pub tool_type: ToolType,
    /// Number of charges of the tool.
    pub charges: i32,
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
    pub trinket_type: TrinketType,
    /// List of infusion slot types.
    pub infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    pub infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    pub suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    pub secondary_suffix_item_id: Option<String>,
    /// List of ids of stat choices that can be made with the item.
    #[serde(default)]
    pub stat_choices: Vec<u32>,
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
    pub upgrade_type: UpgradeType,
    /// List of items which the upgrade component can be applied to.
    pub flags: Vec<UpgradeFlag>,
    /// List of infusion slots the upgrade component can be applied to.
    pub infusion_upgrade_flags: Vec<InfusionType>,
    /// List of rune bonuses, only exits for the upgrade type "Rune">
    pub bonuses: Option<UpgradeType>,
    /// Potential bonus given by the object. Optional property.
    pub infix_upgrade: Option<InfixUpgrade>,
    /// Suffix added to the item when it is applied.
    pub suffix: String,
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
    pub weapon_type: WeaponType,
    /// Damage type.
    pub damage_type: DamageType,
    /// Weapon's minimal power rating.
    pub min_power: String,
    /// Weapon's maximum power rating.
    pub max_power: String,
    /// The weapon's defense rating.
    pub defense: String,
    /// List of infusion slots the upgrade component can be applied to.
    pub infusion_slots: Vec<InfusionSlot>,
    /// Potential bonus given by the object. Optional property.
    pub infix_upgrade: Option<InfixUpgrade>,
    /// Item id of an already applied upgrade component. Can be empty.
    pub suffix_item_id: Option<String>,
    /// Potential secondary upgrade component. Always empty.
    pub secondary_suffix_item_id: Option<String>,
}

impl Armor {}

impl InfusionSlot {}

impl InfixUpgrade {}

impl Stat {}

impl Buff {}

impl Back {}

impl Bag {}

impl Consumable {}

impl Container {}

impl CraftingMaterial {}

impl Gathering {}

impl Gizmo {}

impl MiniPet {}

impl Tool {}

impl Trinket {}

impl Trophy {}

impl UpgradeComponent {}

impl Weapon {}
