//! Item definitions — template struct and all starter items.
//! Uses direct struct construction (no macros) for reliability and IDE support.

use crate::items::*;

/// Template definition for an item type. Immutable, stored in [`ItemDatabase`](crate::ItemDatabase).
///
/// Each `ItemDef` is a static blueprint — it contains the item's base stats,
/// display info, slot restrictions, and other metadata. To create a runtime
/// copy with mutable state (durability, stack count), instantiate an
/// [`ItemInstance`] using the `def_id`.
#[derive(Debug, Clone)]
pub struct ItemDef {
    /// Unique string identifier (e.g. `"iron_sword"`, `"health_potion"`).
    pub id: &'static str,
    /// Display name shown in UI tooltips and panels.
    pub name: &'static str,
    /// Flavour text describing the item's appearance or lore.
    pub description: &'static str,
    /// High-level category (Weapon, Armor, Consumable, etc.).
    pub category: ItemCategory,
    /// Equipment slot, if this item can be equipped.
    pub slot: Option<EquipSlot>,
    /// Quality tier that determines stat multipliers and UI colour.
    pub rarity: ItemRarity,
    /// Base stat modifications applied when this item is equipped.
    pub base_stats: Vec<StatMod>,
    /// Maximum stack count for consumables / materials (1 for equipment).
    pub max_stack: u16,
    /// Identifier for the UI icon sprite.
    pub icon_id: &'static str,
    /// Minimum character level required to equip or use this item.
    pub required_level: u32,
    /// Base vendor price in gold.
    pub vendor_price: u64,
}

/// Returns all starter item definitions.
///
/// Called once at startup by [`init_item_database`](crate::resources::init_item_database)
/// to populate the central [`ItemDatabase`](crate::resources::ItemDatabase).
pub fn starter_item_defs() -> Vec<ItemDef> {
    vec![
        // ── Weapons ─────────────────────────────────────────────────
        ItemDef {
            id: "iron_sword",
            name: "Iron Sword",
            description: "A sturdy iron blade.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 6.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 0.0 },
            ],
            max_stack: 1, icon_id: "icon_iron_sword", required_level: 1, vendor_price: 50,
        },
        ItemDef {
            id: "steel_sword",
            name: "Steel Sword",
            description: "A finely-honed steel blade.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Uncommon,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 10.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 5.0 },
            ],
            max_stack: 1, icon_id: "icon_steel_sword", required_level: 3, vendor_price: 150,
        },
        ItemDef {
            id: "runed_sword",
            name: "Runed Sword",
            description: "A blade inscribed with ancient runes.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Rare,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 16.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 8.0 },
            ],
            max_stack: 1, icon_id: "icon_runed_sword", required_level: 8, vendor_price: 500,
        },
        ItemDef {
            id: "iron_dagger",
            name: "Iron Dagger",
            description: "A quick iron blade.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 4.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 15.0 },
            ],
            max_stack: 1, icon_id: "icon_iron_dagger", required_level: 1, vendor_price: 40,
        },
        ItemDef {
            id: "short_bow",
            name: "Short Bow",
            description: "A basic hunting bow.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 8.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 0.0 },
            ],
            max_stack: 1, icon_id: "icon_short_bow", required_level: 1, vendor_price: 60,
        },
        ItemDef {
            id: "long_bow",
            name: "Long Bow",
            description: "A well-crafted longbow.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Uncommon,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 14.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 5.0 },
            ],
            max_stack: 1, icon_id: "icon_long_bow", required_level: 4, vendor_price: 180,
        },
        ItemDef {
            id: "apprentice_staff",
            name: "Apprentice Staff",
            description: "A staff for budding mages.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 10.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 0.0 },
            ],
            max_stack: 1, icon_id: "icon_apprentice_staff", required_level: 1, vendor_price: 55,
        },
        ItemDef {
            id: "archmage_staff",
            name: "Archmage Staff",
            description: "A staff crackling with power.",
            category: ItemCategory::Weapon,
            slot: Some(EquipSlot::MainHand),
            rarity: ItemRarity::Rare,
            base_stats: vec![
                StatMod { stat: StatType::DamageBonus, value: 22.0 },
                StatMod { stat: StatType::AttackSpeedBonus, value: 10.0 },
            ],
            max_stack: 1, icon_id: "icon_archmage_staff", required_level: 10, vendor_price: 600,
        },

        // ── Armor ───────────────────────────────────────────────────
        ItemDef {
            id: "leather_helm", name: "Leather Helm", description: "A simple leather cap.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Helmet),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 3.0 },
                StatMod { stat: StatType::MaxHealth, value: 10.0 },
            ],
            max_stack: 1, icon_id: "icon_leather_helm", required_level: 1, vendor_price: 30,
        },
        ItemDef {
            id: "iron_helm", name: "Iron Helm", description: "A sturdy iron helmet.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Helmet),
            rarity: ItemRarity::Uncommon,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 8.0 },
                StatMod { stat: StatType::MaxHealth, value: 20.0 },
            ],
            max_stack: 1, icon_id: "icon_iron_helm", required_level: 3, vendor_price: 100,
        },
        ItemDef {
            id: "leather_chest", name: "Leather Tunic", description: "A padded leather tunic.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Chest),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 5.0 },
                StatMod { stat: StatType::MaxHealth, value: 20.0 },
            ],
            max_stack: 1, icon_id: "icon_leather_chest", required_level: 1, vendor_price: 40,
        },
        ItemDef {
            id: "chainmail", name: "Chainmail", description: "A suit of interlocking rings.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Chest),
            rarity: ItemRarity::Uncommon,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 14.0 },
                StatMod { stat: StatType::MaxHealth, value: 40.0 },
            ],
            max_stack: 1, icon_id: "icon_chainmail", required_level: 4, vendor_price: 150,
        },
        ItemDef {
            id: "plate_chest", name: "Plate Armor", description: "Heavy steel plate armor.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Chest),
            rarity: ItemRarity::Rare,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 24.0 },
                StatMod { stat: StatType::MaxHealth, value: 60.0 },
            ],
            max_stack: 1, icon_id: "icon_plate_chest", required_level: 8, vendor_price: 500,
        },
        ItemDef {
            id: "leather_boots", name: "Leather Boots", description: "Simple leather boots.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Boots),
            rarity: ItemRarity::Common,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 1.0 },
                StatMod { stat: StatType::MaxHealth, value: 5.0 },
            ],
            max_stack: 1, icon_id: "icon_leather_boots", required_level: 1, vendor_price: 20,
        },
        ItemDef {
            id: "iron_boots", name: "Iron Boots", description: "Reinforced iron boots.",
            category: ItemCategory::Armor, slot: Some(EquipSlot::Boots),
            rarity: ItemRarity::Uncommon,
            base_stats: vec![
                StatMod { stat: StatType::Armor, value: 4.0 },
                StatMod { stat: StatType::MaxHealth, value: 10.0 },
            ],
            max_stack: 1, icon_id: "icon_iron_boots", required_level: 3, vendor_price: 80,
        },

        // ── Accessories ─────────────────────────────────────────────
        ItemDef {
            id: "copper_ring", name: "Copper Ring",
            description: "A simple copper band.",
            category: ItemCategory::Accessory, slot: Some(EquipSlot::Ring),
            rarity: ItemRarity::Common,
            base_stats: vec![StatMod { stat: StatType::CritChance, value: 0.02 }],
            max_stack: 1, icon_id: "icon_copper_ring", required_level: 1, vendor_price: 25,
        },
        ItemDef {
            id: "silver_ring", name: "Silver Ring",
            description: "A polished silver band.",
            category: ItemCategory::Accessory, slot: Some(EquipSlot::Ring),
            rarity: ItemRarity::Uncommon,
            base_stats: vec![
                StatMod { stat: StatType::CritChance, value: 0.04 },
                StatMod { stat: StatType::DamageBonus, value: 3.0 },
            ],
            max_stack: 1, icon_id: "icon_silver_ring", required_level: 5, vendor_price: 120,
        },

        // ── Consumables ─────────────────────────────────────────────
        ItemDef {
            id: "health_potion", name: "Health Potion",
            description: "Restores 50 HP over 5 seconds.",
            category: ItemCategory::Consumable, slot: None,
            rarity: ItemRarity::Common,
            base_stats: vec![],
            max_stack: 20, icon_id: "icon_health_potion", required_level: 1, vendor_price: 15,
        },
    ]
}
