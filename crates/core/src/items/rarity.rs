//! Item rarity — quality tiers with color coding and stat multipliers.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Quality tiers for items, from common to legendary.
///
/// Rarity affects the item's stat multiplier (higher rarity = stronger stats),
/// UI color coding, vendor price, and loot drop probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    /// Grey — baseline quality, no stat multiplier.
    Common,
    /// Green — slightly better, 1.3× stat multiplier.
    Uncommon,
    /// Blue — good quality, 1.6× stat multiplier.
    Rare,
    /// Purple — excellent quality, 2.0× stat multiplier.
    Epic,
    /// Orange — best quality, 2.5× stat multiplier.
    Legendary,
}

impl ItemRarity {
    /// Display colour for UI tooltips and nameplates.
    pub fn color(&self) -> Color {
        match self {
            Self::Common => Color::srgb(0.7, 0.7, 0.7),
            Self::Uncommon => Color::srgb(0.3, 0.8, 0.3),
            Self::Rare => Color::srgb(0.3, 0.5, 1.0),
            Self::Epic => Color::srgb(0.7, 0.3, 0.9),
            Self::Legendary => Color::srgb(1.0, 0.6, 0.0),
        }
    }

    /// Hex colour string for UI labels (e.g. `"#B3B3B3"`).
    pub fn color_hex(&self) -> &'static str {
        match self {
            Self::Common => "#B3B3B3",
            Self::Uncommon => "#4DCC4D",
            Self::Rare => "#4D7FFF",
            Self::Epic => "#B34DE5",
            Self::Legendary => "#FF9900",
        }
    }

    /// Stat multiplier applied to base item stats based on rarity tier.
    ///
    /// Common: 1.0, Uncommon: 1.3, Rare: 1.6, Epic: 2.0, Legendary: 2.5.
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.3,
            Self::Rare => 1.6,
            Self::Epic => 2.0,
            Self::Legendary => 2.5,
        }
    }

    /// Human-readable label for UI tooltips.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
        }
    }
}
