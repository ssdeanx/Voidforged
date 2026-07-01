//! Stat modifiers — typed stat changes attached to items and buffs.

use serde::{Deserialize, Serialize};

/// Which stat a modifier affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatType {
    DamageBonus,
    AttackSpeedBonus,
    Armor,
    MaxHealth,
    MoveSpeed,
    CritChance,
    CritMultiplier,
    DodgeChance,
    Lifesteal,
    PickupRadius,
    ManaRegen,
    ArmorPenetration,
}

impl StatType {
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

    pub fn format_value(&self, value: f32) -> String {
        match self {
            Self::CritChance | Self::DodgeChance | Self::Lifesteal | Self::ArmorPenetration => {
                format!("+{:.0}%", value * 100.0)
            }
            _ => format!("+{:.0}", value),
        }
    }
}

/// A single stat modification (flat value, not percentage).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatMod {
    pub stat: StatType,
    pub value: f32,
}
