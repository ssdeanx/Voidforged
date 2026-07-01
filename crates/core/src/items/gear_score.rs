//! Gear score and item level system — determines item quality from stat weights.
//!
//! An item's ilvl = base_level + rarity_bonus + stat_weighted_total.
//! Used for loot generation, comparison tooltips, and content scaling.

use crate::items::*;

// ============================================================================
// Stat Weights — determines how much each stat contributes to item level
// ============================================================================

/// Returns the weight of a stat type toward item level calculation.
///
/// Higher weight means each point of that stat contributes more to the
/// item's final level. Rarer stats (CritChance, Lifesteal) have higher
/// weights than common ones (MaxHealth, PickupRadius).
pub fn stat_weight(stat: StatType) -> f32 {
    match stat {
        StatType::DamageBonus => 1.0,
        StatType::AttackSpeedBonus => 0.8,
        StatType::Armor => 0.6,
        StatType::MaxHealth => 0.4,
        StatType::MoveSpeed => 0.5,
        StatType::CritChance => 2.0,       // rarer stat
        StatType::CritMultiplier => 1.5,
        StatType::DodgeChance => 2.0,
        StatType::Lifesteal => 2.5,         // very rare
        StatType::PickupRadius => 0.3,
        StatType::ManaRegen => 0.5,
        StatType::ArmorPenetration => 1.8,
    }
}

// ============================================================================
// Rarity Budget — stat budget multiplier per rarity tier
// ============================================================================

/// Returns the stat budget multiplier for a given rarity tier.
///
/// Higher rarities allow more total stats at the same item level,
/// making rare items strictly stronger than common ones.
pub fn rarity_budget(rarity: ItemRarity) -> f32 {
    match rarity {
        ItemRarity::Common => 1.0,
        ItemRarity::Uncommon => 1.5,
        ItemRarity::Rare => 2.2,
        ItemRarity::Epic => 3.0,
        ItemRarity::Legendary => 4.0,
    }
}

/// Returns the base item level for a given equipment slot.
///
/// Weapons and chest pieces have higher base levels than accessories.
pub fn slot_base_ilvl(slot: EquipSlot) -> u32 {
    match slot {
        EquipSlot::MainHand => 5,
        EquipSlot::OffHand => 3,
        EquipSlot::Helmet => 4,
        EquipSlot::Chest => 5,
        EquipSlot::Boots => 3,
        EquipSlot::Ring => 2,
        EquipSlot::Amulet => 2,
        EquipSlot::Trinket => 2,
    }
}

// ============================================================================
// Item Level Calculation
// ============================================================================

/// Calculates the item level (ilvl) of an item definition.
///
/// Formula: `slot_base + floor(rarity_bonus + stat_weighted_total / 5) + required_level / 2`
pub fn calculate_item_level(def: &ItemDef) -> u32 {
    let base = def.slot.map_or(1, slot_base_ilvl);
    let rarity_mult = rarity_budget(def.rarity);
    let stat_total: f32 = def.base_stats.iter()
        .map(|m| m.value.abs() * stat_weight(m.stat))
        .sum();
    let from_stats = (stat_total * rarity_mult / 5.0).round();
    base + from_stats as u32 + def.required_level / 2
}

/// Calculates the gear score for an item definition.
///
/// Gear score = `calculate_item_level(def) * 2`.
/// Used as a quick power-level comparison in UI tooltips.
pub fn gear_score(def: &ItemDef) -> u32 {
    calculate_item_level(def) * 2
}

/// Returns the item level bonus granted by a given rarity tier.
///
/// Common: 0, Uncommon: 3, Rare: 7, Epic: 12, Legendary: 20.
pub fn rarity_ilvl_bonus(rarity: ItemRarity) -> u32 {
    match rarity {
        ItemRarity::Common => 0,
        ItemRarity::Uncommon => 3,
        ItemRarity::Rare => 7,
        ItemRarity::Epic => 12,
        ItemRarity::Legendary => 20,
    }
}

// ============================================================================
// Loot Generation — creates item drops at a given level range
// ============================================================================

/// Generates a loot table appropriate for enemies at a given power level.
///
/// Returns a list of item definition IDs that can drop from enemies
/// whose level falls in the given range. Higher level enemies
/// have access to better loot tables.
pub fn loot_table_for_level(level: u32) -> Vec<&'static str> {
    match level {
        0..=5 => vec!["iron_sword", "iron_dagger", "short_bow", "leather_helm", "leather_chest", "leather_boots", "copper_ring", "health_potion"],
        6..=10 => vec!["steel_sword", "steel_dagger", "long_bow", "iron_helm", "chainmail", "iron_boots", "silver_ring", "health_potion"],
        11..=20 => vec!["runed_sword", "apprentice_staff", "plate_chest", "silver_ring", "health_potion"],
        _ => vec!["archmage_staff", "runed_sword", "plate_chest", "silver_ring", "health_potion"],
    }
}

// ============================================================================
// Stat Comparison — for tooltips and gear comparison
// ============================================================================

/// Represents the difference in one stat between two items for comparison UI.
pub struct StatDiff {
    /// The stat being compared.
    pub stat: StatType,
    /// Value of the stat on the currently equipped item.
    pub old_value: f32,
    /// Value of the stat on the prospective new item.
    pub new_value: f32,
}

/// Compares two sets of stat modifiers and returns the net differences.
///
/// Only stats that differ by more than 0.01 are included in the result.
/// Used by UI tooltips to show what changes when swapping equipment.
pub fn compare_items(current: &[StatMod], upgraded: &[StatMod]) -> Vec<StatDiff> {
    let mut diffs = Vec::new();
    // Collect all unique stat types
    let mut all_stats: Vec<StatType> = Vec::new();
    for m in current.iter().chain(upgraded.iter()) {
        if !all_stats.contains(&m.stat) {
            all_stats.push(m.stat);
        }
    }
    for stat in all_stats {
        let old_val = current.iter().find(|m| m.stat == stat).map_or(0.0, |m| m.value);
        let new_val = upgraded.iter().find(|m| m.stat == stat).map_or(0.0, |m| m.value);
        if (old_val - new_val).abs() > 0.01 {
            diffs.push(StatDiff { stat, old_value: old_val, new_value: new_val });
        }
    }
    diffs
}
