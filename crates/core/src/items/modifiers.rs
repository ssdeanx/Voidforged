//! Stat modifiers — typed stat changes attached to items and buffs.

use serde::{Deserialize, Serialize};

/// Which stat a modifier affects.
///
/// Used by items, buffs, and abilities to describe what they change.
/// Percentage-based stats (CritChance, DodgeChance, Lifesteal, ArmorPenetration)
/// are stored as 0.0–1.0 fractions; flat stats are stored as absolute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatType {
    /// Flat damage bonus added to attacks.
    DamageBonus,
    /// Attack speed multiplier (percentage, 0.0–1.0+).
    AttackSpeedBonus,
    /// Flat damage reduction from incoming hits.
    Armor,
    /// Bonus maximum health (flat).
    MaxHealth,
    /// Movement speed multiplier (percentage).
    MoveSpeed,
    /// Critical hit chance (fraction 0.0–1.0).
    CritChance,
    /// Critical hit damage multiplier.
    CritMultiplier,
    /// Chance to completely avoid incoming damage (fraction 0.0–1.0).
    DodgeChance,
    /// Percentage of damage dealt returned as healing (fraction 0.0–1.0).
    Lifesteal,
    /// Radius for auto-pickup of items and XP gems.
    PickupRadius,
    /// Mana regeneration rate.
    ManaRegen,
    /// Flat armour penetration — ignores this much target armour.
    ArmorPenetration,
}

impl StatType {
    /// Human-readable label for UI tooltips.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::DamageBonus => "Damage",
            Self::AttackSpeedBonus => "Attack Speed",
            Self::Armor => "Armor",
            Self::MaxHealth => "Max Health",
            Self::MoveSpeed => "Move Speed",
            Self::CritChance => "Crit Chance",
            Self::CritMultiplier => "Crit Damage",
            Self::DodgeChance => "Dodge",
            Self::Lifesteal => "Lifesteal",
            Self::PickupRadius => "Pickup Radius",
            Self::ManaRegen => "Mana Regen",
            Self::ArmorPenetration => "Armor Pen",
        }
    }

    /// Formats a stat value for display.
    ///
    /// Percentage-based stats are multiplied by 100 and suffixed with `%`;
    /// flat stats are shown as-is with a `+` prefix.
    pub fn format_value(&self, value: f32) -> String {
        match self {
            Self::CritChance | Self::DodgeChance | Self::Lifesteal | Self::ArmorPenetration => {
                format!("+{:.0}%", value * 100.0)
            }
            _ => format!("+{:.0}", value),
        }
    }
}

/// A single stat modification with a flat value (not percentage-based).
///
/// Used in item definitions and runtime buffs to describe stat changes.
/// The `stat` field identifies which stat is modified; `value` is the
/// flat amount added (positive for buffs, negative for debuffs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatMod {
    /// Which stat this modifier affects.
    pub stat: StatType,
    /// Flat value of the modification.
    pub value: f32,
}
