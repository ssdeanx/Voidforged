//! Meta-progression upgrade system — permanent upgrades that persist across runs.
//! Players spend Dark Essence and Gold to unlock permanent stat boosts,
//! starting weapons, and class-specific talents.

use bevy::prelude::*;
use ir_core::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Upgrade Definitions
// ============================================================================

/// A purchasable upgrade with tiered progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: UpgradeCategory,
    pub max_tier: u32,
    pub base_cost: u64,
    pub cost_multiplier: f32,
    pub icon_id: &'static str,
    /// Per-tier stat bonuses applied when unlocked.
    pub per_tier_stats: Vec<StatBonus>,
}

/// Category for grouping upgrades in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeCategory {
    Stats,       // Global stat boosts
    Weapons,     // Starting weapon unlocks
    Classes,     // Class-specific talents
    Utility,     // Gold gain, XP gain, pickup radius
}

/// A stat bonus applied at a given tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBonus {
    pub stat: StatType,
    pub value: f32,
}

/// All available meta-progression upgrades.
pub fn all_upgrade_defs() -> Vec<UpgradeDef> {
    vec![
        // ── Stats ─────────────────────────────────────────────────
        UpgradeDef {
            id: "max_hp_up",
            name: "Vitality",
            description: "Increase maximum health.",
            category: UpgradeCategory::Stats,
            max_tier: 5,
            base_cost: 100,
            cost_multiplier: 2.0,
            icon_id: "icon_hp_up",
            per_tier_stats: vec![StatBonus { stat: StatType::MaxHealth, value: 20.0 }],
        },
        UpgradeDef {
            id: "dmg_up",
            name: "Might",
            description: "Increase base damage bonus.",
            category: UpgradeCategory::Stats,
            max_tier: 5,
            base_cost: 100,
            cost_multiplier: 2.2,
            icon_id: "icon_dmg_up",
            per_tier_stats: vec![StatBonus { stat: StatType::DamageBonus, value: 3.0 }],
        },
        UpgradeDef {
            id: "armor_up",
            name: "Fortitude",
            description: "Increase armor rating.",
            category: UpgradeCategory::Stats,
            max_tier: 5,
            base_cost: 80,
            cost_multiplier: 2.0,
            icon_id: "icon_armor_up",
            per_tier_stats: vec![StatBonus { stat: StatType::Armor, value: 5.0 }],
        },
        UpgradeDef {
            id: "move_speed_up",
            name: "Agility",
            description: "Increase movement speed.",
            category: UpgradeCategory::Stats,
            max_tier: 3,
            base_cost: 150,
            cost_multiplier: 2.5,
            icon_id: "icon_speed_up",
            per_tier_stats: vec![StatBonus { stat: StatType::MoveSpeed, value: 0.5 }],
        },
        UpgradeDef {
            id: "crit_up",
            name: "Precision",
            description: "Increase critical hit chance.",
            category: UpgradeCategory::Stats,
            max_tier: 3,
            base_cost: 200,
            cost_multiplier: 2.5,
            icon_id: "icon_crit_up",
            per_tier_stats: vec![StatBonus { stat: StatType::CritChance, value: 0.02 }],
        },
        UpgradeDef {
            id: "lifesteal_up",
            name: "Leech",
            description: "Gain lifesteal.",
            category: UpgradeCategory::Stats,
            max_tier: 3,
            base_cost: 250,
            cost_multiplier: 2.8,
            icon_id: "icon_lifesteal",
            per_tier_stats: vec![StatBonus { stat: StatType::Lifesteal, value: 0.02 }],
        },

        // ── Starting Weapon Unlocks ──────────────────────────────
        UpgradeDef {
            id: "unlock_dagger",
            name: "Shadow's Kiss",
            description: "Unlock the Dagger as a starting weapon option (high attack speed).",
            category: UpgradeCategory::Weapons,
            max_tier: 1,
            base_cost: 300,
            cost_multiplier: 1.0,
            icon_id: "icon_dagger",
            per_tier_stats: vec![],
        },
        UpgradeDef {
            id: "unlock_bow",
            name: "Wind's Reach",
            description: "Unlock the Bow as a starting weapon option (ranged).",
            category: UpgradeCategory::Weapons,
            max_tier: 1,
            base_cost: 350,
            cost_multiplier: 1.0,
            icon_id: "icon_bow",
            per_tier_stats: vec![],
        },
        UpgradeDef {
            id: "unlock_staff",
            name: "Arcane Core",
            description: "Unlock the Staff as a starting weapon option (high damage, slow).",
            category: UpgradeCategory::Weapons,
            max_tier: 1,
            base_cost: 400,
            cost_multiplier: 1.0,
            icon_id: "icon_staff",
            per_tier_stats: vec![],
        },

        // ── Utility ──────────────────────────────────────────────
        UpgradeDef {
            id: "xp_boost",
            name: "Wisdom",
            description: "+15% XP gained from all sources.",
            category: UpgradeCategory::Utility,
            max_tier: 3,
            base_cost: 120,
            cost_multiplier: 2.0,
            icon_id: "icon_xp",
            per_tier_stats: vec![], // Applied multiplicatively in XP system
        },
        UpgradeDef {
            id: "gold_boost",
            name: "Greed",
            description: "+20% Gold found.",
            category: UpgradeCategory::Utility,
            max_tier: 3,
            base_cost: 100,
            cost_multiplier: 2.0,
            icon_id: "icon_gold",
            per_tier_stats: vec![],
        },
        UpgradeDef {
            id: "pickup_radius_up",
            name: "Attraction",
            description: "Increase pickup magnet radius by 1.5.",
            category: UpgradeCategory::Utility,
            max_tier: 3,
            base_cost: 80,
            cost_multiplier: 2.0,
            icon_id: "icon_magnet",
            per_tier_stats: vec![],
        },
    ]
}

// ============================================================================
// Cost Calculation
// ============================================================================

/// Calculate the Dark Essence cost for a given upgrade tier.
pub fn upgrade_cost(def: &UpgradeDef, current_tier: u32) -> u64 {
    if current_tier >= def.max_tier {
        return u64::MAX; // Already maxed
    }
    (def.base_cost as f32 * def.cost_multiplier.powi(current_tier as i32)) as u64
}

// ============================================================================
// Application — applies unlocked upgrades to player stats
// ============================================================================

/// Sums all stat bonuses from purchased upgrades.
pub fn accumulated_upgrade_stats(meta: &MetaProgression) -> Vec<StatBonus> {
    let defs = all_upgrade_defs();
    let mut accumulated: Vec<StatBonus> = Vec::new();

    for upgrade in &meta.upgrades {
        if let Some(def) = defs.iter().find(|d| d.id == upgrade.id) {
            let tier = upgrade.tier;
            // Sum all tiers up to current
            for _ in 0..tier {
                for bonus in &def.per_tier_stats {
                    if let Some(existing) = accumulated.iter_mut().find(|s: &&mut StatBonus| s.stat == bonus.stat) {
                        existing.value += bonus.value;
                    } else {
                        accumulated.push(bonus.clone());
                    }
                }
            }
        }
    }
    accumulated
}

/// Applies accumulated upgrade stats to a CombatStats struct.
pub fn apply_upgrade_stats(stats: &mut CombatStats, meta: &MetaProgression) {
    let bonuses = accumulated_upgrade_stats(meta);
    for bonus in bonuses {
        match bonus.stat {
            StatType::MaxHealth => stats.max_health_bonus += bonus.value,
            StatType::DamageBonus => stats.damage_bonus += bonus.value,
            StatType::Armor => stats.armor += bonus.value,
            StatType::MoveSpeed => stats.move_speed_bonus += bonus.value,
            StatType::CritChance => stats.crit_chance += bonus.value,
            StatType::Lifesteal => stats.lifesteal += bonus.value,
            StatType::PickupRadius => stats.pickup_radius += bonus.value,
            _ => {} // Other stats handled elsewhere
        }
    }
}

// ============================================================================
// Systems
// ============================================================================

/// Attempt to purchase an upgrade — called from UI or system.
pub fn purchase_upgrade(
    upgrade_id: &str,
    meta: &mut MetaProgression,
) -> Result<(), PurchaseError> {
    let defs = all_upgrade_defs();
    let def = defs.iter().find(|d| d.id == upgrade_id)
        .ok_or(PurchaseError::NotFound)?;

    let current_tier = meta.upgrades.iter()
        .find(|u| u.id == upgrade_id)
        .map(|u| u.tier)
        .unwrap_or(0);

    if current_tier >= def.max_tier {
        return Err(PurchaseError::MaxTier);
    }

    let cost = upgrade_cost(def, current_tier);
    if meta.dark_essence < cost {
        return Err(PurchaseError::InsufficientEssence(cost - meta.dark_essence));
    }

    meta.dark_essence -= cost;

    // Update or insert upgrade tier
    if let Some(existing) = meta.upgrades.iter_mut().find(|u| u.id == upgrade_id) {
        existing.tier += 1;
        existing.cost = cost;
    } else {
        meta.upgrades.push(UpgradeTier {
            id: upgrade_id.to_string(),
            tier: 1,
            cost,
        });
    }

    Ok(())
}

#[derive(Debug)]
pub enum PurchaseError {
    NotFound,
    MaxTier,
    InsufficientEssence(u64),
}

impl std::fmt::Display for PurchaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Upgrade not found"),
            Self::MaxTier => write!(f, "Upgrade already at max tier"),
            Self::InsufficientEssence(needed) => write!(f, "Need {} more Dark Essence", needed),
        }
    }
}

impl std::error::Error for PurchaseError {}

// ============================================================================
// Plugin
// ============================================================================

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_meta_upgrades_on_spawn
                .run_if(on_event::<LevelUpEvent>));
    }
}

/// Applies meta-progression upgrades to player stats when they level up or spawn.
fn apply_meta_upgrades_on_spawn(
    meta: Res<MetaProgression>,
    mut player_query: Query<&mut CombatStats, With<Player>>,
) {
    let Ok(mut stats) = player_query.get_single_mut() else { return };
    apply_upgrade_stats(&mut stats, &meta);
}
